use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn UpdatesPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: UpdatesTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "updates", "{tr.title} PAGE" } // end of this page
    }
}
