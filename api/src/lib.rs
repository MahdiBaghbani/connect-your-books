use std::env;

use salvo::compression::Compression;
use salvo::conn::tcp::TcpAcceptor;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

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

    let logger: Logger = Logger::new();

    let compression: Compression = Compression::new()
        .disable_all()
        .enable_brotli(CompressionLevel::Minsize)
        .force_priority(true);

    let router: Router = Router::new()
        .hoop(logger)
        .hoop(compression)
        .path("<**path>")
        .get(
            StaticDir::new([frontend])
                .defaults("index.html")
                .listing(true),
        );

    let acceptor: TcpAcceptor = TcpListener::new(url).bind().await;
    Server::new(acceptor).serve(router).await;
}
