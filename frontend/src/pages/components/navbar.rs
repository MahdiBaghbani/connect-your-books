// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{*, prelude::*};

use crate::{Model, Msg, User};
use crate::components::svg;

pub fn view_navbar(model: &Model, base_url: &Url, user: Option<&User>) -> Vec<Node<Msg>> {
    vec![nav![
        C![
            "bg-white",
            "dark:bg-gray-900",
            "fixed",
            "w-full",
            "z-20",
            "top-0",
            "left-0",
            "border-b",
            "border-gray-200",
            "dark:border-gray-600"
        ],
        div![
            C![
                "max-w-screen-xl",
                "flex",
                "flex-wrap",
                "items-center",
                "justify-between",
                "mx-auto p-4"
            ],
            view_navbar_left(base_url),
            view_navbar_right(model, user),
            // middle section should be last item here do not move it up or down!
            IF!(user.is_some()=>view_navbar_middle(model))
        ]
    ]]
}

fn view_navbar_left(base_url: &Url) -> Node<Msg> {
    // ------ Logo ------
    a![
        C!["flex", "items-center"],
        attrs! {
            At::Href => base_url,
        },
        img![
            C!["h-8", "mr-3"],
            attrs! {
                At::Src => "/assets/icons/logo.svg",
                At::Alt => "Connect Your Books Logo"
            }
        ],
        span![
            C![
                "self-center",
                "text-2xl",
                "font-semibold",
                "whitespace-nowrap",
                "dark:text-white"
            ],
            "CYB"
        ]
    ]
}

fn view_navbar_middle(model: &Model) -> Node<Msg> {
    div![
        IF!(!model.navbar_hamburger_menu_visible => C!["hidden"]),
        C![
            "items-center",
            "justify-between",
            "w-full",
            "md:order-1",
            "md:flex",
            "md:w-auto",
            "md:block"
        ],
        ul![
            C![
                "flex",
                "flex-col",
                "p-4",
                "md:p-0",
                "mt-4",
                "font-medium",
                "border",
                "border-gray-100",
                "rounded-lg",
                "bg-gray-50",
                "md:flex-row",
                "md:space-x-8",
                "md:mt-0",
                "md:border-0",
                "md:bg-white",
                "dark:bg-gray-800",
                "md:dark:bg-gray-900",
                "dark:border-gray-700"
            ],
            model.navbar.iter().map(|item| {
                let id: u8 = item.id;
                a![
                    C!["block", "py-2", "pl-3", "pr-4", "rounded"],
                    if id == model.navbar_active_item_id {
                        C![
                            "text-white",
                            "bg-blue-700",
                            "md:bg-transparent",
                            "md:text-blue-700",
                            "md:p-0",
                            "md:dark:text-blue-500"
                        ]
                    } else {
                        C![
                            "text-gray-900",
                            "hover:bg-gray-100",
                            "md:hover:bg-transparent",
                            "md:hover:text-blue-700",
                            "md:p-0",
                            "md:dark:hover:text-blue-500",
                            "dark:text-white",
                            "dark:hover:bg-gray-700",
                            "dark:hover:text-white",
                            "md:dark:hover:bg-transparent",
                            "dark:border-gray-700"
                        ]
                    },
                    attrs! {At::Href => format!("{}", item.href)},
                    format!("{}", item.name),
                    ev(Ev::Click, move |_| Msg::ChangeNavBarActiveItem(id)),
                ]
            }),
        ],
    ]
}

fn view_navbar_right(model: &Model, user: Option<&User>) -> Node<Msg> {
    div![
        C!["flex", "items-center", "md:order-2",],
        view_dark_mode_button(&model.is_dark_mode),
        if let Some(_user) = user {
            vec![
                view_profile_button(model),
                view_navbar_hamburger_menu(&model.navbar_hamburger_menu_visible),
            ]
        } else {
            vec![view_signin_button()]
        },
    ]
}

fn view_dark_mode_button(is_dark_mode: &bool) -> Node<Msg> {
    button![
        C![
            "inline-flex",
            "items-center",
            "p-2",
            "ml-3",
            "text-sm",
            "text-gray-500",
            "rounded-lg",
            "hover:bg-gray-100",
            "focus:outline-none",
            "focus:ring-2",
            "focus:ring-gray-200",
            "dark:text-gray-400",
            "dark:hover:bg-gray-700",
            "dark:focus:ring-gray-600"
        ],
        attrs! {
            At::Type=>"button",
        },
        span![C!["sr-only"], "Light mode, dark mode switch button"],
        svg![
            C!["w-6", "h-6"],
            attrs! {
                At::Xmlns=>"http://www.w3.org/2000/svg",
                At::ViewBox=>"0 0 20 20",
                At::Fill=>"currentColor",
                At::AriaHidden=>"true"
            },
            if *is_dark_mode {
                path![attrs! {
                    At::D=>svg::logo_sun(),
                    At::FillRule=>"evenodd",
                    At::ClipRule=>"evenodd",
                }]
            } else {
                path![attrs! {
                    At::D=>svg::logo_moon(),
                }]
            },
        ],
        ev(Ev::Click, move |_| Msg::ToggleDarkMode),
    ]
}

fn view_profile_button(model: &Model) -> Node<Msg> {
    div![
        C!["relative"],
        button![
            C![
                "flex",
                "ml-3",
                "text-sm",
                "bg-gray-800",
                "rounded-lg",
                "md:mr-0",
                "focus:ring-4",
                "focus:ring-gray-300",
                "dark:focus:ring-gray-600",
            ],
            attrs! {
                At::Type=>"button",
                At::Id=>"user-menu-button",
                At::AriaExpanded=>"false",
                At::AriaHasPopup=>"true",
            },
            span![C!["sr-only"], "Open user menu"],
            img![
                C!["h-10", "w-10", "rounded-lg"],
                attrs! {
                    At::Src=>"/assets/images/profile-mahdi-baghbani.avif",
                    At::Alt=>"profile picture"
                }
            ],
            ev(Ev::Click, move |event| {
                // prevent Msg::HideProfileMenu.
                event.stop_propagation();
                Msg::ToggleProfileMenu
            }),
        ],
        IF!(model.profile_menu_visible=>view_profile_dropdown(model)),
    ]
}

fn view_profile_dropdown(model: &Model) -> Node<Msg> {
    div![
        C![
            "absolute",
            "right-0",
            "z-10",
            "mt-2",
            "w-48",
            "origin-top-right",
            "rounded-md",
            "bg-white",
            "py-1",
            "shadow-lg",
            "ring-1",
            "ring-black",
            "ring-opacity-5",
            "focus:outline-none",
            "dark:bg-gray-700",
            "dark:divide-gray-600"
        ],
        attrs! {
            At::Role=>"menu",
            At::TabIndex=>"-1",
            At::AriaLabelledBy=>"user-menu-button"
        },
        div![
            C!["px-4", "py-3"],
            span![
                C!["block", "text-sm", "text-gray-900", "dark:text-white"],
                "Michiel de jong"
            ],
            span![
                C![
                    "block",
                    "text-sm",
                    "text-gray-500",
                    "truncate",
                    "dark:text-gray-400"
                ],
                "michiel@ponder.org"
            ],
        ],
        ul![
            C!["py-2"],
            model.profile_menu.iter().map(|item| {
                let id: u8 = item.id;
                li![a![
                    C![
                        "block",
                        "px-4",
                        "py-2",
                        "text-sm",
                        "text-gray-700",
                        "hover:bg-gray-100",
                        "dark:hover:bg-gray-600",
                        "dark:text-gray-200",
                        "dark:hover:text-white"
                    ],
                    attrs! {
                        At::Id=>format!("user-menu-item-{}", id),
                        At::Href => format!("{}", item.href),
                        At::Role=>"menuitem", At::TabIndex=>"-1"
                    },
                    format!("{}", item.name),
                ]]
            }),
        ],
    ]
}

fn view_signin_button() -> Node<Msg> {
    button![
        C![
            "text-white",
            "bg-blue-700",
            "hover:bg-blue-800",
            "focus:ring-4",
            "focus:outline-none",
            "focus:ring-blue-300,"
            "font-medium",
            "rounded-lg",
            "text-sm",
            "px-3",
            "py-2.5",
            "text-center",
            "ml-3",
            "rounded-lg",
            "dark:bg-blue-600",
            "dark:hover:bg-blue-700",
            "dark:focus:ring-blue-800"
        ],
        "Sign in"
    ]
}

fn view_navbar_hamburger_menu(visible: &bool) -> Node<Msg> {
    button![
        C![
            "inline-flex",
            "items-center",
            "p-2",
            "ml-3",
            "text-sm",
            "text-gray-500",
            "rounded-lg",
            "md:hidden",
            "hover:bg-gray-100",
            "focus:outline-none",
            "focus:ring-2",
            "focus:ring-gray-200"
            "dark:text-gray-400",
            "dark:hover:bg-gray-700",
            "dark:focus:ring-gray-600"
        ],
        attrs! {
            At::Type=>"button",
            At::AriaControls=>"mobile-menu",
            At::AriaExpanded=> if *visible {
                "true"
            } else {
                "false"
            }
        },
        span![C!["sr-only"], "Open main menu"],
        // ------ Mobile menu button icon ------
        svg![
            C!["h-6", "w-6"],
            attrs! {
                At::Fill=>"none",
                At::ViewBox=>"0 0 24 24",
                At::Stroke=>"currentColor",
                At::StrokeWidth=>"1.5",
                At::AriaHidden=>"true"
            },
            path![
                attrs! {
                    At::StrokeLinecap=>"round",
                    At::StrokeLineJoin=>"round",
                },
                if *visible {
                    attrs! {
                        At::D=>"M6 18L18 6M6 6l12 12"
                    }
                } else {
                    attrs! {
                        At::D=>"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                    }
                },
            ],
        ],
        ev(Ev::Click, move |_| Msg::ToggleNavBarHamburgerView),
    ]
}
