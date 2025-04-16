use crate::components::Dropdown;

use super::*;
use bdk::prelude::*;
use common::homepage::MemberRole;
use controller::*;
use i18n::*;

#[component]
pub fn MembersNewPage(lang: Language, id: Option<i64>) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: MembersNewTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "members-new",
            "{tr.title} PAGE"
            div { class: "col items-start",
                input {
                    r#type: "text",
                    placeholder: "Name",
                    name: "name",
                    oninput: move |evt| ctrl.name.set(evt.value()),
                }
                input {
                    r#type: "text",
                    placeholder: "Image",
                    name: "image",
                    oninput: move |evt| ctrl.image.set(evt.value()),
                }
                label {
                    "Member Role"
                    Dropdown {
                        items: MemberRole::variants(&lang),
                        selected: ctrl.selected_role(),
                        onselect: move |need| ctrl.set_member_role(need),
                    }
                }
                input {
                    r#type: "text",
                    placeholder: "Email",
                    name: "email",
                    oninput: move |evt| ctrl.email.set(evt.value()),
                }
                input {
                    r#type: "text",
                    placeholder: "Web",
                    name: "web",
                    oninput: move |evt| ctrl.web.set(evt.value()),
                }
                input {
                    r#type: "text",
                    placeholder: "LinkedIn",
                    name: "linkedin",
                    oninput: move |evt| ctrl.linkedin.set(evt.value()),
                }
                input {
                    r#type: "text",
                    placeholder: "GitHub",
                    name: "github",
                    oninput: move |evt| ctrl.github.set(evt.value()),
                }
                textarea {
                    rows: 10,
                    placeholder: "Description",
                    name: "description",
                    oninput: move |evt| ctrl.description.set(evt.value()),
                }

                button {
                    onclick: move |_| async move {
                        ctrl.submit().await;
                    },
                    {tr.btn_submit}
                }
            }
        } // end of this page
    }
}
