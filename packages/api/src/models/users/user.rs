use crate::Result;

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v1/users", table = users)]
pub struct User {
    #[api_model(primary_key, read_action = find_by_id)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(unique)]
    #[validate(email)]
    pub email: String,
    #[api_model()]
    #[validate(custom(function = "validate_hex"))]
    pub password: String,
}
