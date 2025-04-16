use crate::{components::Pagination, route::Route};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ContactsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: ContactsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "contacts", class: "cols",
            "{tr.title} PAGE"

            for c in ctrl.contacts()?.items {
                Link {
                    class: "row hover:bg-gray-800/70",
                    to: Route::ContactsByIdPage {
                        lang,
                        id: c.id,
                    },
                    div { class: "col",
                        p { {c.company_name} }
                        p { {c.needs.translate(&lang)} }
                    }
                    p { {c.help} }
                }
            }


            Pagination {
                total_count: ctrl.contacts()?.total_count,
                page_size: ctrl.page_size,
                onpage: move |page| ctrl.page.set(page),
            }
        } // end of this page
    }
}
