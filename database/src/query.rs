use sea_orm::*;

use ::entity::{devices, devices::Entity as Device, users, users::Entity as User};

pub struct Query;

impl Query {
    pub async fn is_new_mac_address(database_connection: &DbConn, mac_address: &str) -> bool {
        return if let Some(_) = Device::find()
            .select_only()
            .column(devices::Column::MacAddress)
            .filter(devices::Column::MacAddress.contains(mac_address))
            .one(database_connection)
            .await
            .ok()
        {
            true
        } else {
            false
        };
    }
    pub async fn find_device_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
    ) -> Result<Option<devices::Model>, DbErr> {
        Device::find()
            .filter(devices::Column::MacAddress.contains(mac_address))
            .one(database_connection)
            .await
    }
    pub async fn find_user_by_name_password(
        database_connection: &DbConn,
        name: &str,
        password: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        User::find()
            .filter(users::Column::Name.contains(name))
            .filter(users::Column::Password.contains(password))
            .one(database_connection)
            .await
    }
}
