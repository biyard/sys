use crate::components::Pagination;

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn UpdatesPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: UpdatesTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "updates", class: "col",
            "{tr.title} PAGE"
            for u in ctrl.updates()?.items {
                div { class: "row", {u.email} }
            }

            Pagination {
                total_count: ctrl.updates()?.total_count,
                page_size: ctrl.page_size,
                onpage: move |page| ctrl.page.set(page),
            }
        } // end of this page
    }
}
