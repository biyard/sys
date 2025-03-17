use crate::Result;

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v1/repositories", table = repositories)]
pub struct Commit {
    #[api_model(primary_key, read_action = find_by_id)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(action = create, action_by_id = update, query_action = search_by)]
    pub name: String,
    
    #[api_model(unique)]
    pub url: String,
}
