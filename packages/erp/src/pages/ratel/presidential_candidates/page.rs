use crate::route::Route;

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn PresidentialCandidatesPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: PresidentialCandidatesTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "presidential-candidates", class: "col",
            div { class: "row w-full justify-between",
                div { class: "row gap-20 justify-between w-full",
                    "{tr.title} PAGE"

                    Link {
                        class: "btn-primary",
                        to: Route::PresidentialCandidatesNewPage {
                            lang,
                        },
                        "New"
                    }
                }
            } // end of header

            for c in ctrl.candidates()? {
                div { class: "col gap-20",
                    div { class: "row justify-between",
                        h2 { class: "text-2xl", {c.name} }
                        button {
                            class: "btn-secondary",
                            onclick: move |_| async move {
                                ctrl.handle_delete(c.id).await;
                            },
                            "Delete"
                        }
                                        // Link {
                    //     class: "btn-primary",
                    //     to: Route::PresidentialCandidatesEditByIdPage {
                    //         lang,
                    //         id: c.id,
                    //     },
                    //     "Edit"
                    // }
                    }
                    div { class: "row gap-20",
                        img { class: "w-300 h-500", src: c.image }
                        div { class: "col gap-20",
                            p { class: "text-sm", {c.party.translate(&lang)} }
                            p { class: "text-sm", {c.crypto_stance.translate(&lang)} }
                            for p in c.election_pledges {
                                article { dangerous_inner_html: p.promise }
                            }
                        }
                    }
                }
            }

        } // end of this page
    }
}
