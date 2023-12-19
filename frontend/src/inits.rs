use gloo_storage::{LocalStorage, Storage};
use seed::prelude::*;

use crate::constants;
use crate::models;
use crate::pages;
use crate::updates;
use crate::urls;

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> models::Model {
    orders
        .subscribe(updates::Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| {
            updates::Msg::HideProfileMenu
        }));

    let user: Option<models::User> = LocalStorage::get(constants::STORAGE_KEY).ok();
    models::Model {
        ctx: models::Context { user: user.clone() },
        url: url.to_hash_base_url(),
        page: pages::Pages::init(url.clone(), orders, user.as_ref()),
        is_dark_mode: false,
        navbar: navbar(&url.to_hash_base_url()),
        navbar_active_item_id: active_navbar_item_id(url),
        navbar_hamburger_menu_visible: false,
        profile_menu: profile_menu(),
        profile_menu_visible: false,
        secret_message: None,
    }
}

fn navbar(base_url: &Url) -> Vec<models::NavigationItem> {
    vec![
        models::NavigationItem {
            id: 0,
            name: "Dashboard".to_string(),
            href: urls::Urls::new(base_url).dashboard().base().to_string(),
        },
        models::NavigationItem {
            id: 1,
            name: "Team".to_string(),
            href: urls::Urls::new(base_url).team().base().to_string(),
        },
        models::NavigationItem {
            id: 2,
            name: "Projects".to_string(),
            href: urls::Urls::new(base_url).projects().base().to_string(),
        },
        models::NavigationItem {
            id: 3,
            name: "Calendar".to_string(),
            href: urls::Urls::new(base_url).calendar().base().to_string(),
        },
        models::NavigationItem {
            id: 4,
            name: "Reports".to_string(),
            href: urls::Urls::new(base_url).reports().base().to_string(),
        },
    ]
}

fn active_navbar_item_id(mut url: Url) -> u8 {
    match url.remaining_hash_path_parts().as_slice() {
        [] => 10,
        [constants::DASHBOARD] => 0,
        [constants::TEAM] => 1,
        [constants::PROJECTS] => 2,
        [constants::CALENDAR] => 3,
        [constants::REPORTS] => 4,
        _ => 10,
    }
}

fn profile_menu() -> Vec<models::NavigationItem> {
    vec![
        models::NavigationItem {
            id: 0,
            name: "Your Profile".to_string(),
            href: "#".to_string(),
        },
        models::NavigationItem {
            id: 1,
            name: "Settings".to_string(),
            href: "#".to_string(),
        },
        models::NavigationItem {
            id: 2,
            name: "Sign out".to_string(),
            href: "#".to_string(),
        },
    ]
}
