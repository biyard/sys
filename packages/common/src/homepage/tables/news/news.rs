use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/m1/homepage/news", table = news, action_by_id = delete)]
pub struct News {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create)]
    pub category: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    pub image: String,
    #[api_model(summary, action = create)]
    #[validate(length(max = 350))]
    pub contents: String,
    #[api_model(summary, version = v0.1, action = create)]
    pub link: String,
    #[api_model(summary, action = create)]
    pub main: bool,
}
