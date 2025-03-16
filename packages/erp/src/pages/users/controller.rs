use bdk::prelude::*;
use common::dto::User;
use dioxus_oauth::prelude::FirebaseService;

use crate::{config, route::Route};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,
    pub firebase: FirebaseService,
    pub nav: Navigator,
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

        let ctrl = Self {
            lang,
            firebase,
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub async fn handle_google(&mut self) {
        let cred = self.firebase.sign_in_with_popup(vec![]).await;

        let endpoint = config::get().ratel_api_endpoint;
        match User::get_client(endpoint).sign_in(cred.access_token).await {
            Ok(_) => {
                self.nav.push(Route::PoliticiansPage { lang: self.lang });
            }
            Err(e) => {
                tracing::error!("Failed to sign in: {:?}", e);
                return;
            }
        };
    }
}
