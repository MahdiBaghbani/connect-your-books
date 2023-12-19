use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::pages;

// `Model` describes our app state.
pub struct Model {
    pub ctx: Context,
    pub url: Url,
    pub page: pages::Pages,
    pub is_dark_mode: bool,
    pub navbar: Vec<NavigationItem>,
    pub navbar_active_item_id: u8,
    pub navbar_hamburger_menu_visible: bool,
    pub profile_menu: Vec<NavigationItem>,
    pub profile_menu_visible: bool,
    pub secret_message: Option<String>,
}

pub struct Context {
    pub user: Option<User>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: Option<String>,
    pub token: String,
}

pub struct NavigationItem {
    pub id: u8,
    pub name: String,
    pub href: String,
}
