use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn NewsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: NewsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "news", "{tr.title} PAGE" } // end of this page
    }
}
