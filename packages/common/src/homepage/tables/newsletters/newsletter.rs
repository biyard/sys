use bdk::prelude::*;

#[api_model(base = "/m1/homepage/newsletters", table = newsletter_subscribers, action_by_id = delete)]
pub struct Newsletter {
    #[api_model(summary, primary_key)]
    pub id: i64,

    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[validate(email)]
    #[api_model(summary, unique, action = [create, subscribe], action_by_id = update)]
    pub email: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub is_subscribed: bool,
    
}