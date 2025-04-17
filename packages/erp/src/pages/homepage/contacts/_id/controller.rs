use bdk::prelude::*;
use common::homepage::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub contact: Resource<Contact>,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        let contact = use_resource(move || {
            let id = id;
            async move {
                Contact::get_client(crate::config::get().main_api_endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        });

        let ctrl = Self { lang, contact };

        Ok(ctrl)
    }
}
