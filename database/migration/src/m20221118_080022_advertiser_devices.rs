use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Devices::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Devices::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Devices::MacAddress).string().not_null())
                    .col(ColumnDef::new(Devices::Image).string().not_null())
                    .col(ColumnDef::new(Devices::MediaList).string().not_null())
                    .col(ColumnDef::new(Devices::DoorStatus).string().not_null())
                    .col(ColumnDef::new(Devices::AudioVolume).string().not_null())
                    .col(ColumnDef::new(Devices::ImageStatus).string().not_null())
                    .col(ColumnDef::new(Devices::ImageForced).string().not_null())
                    .col(ColumnDef::new(Devices::Temperature).string().not_null())
                    .col(ColumnDef::new(Devices::VoiceCommand).string().not_null())
                    .col(ColumnDef::new(Devices::AntiTheftEnable).string().not_null())
                    .col(ColumnDef::new(Devices::AntiTheftStatus).string().not_null())
                    .to_owned(),
            )
            .await
            .expect("table creation failed");

        // create index on mac address to speed up searches.
        manager
            .create_index(
                Index::create()
                    .name("idx-indexes-mac-address")
                    .table(Devices::Table)
                    .col(Devices::MacAddress)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Devices::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Devices {
    Table,
    Id,
    MacAddress,
    Image,
    MediaList,
    DoorStatus,
    AudioVolume,
    ImageStatus,
    ImageForced,
    Temperature,
    VoiceCommand,
    AntiTheftEnable,
    AntiTheftStatus,
}
