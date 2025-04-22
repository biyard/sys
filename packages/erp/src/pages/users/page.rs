#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn UsersPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: UsersTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            id: "users",
            class: "w-full h-dvh flex flex-col items-center justify-center bg-shade",
            button {
                class: "px-40 py-20 bg-primary text-white font-bold rounded-full",
                onclick: move |_| async move {
                    ctrl.handle_google().await;
                },
                "Google Sign in"
            }
        } // end of this page
    }
}
