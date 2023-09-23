use seed::{*, prelude::*};

use crate::pages::authentication::components;
use crate::pages::authentication::models;
use crate::pages::authentication::updates;

pub fn view(model: &models::Model) -> Vec<Node<updates::Msg>> {
    vec![main![
        C!["bg-white", "dark:bg-gray-900"],
        components::sign_in::view_sign_in_section(&model.base_url),
    ]]
}
