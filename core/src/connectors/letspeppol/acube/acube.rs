use std::error::Error;

use reqwest::{Response, StatusCode};
use reqwest_middleware::ClientWithMiddleware;

use crate::connectors::letspeppol::acube::authentication::Authentication;
use crate::utility::reqwest_utils;

#[derive(Debug)]
pub struct Acube {
    authentication: Authentication,
    reqwest_client: ClientWithMiddleware,
}

impl Acube {
    pub fn create_acube() -> Acube {
        Acube {
            authentication: Authentication::create_authentication(),
            reqwest_client: reqwest_utils::create_reqwest_client(),
        }
    }
}
