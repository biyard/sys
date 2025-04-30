use crate::pages::PresidentialCandidatesNewPage;

use bdk::prelude::*;

#[component]
pub fn PresidentialCandidatesEditByIdPage(id: i64, lang: Language) -> Element {
    rsx! {
        PresidentialCandidatesNewPage { lang, id }
    }
}
