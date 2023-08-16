use salvo::compression::Compression;
use salvo::conn::tcp::TcpAcceptor;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

use database::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

mod config;
mod schema;
mod users;

use config::Config;
use schema::JwtClaims;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

fn api_router() -> Router {
    Router::new().path("api").push(users::router())
}

#[tokio::main]
pub async fn run() {
    // load application configuration from .env file.
    let config: Config = Config::new();

    // create necessary middlewares.
    let logger: Logger = Logger::new();

    let auth_handler: JwtAuth<JwtClaims, _> =
        JwtAuth::new(ConstDecoder::from_secret(config.secret_bytes()))
            .finders(vec![Box::new(HeaderFinder::new())])
            .force_passed(true);

    let cache: CachingHeaders = CachingHeaders::new();

    let compression: Compression = Compression::new()
        .disable_all()
        .enable_brotli(CompressionLevel::Minsize)
        .force_priority(true);

    let cors_handler = Cors::new()
        .allow_origin(&config.url_api())
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    // create base router with all the middlewares.
    let router: Router = Router::new()
        .hoop(logger)
        .hoop(auth_handler)
        .hoop(cache)
        .hoop(compression)
        .hoop(cors_handler)
        .options(handler::empty());

    // push paths into the basic router.
    let router: Router = router.push(api_router());

    // create api documentation from routes.
    let doc: OpenApi = OpenApi::new("Connect Your Books API", "0.0.1").merge_router(&router);

    // push api documentation and swagger into router.
    let router: Router = router
        .push(doc.into_router("/api/doc/openapi.json"))
        .push(SwaggerUi::new("/api/doc/openapi.json").into_router("swagger-ui"));

    // push frontend static files to root of url.
    let router: Router = router.push(
        Router::new().path("<**rest_path>").get(
            StaticDir::new([config.frontend.clone()])
                .defaults("index.html")
                .listing(true),
        ),
    );

    // connect to database.
    let conn: DatabaseConnection = Database::connect(&config.url_database()).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let acceptor: TcpAcceptor = TcpListener::new(config.url_api()).bind().await;
    Server::new(acceptor).serve(router).await;
}
