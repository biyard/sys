use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn MembersPage(lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: MembersTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "members", "{tr.title} PAGE" } // end of this page
    }
}
