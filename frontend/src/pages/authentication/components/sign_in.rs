use seed::{*, prelude::*};

use crate::pages::authentication::updates;

pub fn view_sign_in_section(base_url: &Url) -> Node<updates::Msg> {
    section![
        C!["bg-gray-50", "dark:bg-gray-900"],
        div![
            C![
                "flex",
                "flex-col",
                "items-center",
                "justify-center",
                "px-6",
                "py-8",
                "mx-auto",
                "md:h-screen",
                "lg:py-0"
            ],
            a![
                C![
                    "flex",
                    "items-center",
                    "mb-6",
                    "text-2xl",
                    "font-semibold",
                    "text-gray-900",
                    "dark:text-white"
                ],
                attrs! {
                    At::Href => base_url,
                },
                img![
                    C!["w-8", "h-8", "mr-2"],
                    attrs! {
                        At::Src => "/assets/icons/logo.svg",
                        At::Alt => "Connect Your Books Logo"
                    },
                ],
                "Connect Your Books",
            ],
            div![
                C![
                    "w-full",
                    "bg-white",
                    "rounded-lg",
                    "shadow",
                    "dark:border",
                    "md:mt-0",
                    "sm:max-w-md",
                    "xl:p-0",
                    "dark:bg-gray-800",
                    "dark:border-gray-700"
                ],
                div![
                    C!["p-6", "space-y-4", "md:space-y-6", "sm:p-8"],
                    h1![
                        C![
                            "text-xl",
                            "font-bold",
                            "leading-tight",
                            "tracking-tight",
                            "text-gray-900",
                            "md:text-2xl",
                            "dark:text-white"
                        ],
                        "Sign in to your account",
                    ],
                    form![
                        C!["space-y-4", "md:space-y-6"],
                        div![
                            label![
                                C![
                                    "block",
                                    "mb-2",
                                    "text-sm",
                                    "font-medium",
                                    "text-gray-900",
                                    "dark:text-white"
                                ],
                                attrs! {
                                    At::For => "username",
                                },
                                "Your username",
                            ],
                            input![
                                C![
                                    "bg-gray-50",
                                    "border",
                                    "border-gray-300",
                                    "text-gray-900",
                                    "sm:text-sm",
                                    "rounded-lg",
                                    "focus:ring-primary-600",
                                    "focus:border-primary-600"
                                    "block",
                                    "w-full",
                                    "p-2.5",
                                    "dark:bg-gray-700",
                                    "dark:border-gray-600",
                                    "dark:placeholder-gray-400",
                                    "dark:text-white dark:focus:ring-blue-500",
                                    "dark:focus:border-blue-500"
                                ],
                                attrs! {
                                    At::Id => "username",
                                    At::Name => "username",
                                    At::Type => "text",
                                    At::Placeholder => "User Name",
                                    At::Required => "",
                                },
                            ],
                        ],
                        div![
                            label![
                                C![
                                    "block",
                                    "mb-2",
                                    "text-sm",
                                    "font-medium",
                                    "text-gray-900",
                                    "dark:text-white"
                                ],
                                attrs! {
                                    At::For => "password",
                                },
                                "Password",
                            ],
                            input![
                                C![
                                    "bg-gray-50",
                                    "border",
                                    "border-gray-300",
                                    "text-gray-900",
                                    "sm:text-sm",
                                    "rounded-lg",
                                    "focus:ring-primary-600",
                                    "focus:border-primary-600"
                                    "block",
                                    "w-full",
                                    "p-2.5",
                                    "dark:bg-gray-700",
                                    "dark:border-gray-600",
                                    "dark:placeholder-gray-400",
                                    "dark:text-white dark:focus:ring-blue-500",
                                    "dark:focus:border-blue-500"
                                ],
                                attrs! {
                                    At::Id => "password",
                                    At::Name => "password",
                                    At::Type => "password",
                                    At::Placeholder => "••••••••",
                                    At::Required => "",
                                },
                            ],
                        ],
                        div![
                            C!["flex", "items-center", "justify-between"],
                            div![
                                C!["flex", "items-start"],
                                div![
                                    C!["flex", "items-center", "h-5"],
                                    input![
                                        C![
                                            "w-4",
                                            "h-4",
                                            "border",
                                            "border-gray-300",
                                            "rounded",
                                            "bg-gray-50",
                                            "focus:ring-3",
                                            "focus:ring-primary-300",
                                            "dark:bg-gray-700",
                                            "dark:border-gray-600",
                                            "dark:focus:ring-primary-600",
                                            "dark:ring-offset-gray-800"
                                        ],
                                        attrs! {
                                            At::Id => "remember",
                                            At::Type => "checkbox",
                                            At::AriaDescribedBy => "remember",
                                        },
                                    ],
                                ],
                                div![
                                    C!["ml-3", "text-sm"],
                                    label![
                                        C!["text-gray-500", "dark:text-gray-300"],
                                        attrs! {
                                            At::For => "remember",
                                        },
                                        "Remember me",
                                    ],
                                ],
                            ],
                            a![
                                C![
                                    "text-sm",
                                    "font-medium",
                                    "text-primary-600",
                                    "hover:underline",
                                    "dark:text-primary-500"
                                ],
                                attrs! {
                                    At::Href => "#",
                                },
                                "Forgot password?",
                            ],
                        ],
                        button![
                            C![
                                "w-full",
                                "text-white",
                                "bg-primary-600",
                                "hover:bg-primary-700",
                                "focus:ring-4",
                                "focus:outline-none",
                                "focus:ring-primary-300",
                                "font-medium",
                                "rounded-lg",
                                "text-sm",
                                "px-5",
                                "py-2.5",
                                "text-center",
                                "dark:bg-primary-600",
                                "dark:hover:bg-primary-700",
                                "dark:focus:ring-primary-800"
                            ],
                            attrs! {
                                At::Type => "submit",
                            },
                            "Sign in",
                        ],
                        p![
                            C![
                                "text-sm",
                                "font-light",
                                "text-gray-500",
                                "dark:text-gray-400"
                            ],
                            "Don’t have an account yet? ",
                            a![
                                C![
                                    "font-medium",
                                    "text-primary-600",
                                    "hover:underline",
                                    "dark:text-primary-500"
                                ],
                                attrs! {
                                    At::Href => "#",
                                },
                                "Sign up",
                            ],
                        ],
                    ],
                ],
            ],
        ],
    ]
}
