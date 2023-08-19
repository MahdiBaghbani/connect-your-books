use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

use crate::configs::Config;

pub async fn setup_database(configs: &Config) -> DatabaseConnection {
    // database connection options.
    let mut options: ConnectOptions = ConnectOptions::new(configs.url_database());
    options.sqlx_logging(false);

    // connect to database.
    let connection: DatabaseConnection = Database::connect(options).await.unwrap();

    // apply all pending migrations.
    Migrator::up(&connection, None).await.unwrap();

    connection
}
