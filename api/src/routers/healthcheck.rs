use salvo::http::{StatusCode, StatusError};
use salvo::prelude::*;
use salvo::Router;

pub fn setup() -> Router {
    Router::new().path("healthcheck").get(healthcheck)
}

#[endpoint(status_codes(200, 500))]
async fn healthcheck() -> Result<StatusCode, StatusError> {
    Ok(StatusCode::OK)
}
