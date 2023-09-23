use seed::prelude::*;

use crate::models;
use crate::pages;

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleDarkMode,
    ChangeNavBarActiveItem(u8),
    ToggleNavBarHamburgerView,
    ToggleProfileMenu,
    HideProfileMenu,
    TopSecretFetched(Result<String, gloo_net::Error>),

    // ------ pages ------
    HomeMsg(pages::home::updates::Msg),
    DashboardMsg(pages::dashboard::Msg),
    TeamMsg(pages::team::Msg),
    ProjectsMsg(pages::projects::Msg),
    CalendarMsg(pages::calendar::Msg),
    ReportsMsg(pages::reports::Msg),
    AuthenticationMsg(pages::authentication::updates::Msg),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut models::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = pages::Pages::init(url, orders, model.ctx.user.as_ref());
        }
        Msg::ChangeNavBarActiveItem(id) => {
            model.navbar_active_item_id = id;
        }
        Msg::ToggleDarkMode => model.is_dark_mode = not(model.is_dark_mode),
        Msg::ToggleNavBarHamburgerView => {
            model.navbar_hamburger_menu_visible = not(model.navbar_hamburger_menu_visible)
        }
        Msg::ToggleProfileMenu => {
            model.profile_menu_visible = not(model.profile_menu_visible);
        }
        Msg::HideProfileMenu => {
            if model.profile_menu_visible {
                model.profile_menu_visible = false;
            } else {
                orders.skip();
            }
        }
        Msg::TopSecretFetched(Ok(secret_message)) => {
            model.secret_message = Some(secret_message);
        }

        // ------ pages ------
        Msg::HomeMsg(msg) => {
            if let pages::Pages::Home(model) = &mut model.page {
                pages::home::updates::update(msg, model, &mut orders.proxy(Msg::HomeMsg))
            }
        }
        Msg::DashboardMsg(msg) => {
            if let pages::Pages::Dashboard(model) = &mut model.page {
                pages::dashboard::update(msg, model, &mut orders.proxy(Msg::DashboardMsg))
            }
        }
        Msg::TeamMsg(msg) => {
            if let pages::Pages::Team(model) = &mut model.page {
                pages::team::update(msg, model, &mut orders.proxy(Msg::TeamMsg))
            }
        }
        Msg::ProjectsMsg(msg) => {
            if let pages::Pages::Projects(model) = &mut model.page {
                pages::projects::update(msg, model, &mut orders.proxy(Msg::ProjectsMsg))
            }
        }
        Msg::CalendarMsg(msg) => {
            if let pages::Pages::Calendar(model) = &mut model.page {
                pages::calendar::update(msg, model, &mut orders.proxy(Msg::CalendarMsg))
            }
        }
        Msg::ReportsMsg(msg) => {
            if let pages::Pages::Reports(model) = &mut model.page {
                pages::reports::update(msg, model, &mut orders.proxy(Msg::ReportsMsg))
            }
        }
        Msg::AuthenticationMsg(msg) => {
            if let pages::Pages::Authentication(model) = &mut model.page {
                pages::authentication::updates::update(msg, model, &mut orders.proxy(Msg::AuthenticationMsg))
            }
        }
        _ => {}
    }
}
