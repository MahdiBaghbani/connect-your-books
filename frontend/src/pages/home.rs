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
    vec![main![
        C!["bg-white", "dark:bg-gray-900"],
        view_hero_section(),
        view_feature_list(),
        view_call_to_action(),
    ]]
}

fn view_hero_section() -> Node<Msg> {
    section![
        C!["bg-white", "dark:bg-gray-900"],
        div![
            C![
                "grid",
                "max-w-screen-xl",
                "px-4",
                "py-8",
                "mx-auto",
                "lg:gap-8",
                "xl:gap-0",
                "lg:py-16",
                "lg:grid-cols-12"
            ],
            div![
                C!["mr-auto", "place-self-center", "lg:col-span-7"],
                h1![
                    C![
                        "max-w-2xl",
                        "mb-4",
                        "text-4xl",
                        "font-extrabold",
                        "tracking-tight",
                        "leading-none",
                        "md:text-5xl",
                        "xl:text-6xl",
                        "dark:text-white"
                    ],
                    "Payments tool for software companies"
                ],
                p![
                    C![
                        "max-w-2xl",
                        "mb-6",
                        "font-light",
                        "text-gray-500",
                        "lg:mb-8",
                        "md:text-lg",
                        "lg:text-xl",
                        "dark:text-gray-400"
                    ],
                    "From checkout to global sales tax compliance, companies around the world \
                        use Flowbite to simplify their payment stack."
                ],
                a![
                    C![
                        "inline-flex",
                        "items-center",
                        "justify-center",
                        "px-5",
                        "py-3",
                        "mr-3",
                        "text-base",
                        "font-medium",
                        "text-center",
                        "text-white",
                        "rounded-lg",
                        "bg-primary-700",
                        "hover:bg-primary-800",
                        "focus:ring-4",
                        "focus:ring-primary-300",
                        "dark:focus:ring-primary-900"
                    ],
                    attrs! {
                        At::Href=>"#"
                    },
                    "Get started"
                ],
                a![
                    C![
                        "inline-flex",
                        "items-center",
                        "justify-center",
                        "px-5",
                        "py-3",
                        "text-base",
                        "font-medium",
                        "text-center",
                        "text-gray-900",
                        "border",
                        "border-gray-300",
                        "rounded-lg",
                        "hover:bg-gray-100",
                        "focus:ring-4",
                        "focus:ring-gray-100",
                        "dark:text-white",
                        "dark:border-gray-700",
                        "dark:hover:bg-gray-700",
                        "dark:focus:ring-gray-800"
                    ],
                    attrs! {
                        At::Href=>"#"
                    },
                    "Speak to Sales"
                ],
            ],
            div![
                C!["hidden", "lg:mt-0", "lg:col-span-5", "lg:flex"],
                img![attrs! {
                    At::Src=>"https://flowbite.s3.amazonaws.com/blocks/marketing-ui/hero/phone-mockup.png",
                    At::Alt=>"mockup"
                }]
            ],
        ],
    ]
}

fn view_feature_list() -> Node<Msg> {
    section![
        C![
            "bg-white",
            "dark:bg-gray-900"""
        ],
        div![
            C![
                "py-8",
                "px-4",
                "mx-auto",
                "max-w-screen-xl",
                "sm:py-16",
                "lg:px-6"
            ],
            div![
                C!["max-w-screen-md", "mb-8", "lg:mb-16"],
                h2![
                    C![
                        "mb-4"
                        "text-4xl",
                        "tracking-tight",
                        "font-extrabold",
                        "text-gray-900",
                        "dark:text-white"
                    ],
                    "Designed for business teams like yours"
                ],
                p![
                    C!["text-gray-500", "sm:text-xl", "dark:text-gray-400"],
                    "Here at Flowbite we focus on markets where technology, innovation, \
                    and capital can unlock long-term value and drive economic growth."
                ]
            ],
            div![
                C![
                    "space-y-8",
                    "md:grid",
                    "md:grid-cols-2",
                    "lg:grid-cols-3",
                    "md:gap-12",
                    "md:space-y-0"
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
                div![
                    div![
                        C![
                            "flex",
                            "justify-center",
                            "items-center",
                            "mb-4",
                            "w-10",
                            "h-10",
                            "rounded-full",
                            "bg-primary-100",
                            "lg:h-12",
                            "lg:w-12",
                            "dark:bg-primary-900"
                        ],
                        img![attrs! {
                            At::Src=>"/assets/images/logo.svg"
                        }]
                    ],
                    h3![
                        C!["mb-2", "text-xl", "font-bold", "dark:text-white"],
                        "Marketing"
                    ],
                    p![
                        C!["text-gray-500", "dark:text-gray-400"],
                        "Plan it, create it, launch it. Collaborate seamlessly with all the \
                        organization and hit your marketing goals every month with our \
                        marketing plan."
                    ],
                ],
            ]
        ]
    ]
}

fn view_call_to_action() -> Node<Msg> {
    section![
        C!["bg-white", "dark:bg-gray-900"],
        div![
            C!["gap-8",
                "items-center", "py-8", "px-4", "mx-auto", "max-w-screen-xl", "xl:gap-16" "md:grid", "md:grid-cols-2"
                "sm:py-16", "lg:px-6"],
            img![
                C!["w-full", "dark:hidden"],
                attrs! {
                    At::Src=>"https://flowbite.s3.amazonaws.com/blocks/marketing-ui/cta/cta-dashboard-mockup.svg",
                    At::Alt=>"dashboard image"
                }
            ],
            img![
                C!["w-full", "hidden", "dark:block"],
                attrs! {
                    At::Src=>"https://flowbite.s3.amazonaws.com/blocks/marketing-ui/cta/cta-dashboard-mockup-dark.svg",
                    At::Alt=>"dashboard image"
                }
            ],
            div![
                C!["mt-4", "md:mt-0"],
                h2![
                    C!["mb-4", "text-4xl" "tracking-tight", "font-extrabold", "text-gray-900",
                        "dark:text-white"],
                    "Let's create more tools and ideas that brings us together."
                ],
                p![
                    C![
                        "mb-6",
                        "font-light",
                        "text-gray-500",
                        "md:text-lg",
                        "dark:text-gray-400"
                    ],
                    "Flowbite helps you connect with friends and communities of people who share
                    your interests. Connecting with your friends and family as well as discovering
                    new ones is easy with features like Groups."
                ],
                a![
                    C!["inline-flex"
                        "items-center", "text-white", "bg-primary-700", "hover:bg-primary-800",
                        "focus:ring-4", "focus:ring-primary-300", "font-medium", "rounded-lg",
                        "text-sm", "px-5", "py-2.5", "text-center", "dark:focus:ring-primary-900"],
                    "Get started"
                ],
            ],
        ],
    ]
}
