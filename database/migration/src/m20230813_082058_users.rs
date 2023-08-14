use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::NameFirst).string().not_null())
                    .col(ColumnDef::new(User::NameLast).string().not_null())
                    .col(ColumnDef::new(User::NameUser).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Email).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-name-user")
                    .table(User::Table)
                    .col(User::NameUser)
                    .to_owned(),
            )
            .await?;

        // All good!
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx-name-user").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        // All good!
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    NameFirst,
    NameLast,
    NameUser,
    Password,
    Email,
}
