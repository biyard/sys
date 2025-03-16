use bdk::prelude::*;
use dioxus_oauth::prelude::FirebaseService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub firebase: FirebaseService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            firebase: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn handle_google(&mut self) {
        let cred = self.firebase.sign_in_with_popup(vec![]).await;

        tracing::debug!("cred: {:?}", cred);
    }
}
