use bdk::prelude::*;
use common::ratel::{AssemblyMember, AssemblyMemberQuery, AssemblyMemberSummary, CryptoStance};

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub stance: Signal<Option<CryptoStance>>,
    pub politicians: Resource<Vec<AssemblyMemberSummary>>,
    #[allow(dead_code)]
    pub selected: Signal<Vec<i64>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let conf = config::get();
        let stance = use_signal(|| None);
        let selected = use_signal(|| vec![]);

        let politicians = use_server_future(move || {
            let stance = stance();
            let size = 10000;
            async move {
                let cli = AssemblyMember::get_client(conf.ratel_api_endpoint);
                let res = match stance {
                    Some(stance) => cli.list_by_stance(size, None, stance).await,
                    None => cli.query(AssemblyMemberQuery::new(size)).await,
                };

                res.unwrap_or_default().items
            }
        })?;

        let ctrl = Self {
            lang,
            stance,
            politicians,
            selected,
        };

        Ok(ctrl)
    }

    pub fn click_politician(&mut self, id: i64) {
        if self.selected().contains(&id) {
            self.selected.retain(|&x| x != id);
        } else {
            self.selected.push(id);
        }
    }

    pub fn is_selected(&self, id: i64) -> bool {
        self.selected().contains(&id)
    }
}
