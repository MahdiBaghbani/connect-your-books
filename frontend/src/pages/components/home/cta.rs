// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{*, prelude::*};

use crate::pages::home::Msg;

pub fn view_call_to_action() -> Node<Msg> {
    section![
        C!["bg-white", "dark:bg-gray-900"],
        div![
            C![
                "gap-8",
                "items-center",
                "py-8",
                "px-4",
                "mx-auto",
                "max-w-screen-xl",
                "xl:gap-16",
                "md:grid",
                "md:grid-cols-2",
                "sm:py-16",
                "lg:px-6"
            ],
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
