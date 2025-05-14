use bdk::prelude::*;
use validator::Validate;


#[derive(Validate)]
#[api_model(base = "/m1/ratel/subscription", table = subscribe)]
pub struct ERPSubscribe {
    #[api_model(auto = [insert])]
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, unique, action = create, action_by_id = update)]
    #[validate(email)]
    pub email: String,
}
