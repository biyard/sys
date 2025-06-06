use crate::Result;

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v1/commits", table = git_commits)]
pub struct GitCommit {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique)]
    pub commit_hash: String,

    #[api_model(action = create, many_to_one = git_repositories)]
    pub repository_id: i64,

    #[api_model(action = create, many_to_one = users)]
    pub user_id: i64,
}
