use bdk::prelude::*;
use validator::Validate;

use crate::ratel::*;

#[derive(Validate)]
#[api_model(base = "/m1/ratel/presidential-candidates", table = presidential_candidates, action = [create(election_pledges = Vec<String>)], action_by_id = [delete, update(election_pledges = Vec<ElectionPledgeUpdateRequest>)])]
pub struct PresidentialCandidate {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub image: String,
    #[api_model(summary, type = INTEGER, action = create, action_by_id = update)]
    pub crypto_stance: CryptoStance,
    #[api_model(summary, type = INTEGER, action = create, action_by_id = update)]
    pub party: Party,

    #[api_model(summary, one_to_many = election_pledges, foreign_key = presidential_candidate_id)]
    pub election_pledges: Vec<ElectionPledge>,
}
