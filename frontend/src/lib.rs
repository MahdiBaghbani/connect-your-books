// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{*, prelude::*};

use crate::pages::components;
use crate::pages::components::footer::view_footer;

pub mod pages;

const DASHBOARD: &str = "dashboard";
const TEAM: &str = "team";
const PROJECTS: &str = "projects";
const CALENDAR: &str = "calendar";
const REPORTS: &str = "reports";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideProfileMenu));
    Model {
        ctx: Context {
            user: None,
            token: None,
        },

        base_url: url.to_hash_base_url(),
        page: Pages::init(url.clone(), orders),
        is_dark_mode: false,
        navbar: init_navbar(url.to_hash_base_url()),
        navbar_active_item_id: init_active_navbar_item_id(url),
        navbar_hamburger_menu_visible: false,
        profile_menu: init_profile_menu(),
        profile_menu_visible: false,
    }
}

fn init_navbar(base_url: Url) -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            id: 0,
            name: "Dashboard".to_string(),
            href: Urls::new(&base_url).dashboard().base().to_string(),
        },
        NavigationItem {
            id: 1,
            name: "Team".to_string(),
            href: Urls::new(&base_url).team().base().to_string(),
        },
        NavigationItem {
            id: 2,
            name: "Projects".to_string(),
            href: Urls::new(&base_url).projects().base().to_string(),
        },
        NavigationItem {
            id: 3,
            name: "Calendar".to_string(),
            href: Urls::new(&base_url).calendar().base().to_string(),
        },
        NavigationItem {
            id: 4,
            name: "Reports".to_string(),
            href: Urls::new(&base_url).reports().base().to_string(),
        },
    ]
}

fn init_active_navbar_item_id(mut url: Url) -> u8 {
    match url.remaining_hash_path_parts().as_slice() {
        [] => 10,
        [DASHBOARD] => 0,
        [TEAM] => 1,
        [PROJECTS] => 2,
        [CALENDAR] => 3,
        [REPORTS] => 4,
        _ => 10,
    }
}

fn init_profile_menu() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            id: 0,
            name: "Your Profile".to_string(),
            href: "#".to_string(),
        },
        NavigationItem {
            id: 1,
            name: "Settings".to_string(),
            href: "#".to_string(),
        },
        NavigationItem {
            id: 2,
            name: "Sign out".to_string(),
            href: "#".to_string(),
        },
    ]
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    ctx: Context,
    base_url: Url,
    page: Pages,
    is_dark_mode: bool,
    navbar: Vec<NavigationItem>,
    navbar_active_item_id: u8,
    navbar_hamburger_menu_visible: bool,
    profile_menu: Vec<NavigationItem>,
    profile_menu_visible: bool,
}

// ------ Context ------

pub struct Context {
    user: Option<User>,
    token: Option<String>,
}

pub struct User {
    username: String,
    email: String,
}

struct NavigationItem {
    id: u8,
    name: String,
    href: String,
}

// ------ Page ------

pub enum Pages {
    Home(pages::home::Model),
    Dashboard(pages::dashboard::Model),
    Team(pages::team::Model),
    Projects(pages::projects::Model),
    Calendar(pages::calendar::Model),
    Reports(pages::reports::Model),
    NotFound,
}

impl Pages {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            [] => Self::Home(pages::home::init(url, &mut orders.proxy(Msg::HomeMsg))),
            [DASHBOARD] => Self::Dashboard(pages::dashboard::init(
                url,
                &mut orders.proxy(Msg::DashboardMsg),
            )),
            [TEAM] => Self::Team(pages::team::init(url, &mut orders.proxy(Msg::TeamMsg))),
            [PROJECTS] => Self::Projects(pages::projects::init(
                url,
                &mut orders.proxy(Msg::ProjectsMsg),
            )),
            [CALENDAR] => Self::Calendar(pages::calendar::init(
                url,
                &mut orders.proxy(Msg::CalendarMsg),
            )),
            [REPORTS] => Self::Reports(pages::reports::init(
                url,
                &mut orders.proxy(Msg::ReportsMsg),
            )),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();

impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn dashboard(self) -> pages::dashboard::Urls<'a> {
        pages::dashboard::Urls::new(self.base_url().add_hash_path_part(DASHBOARD))
    }
    pub fn team(self) -> pages::team::Urls<'a> {
        pages::team::Urls::new(self.base_url().add_hash_path_part(TEAM))
    }
    pub fn projects(self) -> pages::projects::Urls<'a> {
        pages::projects::Urls::new(self.base_url().add_hash_path_part(PROJECTS))
    }
    pub fn calendar(self) -> pages::calendar::Urls<'a> {
        pages::calendar::Urls::new(self.base_url().add_hash_path_part(CALENDAR))
    }
    pub fn reports(self) -> pages::reports::Urls<'a> {
        pages::reports::Urls::new(self.base_url().add_hash_path_part(REPORTS))
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleDarkMode,
    ChangeNavBarActiveItem(u8),
    ToggleNavBarHamburgerView,
    ToggleProfileMenu,
    HideProfileMenu,

    // ------ pages ------
    HomeMsg(pages::home::Msg),
    DashboardMsg(pages::dashboard::Msg),
    TeamMsg(pages::team::Msg),
    ProjectsMsg(pages::projects::Msg),
    CalendarMsg(pages::calendar::Msg),
    ReportsMsg(pages::reports::Msg),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
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
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Pages::init(url, orders);
        }

        // ------ pages ------
        Msg::HomeMsg(msg) => {
            if let Pages::Home(model) = &mut model.page {
                pages::home::update(msg, model, &mut orders.proxy(Msg::HomeMsg))
            }
        }
        Msg::DashboardMsg(msg) => {
            if let Pages::Dashboard(model) = &mut model.page {
                pages::dashboard::update(msg, model, &mut orders.proxy(Msg::DashboardMsg))
            }
        }
        Msg::TeamMsg(msg) => {
            if let Pages::Team(model) = &mut model.page {
                pages::team::update(msg, model, &mut orders.proxy(Msg::TeamMsg))
            }
        }
        Msg::ProjectsMsg(msg) => {
            if let Pages::Projects(model) = &mut model.page {
                pages::projects::update(msg, model, &mut orders.proxy(Msg::ProjectsMsg))
            }
        }
        Msg::CalendarMsg(msg) => {
            if let Pages::Calendar(model) = &mut model.page {
                pages::calendar::update(msg, model, &mut orders.proxy(Msg::CalendarMsg))
            }
        }
        Msg::ReportsMsg(msg) => {
            if let Pages::Reports(model) = &mut model.page {
                pages::reports::update(msg, model, &mut orders.proxy(Msg::ReportsMsg))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![div![
        IF!(model.is_dark_mode => C!["dark"]),
        C!["min-h-full"],
        components::navbar::view_navbar(model, &model.base_url, model.ctx.user.as_ref(),),
        view_content(&model.page),
        view_footer(),
    ]]
}

// ----- view_content ------

fn view_content(page: &Pages) -> Vec<Node<Msg>> {
    match page {
        Pages::Home(model) => pages::home::view().map_msg(Msg::HomeMsg),
        Pages::Dashboard(model) => pages::dashboard::view().map_msg(Msg::DashboardMsg),
        Pages::Team(model) => pages::team::view().map_msg(Msg::TeamMsg),
        Pages::Projects(model) => pages::projects::view().map_msg(Msg::ProjectsMsg),
        Pages::Calendar(model) => pages::calendar::view().map_msg(Msg::CalendarMsg),
        Pages::Reports(model) => pages::reports::view().map_msg(Msg::ReportsMsg),
        Pages::NotFound => pages::dashboard::view().map_msg(Msg::DashboardMsg),
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
