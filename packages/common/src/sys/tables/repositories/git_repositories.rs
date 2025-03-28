use bdk::prelude::*;

#[api_model(base = "/v1/repositories", table = git_repositories)]
pub struct GitRepository {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(action = create, action_by_id = update, query_action = search_by)]
    pub name: String,
    
    #[api_model(unique, action = create, action_by_id = update, query_action = search_by)]
    pub url: String,

    #[api_model(action = create, action_by_id = update, query_action = search_by)]
    pub owner: String,

    #[api_model(action = create, action_by_id = update, query_action = search_by)]
    pub user_id: i64,
}
