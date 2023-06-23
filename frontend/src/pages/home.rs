// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::components::svg;
use crate::pages::components::home::{cta, features, hero, team};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model { team: init_team() }
}

fn init_team() -> Vec<TeamMember> {
    vec![
        TeamMember {
            name: "Michiel de Jong".to_string(),
            role: "Code Communist".to_string(),
            description: "Michiel de Jong is an independent freedom hacker, travelling the world \
            while coding for the revolution from his laptop. "
                .to_string(),
            image_url: "/assets/images/vendor/michiel_dejong.avif".to_string(),
            links: Some(vec![
                TeamMemberLink {
                    logo: Some(svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/michielbdejong".to_string()),
                },
                TeamMemberLink {
                    logo: Some(svg::logo_github().to_string()),
                    url: Some("https://github.com/michielbdejong".to_string()),
                },
            ]),
        },
        TeamMember {
            name: "Mahdi Baghbani".to_string(),
            role: "Percussive Maintainer".to_string(),
            description: "Don’t tell your IT staff I said this… but there’s nothing like a good \
            whack to fix a malfunctioning machine."
                .to_string(),
            image_url: "/assets/images/vendor/mahdi_baghbani.avif".to_string(),
            links: Some(vec![
                TeamMemberLink {
                    logo: Some(svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/mahdibaghbani".to_string()),
                },
                TeamMemberLink {
                    logo: Some(svg::logo_github().to_string()),
                    url: Some("https://github.com/MahdiBaghbani".to_string()),
                },
            ]),
        },
        TeamMember {
            name: "Yashar PourMohammad".to_string(),
            role: "Code Guru".to_string(),
            description: "Yashar is keen to understand the inner workings of mind, code and \
            society. He thinks too much and codes too little.".to_string(),
            image_url: "/assets/images/vendor/yashar_pourmohammad.avif".to_string(),
            links: Some(vec![
                TeamMemberLink {
                    logo: Some(svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/yasharpm".to_string()),
                },
                TeamMemberLink {
                    logo: Some(svg::logo_github().to_string()),
                    url: Some("https://github.com/yasharpm".to_string()),
                },
            ]),
        },
    ]
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    pub team: Vec<TeamMember>,
}

#[derive(Clone, Debug)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub description: String,
    pub image_url: String,
    pub links: Option<Vec<TeamMemberLink>>,
}

#[derive(Clone, Debug)]
pub struct TeamMemberLink {
    pub logo: Option<String>,
    pub url: Option<String>,
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
        hero::view_hero_section(),
        features::view_feature_list(),
        cta::view_call_to_action(),
        team::view_our_team(&model.team),
    ]]
}
