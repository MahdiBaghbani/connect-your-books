use std::env;

use salvo::compression::Compression;
use salvo::conn::tcp::TcpAcceptor;
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

mod users;

#[endpoint]
async fn hello(name: QueryParam<String, false>) -> String {
    format!("Hello, {}!", name.as_deref().unwrap_or("World"))
}

#[tokio::main]
pub async fn run() {
    let host: String = env::var("CYB_HOST").expect("CYB_HOST is not set in .env file");
    let port: String = env::var("CYB_PORT").expect("CYB_PORT is not set in .env file");
    let url: String = format!("{host}:{port}");

    let _db_type: String = env::var("CYB_DB_TYPE").expect("CYB_DB_TYPE is not set in .env file");
    let _db_host: String = env::var("CYB_DB_HOST").expect("CYB_DB_HOST is not set in .env file");
    let _db_port: String = env::var("CYB_DB_PORT").expect("CYB_DB_PORT is not set in .env file");
    let _db_name: String = env::var("CYB_DB_NAME").expect("CYB_DB_NAME is not set in .env file");
    let _db_user: String = env::var("CYB_DB_USER").expect("CYB_DB_USER is not set in .env file");
    let _db_pass: String = env::var("CYB_DB_PASS").expect("CYB_DB_PASS is not set in .env file");

    let frontend: String = env::var("CYB_FRONTEND").expect("CYB_FRONTEND is not set in .env file");

    // create necessary middlewares.
    let logger: Logger = Logger::new();

    let cache: CachingHeaders = CachingHeaders::new();

    let compression: Compression = Compression::new()
        .disable_all()
        .enable_brotli(CompressionLevel::Minsize)
        .force_priority(true);

    let cors_handler = Cors::new()
        .allow_origin(&url)
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .into_handler();

    // create base router with all the middlewares.
    let router: Router = Router::new()
        .hoop(logger)
        .hoop(cache)
        .hoop(compression)
        .hoop(cors_handler)
        .options(handler::empty());

    // push paths into the basic router.
    let router: Router = router.push(Router::new().path("hello").get(hello));

    // create api documentation from routes.
    let doc: OpenApi = OpenApi::new("Connect Your Books API", "0.0.1").merge_router(&router);

    // push api documentation and swagger into router.
    let router: Router = router
        .push(doc.into_router("/api/doc/openapi.json"))
        .push(SwaggerUi::new("/api/doc/openapi.json").into_router("swagger-ui"));

    // push frontend static files to root of url.
    let router: Router = router.push(
        Router::new().path("<**rest_path>").get(
            StaticDir::new([frontend])
                .defaults("index.html")
                .listing(true),
        ),
    );

    let acceptor: TcpAcceptor = TcpListener::new(url).bind().await;
    Server::new(acceptor).serve(router).await;
}
