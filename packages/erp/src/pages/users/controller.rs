use bdk::prelude::*;
use dioxus_oauth::prelude::FirebaseService;

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub firebase: FirebaseService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let firebase = &config::get().firebase;
        let firebase = FirebaseService::new(
            firebase.api_key.clone(),
            firebase.auth_domain.clone(),
            firebase.project_id.clone(),
            firebase.storage_bucket.clone(),
            firebase.messaging_sender_id.clone(),
            firebase.app_id.clone(),
            firebase.measurement_id.clone(),
        );

        let ctrl = Self { lang, firebase };

        Ok(ctrl)
    }

    pub async fn handle_google(&mut self) {
        let cred = self.firebase.sign_in_with_popup(vec![]).await;

        tracing::debug!("cred: {:?}", cred);
    }
}
