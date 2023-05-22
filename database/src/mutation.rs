use sea_orm::*;
use uuid::Uuid;

use ::entity::devices;
use ::entity::users;

use crate::query::Query as QueryCore;

pub struct Mutation;

impl Mutation {
    pub async fn create_device(
        database_connection: &DbConn,
        mac_address: &str,
    ) -> Result<devices::Model, DbErr> {
        devices::ActiveModel {
            id: Set(Uuid::new_v4().to_owned()),
            mac_address: Set(mac_address.to_owned()),
            image: Set("no_image".to_owned()),
            media_list: Set("no_list".to_owned()),
            door_status: Set("closed".to_owned()),
            audio_volume: Set("100".to_owned()),
            image_status: Set("0".to_owned()),
            image_forced: Set("false".to_owned()),
            temperature: Set("25".to_owned()),
            voice_command: Set("no_command".to_owned()),
            anti_theft_enable: Set("false".to_owned()),
            anti_theft_status: Set("not_detected".to_owned()),
            ..Default::default()
        }
            .insert(database_connection)
            .await
    }

    pub async fn create_user(
        database_connection: &DbConn,
        name: &str,
        password: &str,
        level: &str,
    ) -> Result<users::Model, DbErr> {
        users::ActiveModel {
            id: Set(Uuid::new_v4().to_owned()),
            name: Set(name.to_owned()),
            password: Set(password.to_owned()),
            level: Set(level.to_owned()),
            ..Default::default()
        }
            .insert(database_connection)
            .await
    }

    pub async fn update_anti_theft_enable_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        enabled: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.anti_theft_enable = Set(enabled.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_anti_theft_status_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        status: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.anti_theft_status = Set(status.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_audio_volume_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        volume: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.audio_volume = Set(volume.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_door_status_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        status: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.door_status = Set(status.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_image_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        base64: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.image = Set(base64.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_image_status_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        status: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.image_status = Set(status.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_image_forced_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        take: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.image_forced = Set(take.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_media_list_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        list: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.media_list = Set(list.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_temperature_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        value: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.temperature = Set(value.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }

    pub async fn update_voice_command_by_mac_address(
        database_connection: &DbConn,
        mac_address: &str,
        command: &str,
    ) -> Option<Result<devices::Model, DbErr>> {
        let device: Option<devices::Model> =
            QueryCore::find_device_by_mac_address(database_connection, mac_address)
                .await
                .ok()?;

        return if let Some(device) = device {
            let mut device: devices::ActiveModel = device.into();

            device.voice_command = Set(command.to_owned());

            Some(device.update(database_connection).await)
        } else {
            None
        };
    }
}
