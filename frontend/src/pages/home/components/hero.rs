use seed::{*, prelude::*};

use crate::pages::home::updates;

pub fn view_hero_section() -> Node<updates::Msg> {
    section![
        C!["bg-white", "dark:bg-gray-900", "mt-[65px]"],
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
