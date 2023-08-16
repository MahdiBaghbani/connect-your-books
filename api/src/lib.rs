use salvo::conn::tcp::TcpAcceptor;
use salvo::prelude::*;

use database::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

mod configs;
mod middlewares;
mod routers;
mod schemas;
mod users;

use configs::Config;
use schemas::AppState;

#[tokio::main]
pub async fn run() {
    // load application configuration from .env file.
    let config: Config = Config::new();

    // connect to database.
    let db_connection: DatabaseConnection =
        Database::connect(&config.url_database()).await.unwrap();
    // apply all pending migrations.
    Migrator::up(&db_connection, None).await.unwrap();

    // global app state.
    let app_state: AppState = AppState { db_connection };

    // setup all required middlewares.
    let middlewares: Router = middlewares::setup(&config, app_state);

    // setup api router with all middlewares and static assets.
    let router: Router = routers::setup(&config, middlewares);

    // start server with router.
    let acceptor: TcpAcceptor = TcpListener::new(config.url_api()).bind().await;
    Server::new(acceptor).serve(router).await;
}
