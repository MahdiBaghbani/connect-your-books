use salvo::compression::Compression;
use salvo::cors::{Cors, CorsHandler};
use salvo::http::Method;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder, QueryFinder};
use salvo::prelude::*;

use crate::schemas::{AppState, JwtClaims};

pub fn setup(app_state: &AppState) -> Router {
    // create necessary middlewares.
    let logger: Logger = Logger::new();

    let auth_handler: JwtAuth<JwtClaims, _> = JwtAuth::new(ConstDecoder::from_secret(
        app_state.configs.jwt_secret_bytes(),
    ))
        .finders(vec![
            Box::new(HeaderFinder::new()),
            Box::new(QueryFinder::new("jwt_token")),
        ])
    .force_passed(true);

    let cache: CachingHeaders = CachingHeaders::new();

    let compression: Compression = Compression::new()
        .disable_all()
        .enable_brotli(CompressionLevel::Minsize)
        .force_priority(true);

    let cors_handler: CorsHandler = Cors::new()
        .allow_origin(&app_state.configs.url_api())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    // create a router with all the middlewares.
    Router::new()
        .hoop(affix::inject(app_state.clone()))
        .hoop(logger)
        .hoop(auth_handler)
        .hoop(cache)
        .hoop(compression)
        .hoop(cors_handler)
        .options(handler::empty())
}
