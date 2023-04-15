use std::env;

use salvo::compression::{Compression, CompressionAlgo};
use salvo::listener::TcpListener;
use salvo::logging::Logger;
use salvo::serve_static::StaticDir;
use salvo::{Router, Server};

use core::connectors::letspeppol::acube::authentication;

#[tokio::main]
pub async fn run() {
    let host: String = env::var("CYB_HOST").expect("CYB_HOST is not set in .env file");
    let port: String = env::var("CYB_PORT").expect("CYB_PORT is not set in .env file");
    let server_url: String = format!("{host}:{port}");

    let _db_type: String = env::var("CYB_DB_TYPE").expect("CYB_DB_TYPE is not set in .env file");
    let _db_host: String = env::var("CYB_DB_HOST").expect("CYB_DB_HOST is not set in .env file");
    let _db_port: String = env::var("CYB_DB_PORT").expect("CYB_DB_PORT is not set in .env file");
    let _db_name: String = env::var("CYB_DB_NAME").expect("CYB_DB_NAME is not set in .env file");
    let _db_user: String = env::var("CYB_DB_USER").expect("CYB_DB_USER is not set in .env file");
    let _db_pass: String = env::var("CYB_DB_PASS").expect("CYB_DB_PASS is not set in .env file");

    let client: String = env::var("CYB_CLIENT").expect("CYB_CLIENT is not set in .env file");

    let router: Router = Router::new()
        .hoop(Logger)
        .hoop(Compression::new().with_algos(&[CompressionAlgo::Brotli]))
        .path("<**path>")
        .get(
            StaticDir::new([client])
                .with_defaults("index.html")
                .with_listing(true),
        );

    let s = authentication::Authentication::create_authentication();

    // s.get_token().await;

    let acceptor: TcpListener = TcpListener::bind(&server_url);
    tracing::info!("Listening on {server_url}");
    Server::new(acceptor).serve(router).await;
}
