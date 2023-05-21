use std::error::Error;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine as _};
use reqwest::{Response, StatusCode};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::connectors::letspeppol::acube::constants::Constants;
use crate::utility::reqwest_utils;

#[derive(Debug)]
pub struct Authentication {
    constants: Constants,
    reqwest_client: ClientWithMiddleware,
}

impl Default for Authentication {
    fn default() -> Self {
        Authentication::new()
    }
}

impl Authentication {
    pub fn new() -> Self {
        Authentication {
            constants: Constants::new(),
            reqwest_client: reqwest_utils::create_reqwest_client(),
        }
    }

    // TODO: handle error return type.
    pub async fn get_token(&self) -> Result<String, Box<dyn Error>> {
        // read token from file, in case of any file errors, create new token.
        let token: String = match self.read_token_from_file() {
            Ok(token) => {
                // check the token read from file is still valid,
                // if the token is expired create new one.
                let token: Token = Token { token };
                if token.is_valid()? {
                    token.token
                } else {
                    self.generate_token().await?
                }
            }
            Err(_) => self.generate_token().await?,
        };
        Ok(token)
    }

    pub fn read_token_from_file(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(self.constants.token_file())?)
    }

    // TODO: handle error return type.
    // generate new token and write in to file.
    pub async fn generate_token(&self) -> Result<String, Box<dyn Error>> {
        // send user info to login url and acquire a token.
        let response: Response = self
            .reqwest_client
            .post(self.constants.login_url())
            .json(&self.constants.login_user())
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let token: Token = response.json::<Token>().await?;
            fs::write(self.constants.token_file(), token.token.clone())?;
            return Ok(token.token);
        }

        // TODO: handle error when response is not ok.
        Err(Box::try_from("Something went wrong!".to_string())?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    token: String,
}

impl Token {
    fn decode(&self) -> Result<TokenData, Box<dyn Error>> {
        // The token is composed by 3 parts, each part is delimited by a dot (.) char.
        // Exploding the token by . and doing a base64 decode of the second part you
        // will obtain some useful information.
        let parts: Vec<&str> = self.token.split('.').collect();
        let bytes: Vec<u8> = general_purpose::STANDARD.decode(parts[1])?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    fn is_valid(&self) -> Result<bool, Box<dyn Error>> {
        let token_data: TokenData = self.decode()?;
        let expiry_date: SystemTime =
            UNIX_EPOCH + Duration::from_secs(token_data.exp.parse::<u64>()?);
        Ok(SystemTime::now() < expiry_date)
    }
}

// token data json example:
// {
//      "iat":1675081666,
//      "exp":1675168066,
//      "roles": {
//          "it.api.acubeapi.com":["ROLE_WRITER"],
//          "peppol.api.acubeapi.com":["ROLE_WRITER"]
//      },
//      "username":"",
//      "uid":null,
//      "fid":null,
//      "pid":null
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenData {
    pub iat: String,
    pub exp: String,
    pub roles: Roles,
    pub username: String,
    pub uid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    #[serde(rename = "it.api.acubeapi.com")]
    pub it: Vec<String>,
    #[serde(rename = "peppol.api.acubeapi.com")]
    pub peppol: Vec<String>,
}
