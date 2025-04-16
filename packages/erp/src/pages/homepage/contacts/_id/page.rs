use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ContactsByIdPage(id: i64, lang: Language) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: ContactsByIdTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "contacts-by-id", class: "col",
            "{tr.title} PAGE"

            if let Ok(contact) = ctrl.contact() {
                input { disabled: true, value: contact.first_name }

                input { disabled: true, value: contact.last_name }

                input { disabled: true, value: contact.email }

                input { disabled: true, value: contact.company_name }

                input { disabled: true, value: contact.needs.translate(&lang) }
                input { disabled: true, value: contact.help }
            }
        } // end of this page
    }
}
