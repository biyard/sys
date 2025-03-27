use bdk::prelude::*;
use by_types::Role;

#[api_model(base = "/m1/users", table = users, action = [login(id_token = String)])]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(action = create, type = INTEGER)]
    pub role: Role,

    #[api_model(auto = [insert])]
    pub created_at: i64,

    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique, read_action = [get_user, find_by_email])]
    pub email: String,

    #[api_model(unique)]
    pub github_id: Option<String>,
}
