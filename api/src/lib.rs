use salvo::conn::tcp::TcpAcceptor;
use salvo::prelude::*;

mod configs;
mod databases;
mod middlewares;
mod routers;
mod schemas;
mod users;

use configs::Config;
use database::sea_orm::DatabaseConnection;
use schemas::AppState;

#[tokio::main]
pub async fn run() {
    // load application configuration from .env file.
    let configs: Config = Config::new();

    // setup database.
    let db_connection: DatabaseConnection = databases::setup_database(&configs).await;

    // global app state.
    let app_state: AppState = AppState { db_connection };

    // setup all required middlewares.
    let middlewares: Router = middlewares::setup(&configs, app_state);

    // setup api router with all middlewares and static assets.
    let router: Router = routers::setup(&configs, middlewares);

    // start server with router.
    let acceptor: TcpAcceptor = TcpListener::new(configs.url_api()).bind().await;
    Server::new(acceptor).serve(router).await;
}
