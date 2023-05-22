// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use chrono::prelude::*;
use seed::{*, prelude::*};

use crate::Msg;

pub fn view_footer() -> Vec<Node<Msg>> {
    vec![footer![
        C![
            "p-4", "bg-white", "md:p-8", "lg:p-10" "dark:bg-gray-800"
        ],
        div![
            C!["mx-auto", "max-w-screen-xl", "text-center"],
            a![
                C![
                    "flex",
                    "justify-center",
                    "items-center",
                    "text-2xl",
                    "font-semibold",
                    "text-gray-900",
                    "dark:text-white"
                ],
                attrs! {
                    At::Href=>"#"
                },
                img![
                    C!["mr-2", "h-8"],
                    attrs! {
                        At::Src=>"/assets/images/logo.svg",
                        At::Alt=>"Ponder Source Logo"
                    }
                ],
                "Ponder Source"
            ],
            p![
                C!["my-6", "text-gray-500", "dark:text-gray-400"],
                "Open-source library of over 400+ web components and \
                    interactive elements built for better web."
            ],
            ul![
                C![
                    "flex",
                    "flex-wrap",
                    "justify-center",
                    "items-center",
                    "mb-6 text-gray-900",
                    "dark:text-white"
                ],
                li![a![
                    C!["mr-4", "hover:underline", "md:mr-6 "],
                    attrs! {
                        At::Href=>"#"
                    },
                    "About"
                ],],
            ],
            span![
                C![
                    "text-sm",
                    "text-gray-500",
                    "sm:text-center",
                    "dark:text-gray-400"
                ],
                // don't remove spaces in below strings..
                format!("Copyleft 2022 - {} ", Utc::now().year()),
                a![C!["hover:underline"], "Ponder Source"],
                ". All Rights Reserved."
            ],
        ],
    ]]
}
