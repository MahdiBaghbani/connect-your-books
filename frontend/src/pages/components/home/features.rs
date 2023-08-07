// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{*, prelude::*};

use crate::pages::home::Msg;

pub fn view_feature_list() -> Node<Msg> {
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
                            At::Src=>"/assets/icons/logo.svg"
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
                            At::Src=>"/assets/icons/logo.svg"
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
                            At::Src=>"/assets/icons/logo.svg"
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
                            At::Src=>"/assets/icons/logo.svg"
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
                            At::Src=>"/assets/icons/logo.svg"
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
                            At::Src=>"/assets/icons/logo.svg"
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
