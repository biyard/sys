use crate::{components::Pagination, route::Route};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn NewsPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: NewsTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "news", class: "flex flex-col gap-20",
            "{tr.title} PAGE"
            div { class: "w-full flex flex-row justify-end",
                Link { class: "btn-primary", to: Route::NewsNewPage { lang }, {tr.btn_new} }
            }

            for news in ctrl.news()?.items {
                div { class: "flex flex-row gap-10",
                    img { class: "h-150 object-cover", src: news.image }
                    div { class: "flex flex-col gap-4",
                        label { class: "text-sm", {news.category} }
                        h2 { class: "text-2xl font-bold", {news.title} }
                        p { class: "text-lg", {news.contents} }
                    }
                }
            }

            Pagination {
                total_count: ctrl.news()?.total_count,
                page_size: ctrl.page_size,
                onpage: move |page| ctrl.page.set(page),
            }
        } // end of this page
    }
}
