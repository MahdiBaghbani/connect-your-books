// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use crate::{Model, Msg, User};

pub fn view_navigation_bar(model: &Model, base_url: &Url, user: Option<&User>) -> Vec<Node<Msg>> {
    vec![nav![
        C!["bg-gray-800"],
        view_navigation_bar_desktop(model, base_url, user),
        view_navigation_bar_mobile(model, base_url, user),
    ]]
}

fn view_navigation_bar_desktop(model: &Model, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["mx-auto", "max-w-7xl", "px-4", "sm:px-6", "lg:px-8"],
        div![
            C!["flex", "h-16", "items-center", "justify-between"],
            view_left_side(model, base_url, user),
            view_right_side(model, base_url, user),
            view_navigation_bar_mobile_button(model, base_url, user),
        ],
    ]
}

fn view_left_side(model: &Model, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["flex", "items-center"],
        // ------ Logo ------
        div![
            C!["flex-shrink-0"],
            a![
                attrs! {
                    At::Href => base_url,
                },
                img![
                    C!["h-8", "w-8"],
                    attrs! {
                        At::Src => "/assets/images/logo.svg",
                        At::Alt => "Ponder Source"
                    }
                ],
            ],
        ],
        if user.is_some() {
            // ------ Navigation Bar Items ------
            div![
                C!["hidden", "md:block"],
                div![
                    C!["ml-10", "flex", "items-baseline", "space-x-4"],
                    // Active Item: "bg-gray-900 text-white",
                    // Default: "text-gray-300 hover:bg-gray-700 hover:text-white".
                    model.navigation_bar.iter().map(|item| {
                        let id: u8 = item.id;
                        a![
                            if id == model.navigation_bar_active_item_id {
                                C!["bg-gray-900", "text-white"]
                            } else {
                                C!["text-gray-300", "hover:bg-gray-700", "hover:text-white"]
                            },
                            C!["rounded-md", "px-3", "py-2", "text-sm", "font-medium"],
                            attrs! {At::Href => format!("{}", item.href)},
                            format!("{}", item.name),
                            ev(Ev::Click, move |_| Msg::ChangeNavigationBarActiveItem(id)),
                        ]
                    }),
                ],
            ]
        } else {
            div![]
        },
    ]
}

fn view_right_side(model: &Model, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["hidden", "md:block"],
        div![
            C!["ml-4", "flex", "items-center", "md:ml-6"],
            button![
                C![
                    "text-gray-500",
                    "dark:text-gray-400",
                    "hover:bg-gray-100",
                    "dark:hover:bg-gray-700",
                    "rounded-lg",
                    "text-sm",
                    "p-2.5",
                    "inline-flex",
                    "items-center"
                ],
                attrs! {
                    At::Type=>"button",
                },
                svg![
                    C!["w-5", "h-5"],
                    attrs! {
                        At::Xmlns=>"http://www.w3.org/2000/svg",
                        At::ViewBox=>"0 0 20 20",
                        At::Fill=>"currentColor",
                        At::AriaHidden=>"true"
                    },
                    if model.isDarkMode {
                        path![attrs! {
                            At::D=>"M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 \
                            0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 \
                            0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 \
                            11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 \
                            100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 \
                            1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 \
                            8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 \
                            11a1 1 0 100-2H3a1 1 0 000 2h1z",
                            At::FillRule=>"evenodd",
                            At::ClipRule=>"evenodd",
                        }]
                    } else {
                        path![attrs! {
                            At::D=>"M17.293 13.293A8 8 0 016.707 \
                            2.707a8.001 8.001 0 1010.586 10.586z"
                        }]
                    },
                ],
                ev(Ev::Click, move |_| Msg::ToggleDarkMode),
            ],
            if let Some(user) = user {
                view_profile_button(model, base_url, user)
            } else {
                view_signin_buttons(model, base_url)
            }
        ],
    ]
}

fn view_signin_buttons(model: &Model, base_url: &Url) -> Node<Msg> {
    button![
        C![
            "text-white",
            "bg-blue-700",
            "hover:bg-blue-800",
            "font-medium rounded-lg",
            "text-sm",
            "px-3",
            "py-2.5",
            "m-2",
            "rounded-lg",
            "dark:bg-blue-600",
            "dark:hover:bg-blue-700"
        ],
        "Sign in"
    ]
}

fn view_profile_button(model: &Model, base_url: &Url, user: &User) -> Node<Msg> {
    div![
        C!["relative", "ml-3"],
        div![button![
            C![
                "flex",
                "max-w-xs",
                "items-center",
                "rounded-full",
                "bg-gray-800",
                "text-sm",
                "focus:outline-none",
                "focus:ring-2",
                "focus:ring-white",
                "focus:ring-offset-2",
                "focus:ring-offset-gray-800"
            ],
            attrs! {
                At::Type=>"button",
                At::Id=>"",
                At::AriaExpanded=>"false",
                At::AriaHasPopup=>"true"
            },
            span![C!["sr-only"], "Open user menu"],
            img![
                C!["h-8", "w-8", "rounded-full"],
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
        ],],
        // ------ Profile dropdown ------
        IF!(model.profile_menu_visible =>
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
                    "focus:outline-none"
                ],
                attrs! {
                    At::Role=>"menu",
                    At::TabIndex=>"-1",
                    At::AriaLabelledBy=>"user-menu-button"
                },
                model.profile_menu.iter().map(|item| {
                    let id: u8 = item.id;
                    a![
                        C!["block", "px-4", "py-2", "text-sm", "text-gray-700"],
                        attrs! {
                            At::Id=>format!("user-menu-item-{}", id),
                            At::Href => format!("{}", item.href),
                            At::Role=>"menuitem", At::TabIndex=>"-1"
                        },
                        format!("{}", item.name),
                    ]
                }),
            ]
        ),
    ]
}

fn view_navigation_bar_mobile_button(
    model: &Model,
    base_url: &Url,
    user: Option<&User>,
) -> Node<Msg> {
    div![
        C!["-mr-2", "flex", "md:hidden"],
        // Mobile menu button.
        button![
            C![
                "inline-flex",
                "items-center",
                "justify-center",
                "rounded-md",
                "bg-gray-800",
                "p-2",
                "text-gray-400",
                "hover:bg-gray-700",
                "hover:text-white",
                "focus:outline-none",
                "focus:ring-2",
                "focus:ring-white",
                "focus:ring-offset-2",
                "focus:ring-offset-gray-800"
            ],
            attrs! {
                At::Type=>"button",
                At::AriaControls=>"mobile-menu",
                At::AriaExpanded=> if model.navigation_bar_mobile_visible {
                    "true"
                } else {
                    "false"
                }
            },
            span![C!["sr-only"], "Open main menu"],
            // Menu open: "hidden", Menu closed: "block".
            svg![
                if model.navigation_bar_mobile_visible {
                    C!["hidden"]
                } else {
                    C!["block"]
                },
                C!["h-6", "w-6"],
                attrs! {
                    At::Fill=>"none",
                    At::ViewBox=>"0 0 24 24",
                    At::Stroke=>"currentColor",
                    At::StrokeWidth=>"1.5",
                    At::AriaHidden=>"true"
                },
                path![attrs! {
                    At::StrokeLinecap=>"round",
                    At::StrokeLineJoin=>"round",
                    At::D=>"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                }],
            ],
            // Menu open: "block", Menu closed: "hidden".
            svg![
                if model.navigation_bar_mobile_visible {
                    C!["block"]
                } else {
                    C!["hidden"]
                },
                C!["h-6", "w-6"],
                attrs! {
                    At::Fill=>"none",
                    At::ViewBox=>"0 0 24 24",
                    At::Stroke=>"currentColor",
                    At::StrokeWidth=>"1.5",
                    At::AriaHidden=>"true"
                },
                path![attrs! {
                    At::StrokeLinecap=>"round",
                    At::StrokeLineJoin=>"round",
                    At::D=>"M6 18L18 6M6 6l12 12"
                }],
            ],
            ev(Ev::Click, move |_| Msg::ToggleNavigationBarMobileView),
        ],
    ]
}

fn view_navigation_bar_mobile(model: &Model, base_url: &Url, user: Option<&User>) -> Node<Msg> {
    div![
        C!["md:hidden"],
        attrs! {At::Id=>"mobile-menu"},
        IF!(not(model.navigation_bar_mobile_visible) => style!{St::Display => "none"}),
        div![
            C!["space-y-1", "px-2", "pb-3", "pt-2", "sm:px-3"],
            // Active Item: "bg-gray-900 text-white",
            // Default: "text-gray-300 hover:bg-gray-700 hover:text-white".
            model.navigation_bar.iter().map(|item| {
                let id: u8 = item.id;
                a![
                    if id == model.navigation_bar_active_item_id {
                        C!["bg-gray-900", "text-white"]
                    } else {
                        C!["text-gray-300", "hover:bg-gray-700", "hover:text-white"]
                    },
                    C![
                        "block",
                        "rounded-md",
                        "px-3",
                        "py-2",
                        "text-base",
                        "font-medium"
                    ],
                    attrs! {At::Href => format!("{}", item.href)},
                    format!("{}", item.name),
                    ev(Ev::Click, move |_| Msg::ChangeNavigationBarActiveItem(id)),
                ]
            }),
        ],
        div![
            C!["border-t", "border-gray-700", "pb-3", "pt-4"],
            div![
                C!["flex", "items-center", "px-5"],
                div![
                    C!["flex-shrink-0"],
                    img![
                        C!["h-10", "w-10", "rounded-full"],
                        attrs! {
                            At::Src=>"/assets/images/profile-mahdi-baghbani.avif",
                            At::Alt=>"profile picture"
                        }
                    ],
                ],
                div![
                    C!["ml-3"],
                    div![
                        C!["text-base", "font-medium", "leading-none", "text-white"],
                        "Tom Cook"
                    ],
                    div![
                        C!["text-sm", "font-medium", "leading-none", "text-gray-400"],
                        "tom@example.com"
                    ],
                ],
            ],
            div![
                C!["mt-3", "space-y-1", "px-2"],
                model.profile_menu.iter().map(|item| {
                    a![
                        C![
                            "block",
                            "rounded-md",
                            "px-3",
                            "py-2",
                            "text-base",
                            "font-medium",
                            "text-gray-400",
                            "hover:bg-gray-700",
                            "hover:text-white"
                        ],
                        format!("{}", item.name),
                    ]
                }),
            ],
        ],
    ]
}
