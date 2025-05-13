use crate::{components::Pagination, route::Route};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn SubscriptionPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: SubscriptionTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            id: "subscriptions",
            class: "flex flex-col px-6 py-4 bg-black min-h-screen text-white space-y-4",

            h1 {
                class: "text-2xl font-semibold text-center mb-4",
                "{tr.title}"
            }

            div {
                class: "w-full overflow-x-auto",

                table {
                    class: "w-full text-left border-collapse",

                    thead {
                        class: "bg-gray-900",
                        tr {
                            th { class: "px-4 py-2 border-b border-gray-700", "Email" }
                            th { class: "px-4 py-2 border-b border-gray-700", "Subscribed At" }
                        }
                    }

                    tbody {
                        for sub in ctrl.subscriptions()?.items {
                            tr {
                                class: "hover:bg-gray-800 transition-all duration-300",
                                td {
                                    class: "px-4 py-2 border-b border-gray-700",
                                    "{sub.email}"
                                }
                                td {
                                    class: "px-4 py-2 border-b border-gray-700",
                                    "{sub.created_at}"
                                }
                            }
                        }
                    }
                }
            }

            Pagination {
                total_count: ctrl.subscriptions()?.total_count,
                page_size: ctrl.page_size,
                onpage: move |page| ctrl.page.set(page),
            }
        }
    }
}
