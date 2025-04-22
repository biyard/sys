use bdk::prelude::*;
use common::ratel::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub candidates: Resource<Vec<PresidentialCandidateSummary>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let candidates = use_server_future(move || async move {
            let res = PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                .query(PresidentialCandidateQuery::new(20))
                .await
                .unwrap_or_default();
            res.items
        })?;
        let ctrl = Self { lang, candidates };

        Ok(ctrl)
    }

    pub async fn handle_delete(&mut self, id: i64) {
        PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
            .delete(id)
            .await
            .unwrap_or_default();
        self.candidates.restart();
    }
}
