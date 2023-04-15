pub use sea_orm_migration::prelude::*;

mod m20221118_080022_advertiser_devices;
mod m20221126_072008_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221118_080022_advertiser_devices::Migration),
            Box::new(m20221126_072008_users::Migration),
        ]
    }
}
