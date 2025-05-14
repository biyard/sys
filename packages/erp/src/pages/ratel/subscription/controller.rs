use bdk::prelude::{by_types::QueryResponse, use_resource, use_signal, DioxusController, Language, RenderError, Resource, Signal};
use common::ratel::{ERPSubscribe, ERPSubscribeQuery, ERPSubscribeSummary};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    // pub subscriptions: Resource<Vec<ERPSubscribeSummary>>,
    pub subscriptions: Resource<QueryResponse<ERPSubscribeSummary>>,
    pub page: Signal<usize>,
    pub page_size: usize,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);
        let page_size = 10;

        let subscriptions = use_resource(move || {
            let page = page();

            async move {
                ERPSubscribe::get_client(crate::config::get().main_api_endpoint)
                    .query(ERPSubscribeQuery::new(page_size).with_page(page))
                    .await
                    .unwrap_or_default()
            }
        });

        let ctrl = Self {
            lang,
            subscriptions,
            page,
            page_size,
        };

        Ok(ctrl)
    }
}
