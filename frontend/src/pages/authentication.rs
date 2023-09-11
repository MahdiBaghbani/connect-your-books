// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{*, prelude::*};

use crate::pages::components::authentication::sign_in;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model { base_url: url }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    base_url: Url,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();

impl<'a> Urls<'a> {
    pub fn base(self) -> Url {
        self.base_url()
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
pub enum Msg {}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![main![
        C!["bg-white", "dark:bg-gray-900"],
        sign_in::view_sign_in_section(&model.base_url),
    ]]
}
