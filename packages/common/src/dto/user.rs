use bdk::prelude::*;
use by_types::Role;

#[api_model(base = "/v1/users", database = skip, action = [sign_in(id_token = String)])]
pub struct User {
    pub email: String,
    pub role: Role,
}
