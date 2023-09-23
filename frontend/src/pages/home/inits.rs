use seed::prelude::*;

use crate::components;
use crate::pages::home::models;
use crate::pages::home::updates;

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<updates::Msg>) -> models::Model {
    models::Model { team: init_team() }
}

fn init_team() -> Vec<models::TeamMember> {
    vec![
        models::TeamMember {
            name: "Michiel de Jong".to_string(),
            role: "Code Communist".to_string(),
            description: "Michiel de Jong is an independent freedom hacker, travelling the world \
            while coding for the revolution from his laptop. "
                .to_string(),
            image_url: "/assets/images/vendor/michiel_dejong.avif".to_string(),
            links: Some(vec![
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/michielbdejong".to_string()),
                },
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_github().to_string()),
                    url: Some("https://github.com/michielbdejong".to_string()),
                },
            ]),
        },
        models::TeamMember {
            name: "Mahdi Baghbani".to_string(),
            role: "Percussive Maintainer".to_string(),
            description: "Don’t tell your IT staff I said this… but there’s nothing like a good \
            whack to fix a malfunctioning machine."
                .to_string(),
            image_url: "/assets/images/vendor/mahdi_baghbani.avif".to_string(),
            links: Some(vec![
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/mahdibaghbani".to_string()),
                },
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_github().to_string()),
                    url: Some("https://github.com/MahdiBaghbani".to_string()),
                },
            ]),
        },
        models::TeamMember {
            name: "Yashar PourMohammad".to_string(),
            role: "Code Guru".to_string(),
            description: "Yashar is keen to understand the inner workings of mind, code and \
            society. He thinks too much and codes too little."
                .to_string(),
            image_url: "/assets/images/vendor/yashar_pourmohammad.avif".to_string(),
            links: Some(vec![
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_linkedin().to_string()),
                    url: Some("https://www.linkedin.com/in/yasharpm".to_string()),
                },
                models::TeamMemberLink {
                    logo: Some(components::svg::logo_github().to_string()),
                    url: Some("https://github.com/yasharpm".to_string()),
                },
            ]),
        },
    ]
}