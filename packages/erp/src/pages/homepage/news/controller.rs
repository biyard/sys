use bdk::prelude::{by_types::QueryResponse, *};
use common::homepage::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub news: Resource<QueryResponse<NewsSummary>>,
    pub page: Signal<usize>,
    pub page_size: usize,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);
        let page_size = 10;
        let news = use_resource(move || {
            let page = page();
            async move {
                News::get_client(crate::config::get().main_api_endpoint)
                    .query(NewsQuery::new(page_size).with_page(page))
                    .await
                    .unwrap_or_default()
            }
        });

        let ctrl = Self {
            lang,
            news,
            page,
            page_size,
        };

        Ok(ctrl)
    }
}
