use gloo_net::http::Request;
use seed::prelude::*;

use crate::constants;
use crate::models;
use crate::updates;

pub mod authentication;
pub mod calendar;
pub mod dashboard;
pub mod home;
pub mod projects;
pub mod reports;
pub mod team;

fn send_request_to_top_secret(token: String, orders: &mut impl Orders<updates::Msg>) {
    orders.perform_cmd(async move {
        updates::Msg::TopSecretFetched(
            async {
                let api_url: &str = constants::API_URL;
                Request::get(&format!("{api_url}/top_secret"))
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await?
                    .text()
                    .await
            }
                .await,
        )
    });
}

pub enum Pages {
    Home(home::models::Model),
    Dashboard(dashboard::Model),
    Team(team::Model),
    Projects(projects::Model),
    Calendar(calendar::Model),
    Reports(reports::Model),
    Authentication(authentication::models::Model),
    NotFound,
}

impl Pages {
    pub fn init(
        mut url: Url,
        orders: &mut impl Orders<updates::Msg>,
        user: Option<&models::User>,
    ) -> Self {
        match url.remaining_hash_path_parts().as_slice() {
            [] => {
                if let Some(user) = user {
                    send_request_to_top_secret(user.token.clone(), orders);
                };
                Self::Home(home::inits::init(url, &mut orders.proxy(updates::Msg::HomeMsg)))
            }
            [constants::DASHBOARD] => Self::Dashboard(dashboard::init(
                url,
                &mut orders.proxy(updates::Msg::DashboardMsg),
            )),
            [constants::TEAM] => {
                Self::Team(team::init(url, &mut orders.proxy(updates::Msg::TeamMsg)))
            }
            [constants::PROJECTS] => Self::Projects(projects::init(
                url,
                &mut orders.proxy(updates::Msg::ProjectsMsg),
            )),
            [constants::CALENDAR] => Self::Calendar(calendar::init(
                url,
                &mut orders.proxy(updates::Msg::CalendarMsg),
            )),
            [constants::REPORTS] => Self::Reports(reports::init(
                url,
                &mut orders.proxy(updates::Msg::ReportsMsg),
            )),
            [constants::AUTHENTICATION] => Self::Authentication(authentication::inits::init(
                url,
                &mut orders.proxy(updates::Msg::AuthenticationMsg),
            )),
            _ => Self::NotFound,
        }
    }
}
