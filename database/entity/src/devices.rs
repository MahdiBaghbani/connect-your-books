//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub mac_address: String,
    pub image: String,
    pub media_list: String,
    pub door_status: String,
    pub audio_volume: String,
    pub image_status: String,
    pub image_forced: String,
    pub temperature: String,
    pub voice_command: String,
    pub anti_theft_enable: String,
    pub anti_theft_status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
