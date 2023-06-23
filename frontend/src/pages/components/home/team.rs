// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::pages::home::Msg;
use crate::pages::home::{TeamMember, TeamMemberLink};

pub fn view_our_team(team: &[TeamMember]) -> Node<Msg> {
    section![
        C!["bg-white", "dark:bg-gray-900"],
        div![
            C![
                "py-8",
                "px-4",
                "mx-auto",
                "max-w-screen-xl",
                "lg:py-16",
                "lg:px-6"
            ],
            div![
                C![
                    "mx-auto",
                    "max-w-screen-sm",
                    "text-center",
                    "mb-8",
                    "lg:mb-16"
                ],
                h2![
                    C![
                        "mb-4",
                        "text-4xl",
                        "tracking-tight",
                        "font-extrabold",
                        "text-gray-900",
                        "dark:text-white"
                    ],
                    "Our Team",
                ],
                p![
                    C![
                        "font-light",
                        "text-gray-500",
                        "lg:mb-16",
                        "sm:text-xl",
                        "dark:text-gray-400"
                    ],
                    "Explore the whole collection of open-source",
                ],
            ],
            div![
                C!["grid", "gap-6", "mb-2", "lg:mb-4", "md:grid-cols-3"],
                team.iter().map(|member| { view_team_member(member) }),
            ],
        ],
    ]
}

fn view_team_member(member: &TeamMember) -> Node<Msg> {
    div![
        C![
            "max-w-sm",
            "bg-white",
            "border",
            "border-gray-200",
            "rounded-lg",
            "shadow",
            "dark:bg-gray-800",
            "dark:border-gray-700"
        ],
        a![
            attrs! {
                At::Href=>"#"
            },
            img![
                C!["rounded-t-lg"],
                attrs! {
                    At::Src=>member.image_url,
                    At::Alt=>format!("{} Avatar", member.name)
                }
            ],
        ],
        div![
            C!["p-5"],
            h3![
                C![
                    "text-xl",
                    "font-bold",
                    "tracking-tight",
                    "text-gray-900",
                    "dark:text-white"
                ],
                a![
                    attrs! {
                        At::Href=>"#"
                    },
                    member.name.clone(),
                ]
            ],
            span![
                C!["text-gray-500", "dark:text-gray-400"],
                member.role.clone(),
            ],
            p![
                C![
                    "mt-3",
                    "mb-4",
                    "font-light",
                    "text-gray-500",
                    "dark:text-gray-400"
                ],
                member.description.clone(),
            ],
            if let Some(links) = member.links.clone() {
                vec![ul![
                    C!["flex", "space-x-4", "sm:mt-0"],
                    links.iter().map(|link| { view_team_member_link(link) }),
                ]]
            } else {
                vec![]
            },
        ],
    ]
}

fn view_team_member_link(link: &TeamMemberLink) -> Node<Msg> {
    li![a![
        C![
            "text-gray-500",
            "hover:text-gray-900",
            "dark:hover:text-white"
        ],
        if let Some(url) = link.url.clone() {
            attrs! {
                At::Href=>url
            }
        } else {
            attrs! {
                At::Href=>""
            }
        },
        if let Some(logo) = link.logo.clone() {
            vec![svg![
                C!["w-5", "h-5"],
                attrs! {
                    At::Xmlns=>"http://www.w3.org/2000/svg",
                    At::Fill=>"currentColor",
                    At::ViewBox=>"0 0 24 24",
                    At::AriaHidden=>"true",
                    At::ClipRule=>"evenodd",
                },
                path![attrs! {
                    At::FillRule=>"evenodd",
                    At::D=>logo,
                },],
            ]]
        } else {
            vec![]
        },
    ],]
}
