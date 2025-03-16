#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use by_components::meta::MetaPage;
use controller::Controller;
use i18n::HomePageTranslate;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let _ctrl = Controller::new(lang)?;
    let tr: HomePageTranslate = translate(&lang);

    rsx! {
        MetaPage { title: tr.title }
        div {
            id: "home-page",
            class: "flex flex-col w-full justify-start items-center",
        }
    }
}
