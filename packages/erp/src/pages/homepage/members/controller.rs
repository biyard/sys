use bdk::prelude::*;

use common::homepage::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub members: Resource<Vec<MemberSummary>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let members = use_server_future(move || async move {
            Member::get_client(crate::config::get().main_api_endpoint)
                .query(MemberQuery::new(20).with_page(1))
                .await
                .unwrap_or_default()
                .items
        })?;
        let ctrl = Self { lang, members };

        Ok(ctrl)
    }

    pub async fn delete(&mut self, id: i64) {
        match Member::get_client(crate::config::get().main_api_endpoint)
            .delete(id)
            .await
        {
            Ok(_) => {
                self.members.restart();
            }
            Err(e) => {
                btracing::error!("Error deleting member: {:?}", e);
            }
        }
    }
}
