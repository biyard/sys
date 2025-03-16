#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn PoliticiansPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: PoliticiansTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "politicians", "{tr.title} PAGE" } // end of this page
    }
}
