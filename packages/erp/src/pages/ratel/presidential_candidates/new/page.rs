use crate::components::Dropdown;

use super::*;
use bdk::prelude::{by_components::rich_texts::RichText, *};
use common::ratel::*;
use controller::*;
use i18n::*;

#[component]
pub fn PresidentialCandidatesNewPage(id: Option<i64>, lang: Language) -> Element {
    let mut ctrl = Controller::new(lang, id)?;
    let tr: PresidentialCandidatesNewTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }


        div { id: "presidential-candidate-form", class: "min-h-screen",
            "{tr.title} PAGE"
            div { class: "col items-start",
                input {
                    r#type: "text",
                    value: ctrl.name(),
                    placeholder: "Name",
                    name: "name",
                    oninput: move |evt| ctrl.name.set(evt.value()),
                }
                label {
                    "Crypto Stance"
                    Dropdown {
                        items: CryptoStance::variants(&lang),
                        selected: ctrl.selected_crypto_stance(),
                        onselect: move |stance| ctrl.set_crypto_stance(stance),
                    }
                }
                label {
                    "Party"
                    Dropdown {
                        items: Party::variants(&lang),
                        selected: ctrl.selected_party(),
                        onselect: move |party| ctrl.set_party(party),
                    }
                }

                for (i , content) in ctrl.election_pledges().into_iter().enumerate() {
                    RichText {
                        id: "election-pledge-{i}",
                        content,
                        onchange: move |text| {
                            ctrl.set_election_pledges(i, text);
                        },
                    }
                }
                button {
                    class: "w-full btn-secondary",
                    onclick: move |_| ctrl.add_election_pledge(),
                    {tr.btn_add_election_pledge}
                }

                button {
                    onclick: move |_| async move {
                        ctrl.submit().await;
                    },
                    {tr.btn_submit}
                }
            }
        }
    }
}
