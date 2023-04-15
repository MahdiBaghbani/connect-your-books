// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {}
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {}

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

pub fn view() -> Vec<Node<Msg>> {
    vec![
        header![
            C!["bg-white", "shadow"],
            div![
                C!["mx-auto", "max-w-7xl", "px-4", "py-6", "sm:px-6", "lg:px-8"],
                h1![
                    C!["text-3xl", "font-bold", "tracking-tight", "text-gray-900"],
                    "Dashboard",
                ],
            ],
        ],
        main![div![C![
            "mx-auto",
            "max-w-7xl",
            "py-6",
            "sm:px-6",
            "lg:px-8"
        ],],],
    ]
}
