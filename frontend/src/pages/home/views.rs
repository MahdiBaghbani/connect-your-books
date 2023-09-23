use seed::{*, prelude::*};

use crate::pages::home::components;
use crate::pages::home::models;
use crate::pages::home::updates;

pub fn view(model: &models::Model) -> Vec<Node<updates::Msg>> {
    vec![main![
        C!["bg-white", "dark:bg-gray-900"],
        components::hero::view_hero_section(),
        components::features::view_feature_list(),
        components::cta::view_call_to_action(),
        components::team::view_our_team(&model.team),
    ]]
}
