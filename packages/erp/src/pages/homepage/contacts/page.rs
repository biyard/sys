use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ContactsPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: ContactsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "contacts", "{tr.title} PAGE" } // end of this page
    }
}
