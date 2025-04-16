use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn MembersPage(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: MembersTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "members", class: "col gap-30",
            "{tr.title} PAGE"

            for m in ctrl.members()? {
                div { class: "row group",
                    img { src: m.image }
                    div { class: "col items-start",
                        h2 { class: "text-2xl font-bold", {m.name} }
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

                        p { class: "hidden group-hover:block", {m.description} }
                    }
                }
            }

        } // end of this page
    }
}
