use bdk::prelude::*;
use common::ratel::{
    AssemblyMember, AssemblyMemberQuery, AssemblyMemberSummary, CryptoStance, PoliticianStances,
};
use dioxus_elements::input_data::MouseButton;

use crate::config;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub stance: Signal<Option<CryptoStance>>,
    pub politicians: Resource<Vec<AssemblyMemberSummary>>,
    pub selected: Signal<Vec<i64>>,
    pub is_dragging: Signal<bool>,
    pub mouse_pos: Signal<(f64, f64)>,
    pub context_menu: Signal<bool>,
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
                let cli = AssemblyMember::get_client(conf.main_api_endpoint);
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
            is_dragging: use_signal(|| false),
            mouse_pos: use_signal(|| (0.0, 0.0)),
            context_menu: use_signal(|| false),
        };

        Ok(ctrl)
    }

    pub fn handle_contextmenu(&mut self, e: MouseEvent) {
        e.prevent_default();
        self.context_menu.set(true);
        let rect = e.client_coordinates();
        self.mouse_pos.set((rect.x, rect.y));
    }

    pub fn click_politician(&mut self, id: i64, e: MouseEvent) {
        if e.trigger_button() == Some(MouseButton::Primary) {
            self.selected.with_mut(|s| {
                if let Some(pos) = s.iter().position(|&x| x == id) {
                    s.remove(pos);
                } else {
                    s.push(id);
                }
            });
        }
    }

    pub fn is_selected(&self, id: &i64) -> bool {
        self.selected().contains(id)
    }

    pub fn handle_mousedown(&mut self, e: MouseEvent) {
        if e.trigger_button() == Some(MouseButton::Primary) {
            self.is_dragging.set(true);
        }
    }

    pub async fn handle_set_stance(&mut self, stance: CryptoStance) {
        tracing::debug!("Setting stance to {:?}", stance);
        let endpoint = config::get().main_api_endpoint;

        let ids = self.selected().clone();

        match PoliticianStances::get_client(endpoint)
            .change_stances(ids, stance)
            .await
        {
            Ok(_) => {
                btracing::info!("Successfully changed stance to {:?}", stance);
                self.politicians.restart();
            }
            Err(e) => {
                btracing::error!("Failed to change stance: {:?}", e);
            }
        }
        self.context_menu.set(false);
        self.selected.clear();
    }

    pub fn handle_mouseover(&mut self, id: i64) {
        if self.is_dragging() {
            self.selected.with_mut(|s| {
                if !s.contains(&id) {
                    s.push(id);
                }
            });
        }
    }
}
