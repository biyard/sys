use bdk::prelude::*;

use common::homepage::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub updates: Resource<by_types::QueryResponse<UpdateSummary>>,
    pub page: Signal<usize>,
    pub page_size: usize,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);
        let page_size = 10;
        let updates = use_server_future(move || {
            let page = page();

            async move {
                Update::get_client(crate::config::get().main_api_endpoint)
                    .query(UpdateQuery::new(page_size).with_page(page))
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,
            updates,
            page,
            page_size,
        };

        Ok(ctrl)
    }
}
