use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/m1/homepage/updates", table = update, action_by_id = delete)]
pub struct Update {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique)]
    #[validate(email)]
    pub email: String,
}
