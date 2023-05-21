use reqwest_middleware::ClientWithMiddleware;

use crate::connectors::letspeppol::acube::authentication::Authentication;
use crate::utility::reqwest_utils;

#[derive(Debug)]
pub struct Acube {
    authentication: Authentication,
    reqwest_client: ClientWithMiddleware,
}

impl Default for Acube {
    fn default() -> Self {
        Acube::new()
    }
}

impl Acube {
    pub fn new() -> Self {
        Acube {
            authentication: Authentication::new(),
            reqwest_client: reqwest_utils::create_reqwest_client(),
        }
    }
}
