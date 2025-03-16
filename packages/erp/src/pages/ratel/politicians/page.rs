#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use common::ratel::CryptoStance;
use controller::*;
use i18n::*;

#[component]
pub fn PoliticiansPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: PoliticiansTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "politicians", class: "w-full flex flex-col gap-20",
            div { class: "flex flex-row gap-10",
                for stance in CryptoStance::VARIANTS {
                    button {
                        class: "px-20 py-10",
                        background: if Some(*stance) == ctrl.stance() { "var(--color-primary)" },
                        onclick: move |_| {
                            ctrl.stance.set(Some(stance.clone()));
                        },
                        {stance.translate(&lang)}
                    }
                }
            }

            div { class: "flex flex-col w-full",
                div { class: "w-full grid grid-cols-3 gap-10 border-b py-5",
                    span { class: "w-full col-span-1 flex items-center justify-center",
                        {tr.th_name}
                    }
                    span { class: "w-full col-span-1  items-center justify-center", {tr.th_party} }
                    span { class: "w-full col-span-1  items-center justify-center", {tr.th_stance} }
                }

                div { class: "w-full flex flex-col",
                    for p in ctrl.politicians()? {
                        div {
                            class: "w-full grid grid-cols-3 gap-10 border-b py-10 cursor-pointer",
                            onclick: move |_| {
                                ctrl.click_politician(p.id);
                            },
                            background: if ctrl.is_selected(p.id) { "var(--color-primary)" },
                            span { class: "w-full col-span-1 flex items-center justify-center",
                                "{p.name}"
                            }
                            span { class: "w-full col-span-1  items-center justify-center",
                                "{p.party}"
                            }
                            span { class: "w-full col-span-1  items-center justify-center",
                                {p.stance.translate(&lang)}
                            }
                        }
                    }
                }
            }
        } // end of this page
    }
}
