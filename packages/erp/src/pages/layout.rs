#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupZone;

use crate::route::Route;
use by_components::loaders::cube_loader::CubeLoader;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        RootBase { lang }
    }
}

#[component]
pub fn RootBase(lang: Language, children: Element) -> Element {
    rsx! {
        PopupZone {
            background_color: "rgba(26, 26, 26, 1)",
            border_class: "shadow-[#FFCE4740] shadow-2xl", // FIXME: need shadow size to 100px
        }
        div { class: "w-full h-full bg-background text-white flex flex-row items-start justify-start",
            div { class: "fixed top-0 left-0 w-250 h-[calc(100vh-20px)] flex flex-col gap-20 bg-black rounded-[20px] m-10 px-20 py-50",

                div { class: "w-full flex flex-col",
                    p { class: "text-lg font-bold", "Ratel" }
                    Link {
                        to: Route::PoliticiansPage { lang },
                        class: "text-white pl-20 hover:text-primary text-base font-semibold",
                        span { "Politicians" }
                    }

                }
            }

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        CubeLoader {}
                    }
                },
                div { class: "ml-270 h-full grow overflow-x-hidden scroll-smooth flex flex-col items-center justify-center",
                    Outlet::<Route> {}

                    {children}
                }
            }
        }
    }
}
