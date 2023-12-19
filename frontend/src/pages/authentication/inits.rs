use seed::prelude::*;

use crate::pages::authentication::models;
use crate::pages::authentication::updates;

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> models::Model {
    models::Model { base_url: url }
}