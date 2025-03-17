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
            div { class: "flex flex-row justify-between items-center",
                div { class: "flex flex-row gap-10",
                    for stance in CryptoStance::VARIANTS {
                        button {
                            class: "px-20 py-10",
                            background: if Some(*stance) == ctrl.stance() { "var(--color-primary)" },
                            onclick: move |_| ctrl.handle_select_stance(*stance),
                            {stance.translate(&lang)}
                        }
                    }
                }
                button {
                    class: "px-20 py-10 cursor-pointer",
                    background: if ctrl.stance().is_none() { "var(--color-primary)" },
                    onclick: move |_| {
                        ctrl.politicians.restart();
                    },
                    {tr.btn_refresh}
                }
            }

            div { class: "flex flex-col w-full",
                div { class: "w-full grid grid-cols-3 gap-10 border-b py-5",
                    span { class: "w-full col-span-1 flex items-center justify-center",
                        {tr.th_name}
                    }
                    span { class: "w-full col-span-1 flex items-center justify-center",
                        {tr.th_party}
                    }
                    span { class: "w-full col-span-1 flex items-center justify-center",
                        {tr.th_stance}
                    }
                }
                div {
                    class: "w-full flex flex-col",
                    onmousedown: move |e| ctrl.handle_mousedown(e),
                    onmouseup: move |_| ctrl.is_dragging.set(false),
                    oncontextmenu: move |e| ctrl.handle_contextmenu(e),
                    onmouseleave: move |_| ctrl.is_dragging.set(false),
                    for p in ctrl.politicians()? {
                        div {
                            class: "w-full grid grid-cols-3 gap-10 border-b py-10 cursor-pointer",
                            background: if ctrl.is_selected(&p.id) { "var(--color-primary)" },
                            onmousedown: move |e| ctrl.click_politician(p.id, e),
                            onmouseover: move |_| ctrl.handle_mouseover(p.id),
                            span { class: "w-full col-span-1 flex items-center justify-center",
                                "{p.name}"
                            }
                            span { class: "w-full col-span-1 flex items-center justify-center",
                                "{p.party}"
                            }
                            span { class: "w-full col-span-1 flex items-center justify-center",
                                {p.stance.translate(&lang)}
                            }
                        }
                    } // for

                    if ctrl.context_menu() {
                        div {
                            class: "fixed bg-white border shadow-md p-2 z-50 text-black",
                            style: "left: {ctrl.mouse_pos().0}px; top: {ctrl.mouse_pos().1}px;",
                            for stance in CryptoStance::VARIANTS {
                                div {
                                    class: "p-2 cursor-pointer hover:bg-gray-200",
                                    onclick: move |_| async move {
                                        tracing::debug!("Clicked context menu {:?}", stance);
                                        ctrl.handle_set_stance(*stance).await;
                                    },
                                    {stance.translate(&lang)}
                                }
                            }
                        }
                    } // if

                }
            }
        }
    }
}
