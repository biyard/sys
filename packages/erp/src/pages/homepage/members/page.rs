use crate::route::Route;

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn MembersPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: MembersTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "members", class: "col gap-30",
            div { class: "row justify-between",
                "{tr.title} PAGE"
                Link { class: "btn-primary", to: Route::MembersNewPage { lang }, {tr.btn_new} }
            }

            for m in ctrl.members()? {
                div { class: "row group",
                    img { class: "h-300", src: m.image }
                    div { class: "col items-start",

                        h2 { class: "text-2xl font-bold row justify-between w-full",
                            {m.name}
                            button {
                                class: "text-red btn-secondary text-sm py-10 px-15",
                                onclick: move |_| async move {
                                    ctrl.delete(m.id).await;
                                },
                                "Delete"
                            }

                        }
                        p { {m.role.translate(&lang)} }
                        p { {m.email} }
                        if let Some(web) = m.web {
                            p { {web} }
                        }

                        if let Some(linkedin) = m.linkedin {
                            p { {linkedin} }
                        }

                        if let Some(github) = m.github {
                            p { {github} }
                        }

                        p { {m.description} }
                    }
                }
            }

        } // end of this page
    }
}
