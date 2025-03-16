use bdk::prelude::*;
use common::ratel::{AssemblyMember, CryptoStance};

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };
        let conf = config::get();
        let stance = use_signal(|| CryptoStance::default());

        use_server_future(move || {
            let stance = stance();
            async move { AssemblyMember::get_client(conf.ratel_api_endpoint) }
        });

        Ok(ctrl)
    }
}
