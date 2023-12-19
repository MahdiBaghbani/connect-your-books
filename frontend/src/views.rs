use seed::{*, prelude::*};

use crate::components;
use crate::models;
use crate::pages;
use crate::updates;

pub fn view(model: &models::Model) -> Vec<Node<updates::Msg>> {
    vec![div![
        IF!(model.is_dark_mode => C!["dark"]),
        C!["flex", "flex-col", "min-h-full"],
        components::navbar::view_navbar(model, &model.url, model.ctx.user.as_ref(),),
        view_content(&model.page),
        components::footer::view_footer(),
    ]]
}

// ----- view_content ------
fn view_content(page: &pages::Pages) -> Vec<Node<updates::Msg>> {
    match page {
        pages::Pages::Home(model) => pages::home::views::view(model).map_msg(updates::Msg::HomeMsg),
        pages::Pages::Dashboard(_model) => {
            pages::dashboard::view().map_msg(updates::Msg::DashboardMsg)
        }
        pages::Pages::Team(_model) => pages::team::view().map_msg(updates::Msg::TeamMsg),
        pages::Pages::Projects(_model) => {
            pages::projects::view().map_msg(updates::Msg::ProjectsMsg)
        }
        pages::Pages::Calendar(_model) => {
            pages::calendar::view().map_msg(updates::Msg::CalendarMsg)
        }
        pages::Pages::Reports(_model) => pages::reports::view().map_msg(updates::Msg::ReportsMsg),
        pages::Pages::Authentication(model) => {
            pages::authentication::views::view(model).map_msg(updates::Msg::AuthenticationMsg)
        }
        pages::Pages::NotFound => pages::dashboard::view().map_msg(updates::Msg::DashboardMsg),
    }
}
