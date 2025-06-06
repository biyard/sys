use std::iter::zip;

use bdk::prelude::*;
use common::ratel::*;

use crate::route::Route;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub lang: Language,

    pub id: Option<i64>,
    pub name: Signal<String>,
    pub crypto_stance: Memo<CryptoStance>,
    pub party: Memo<Party>,
    pub election_pledges: Signal<Vec<String>>,
    pub election_pledges_ids: Signal<Vec<i64>>,
    pub image: Signal<String>,

    pub selected_crypto_stance: Signal<usize>,
    pub selected_party: Signal<usize>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language, id: Option<i64>) -> std::result::Result<Self, RenderError> {
        let selected_party = use_signal(|| 0);
        let selected_crypto_stance = use_signal(|| 0);

        let mut ctrl = Self {
            id,
            lang,
            name: use_signal(|| String::new()),
            image: use_signal(|| String::new()),
            crypto_stance: use_memo(move || {
                let stance = selected_crypto_stance();
                CryptoStance::VARIANTS[stance]
            }),
            party: use_memo(move || {
                let party = selected_party();
                Party::VARIANTS[party]
            }),
            election_pledges: use_signal(|| vec![]),
            election_pledges_ids: use_signal(|| vec![]),
            selected_crypto_stance,
            selected_party,
            nav: use_navigator(),
        };

        use_effect(move || {
            tracing::debug!("Loading presidential candidate data {id:?}");
            if let Some(id) = id {
                spawn(async move {
                    let res =
                        PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                            .get(id)
                            .await
                            .unwrap_or_default();
                    ctrl.name.set(res.name);
                    ctrl.selected_crypto_stance.set(
                        CryptoStance::VARIANTS
                            .iter()
                            .position(|x| *x == res.crypto_stance)
                            .unwrap(),
                    );
                    ctrl.selected_party.set(
                        Party::VARIANTS
                            .iter()
                            .position(|x| *x == res.party)
                            .unwrap(),
                    );
                    ctrl.election_pledges_ids
                        .set(res.election_pledges.iter().map(|x| x.id).collect());
                    ctrl.election_pledges.set(
                        res.election_pledges
                            .into_iter()
                            .map(|x| x.promise)
                            .collect(),
                    );
                });
            }
        });

        Ok(ctrl)
    }

    pub fn set_election_pledges(&mut self, i: usize, pledges: String) {
        self.election_pledges.with_mut(|election_pledges| {
            if i < election_pledges.len() {
                election_pledges[i] = pledges;
            } else {
                election_pledges.push(pledges);
            }
        });
    }

    pub fn add_election_pledge(&mut self) {
        self.election_pledges.with_mut(|election_pledges| {
            election_pledges.push(String::new());
        });
    }

    pub async fn submit(&self) {
        if let Some(id) = self.id {
            PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                .update(
                    id,
                    self.name(),
                    self.image(),
                    self.crypto_stance(),
                    self.party(),
                    zip(self.election_pledges_ids(), self.election_pledges())
                        .map(|(id, promise)| ElectionPledgeUpdateRequest { id, promise })
                        .collect(),
                )
                .await
                .unwrap_or_default();
        } else {
            PresidentialCandidate::get_client(crate::config::get().main_api_endpoint)
                .create(
                    self.name(),
                    self.image(),
                    self.crypto_stance(),
                    self.party(),
                    self.election_pledges(),
                )
                .await
                .unwrap_or_default();
        }

        self.nav
            .push(Route::PresidentialCandidatesPage { lang: self.lang });
    }
}
