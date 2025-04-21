use bdk::prelude::*;

use crate::pages::MembersNewPage;

#[component]
pub fn MembersEditPage(lang: Language, id: i64) -> Element {
    rsx! {
        MembersNewPage { lang, id }
    }
}
