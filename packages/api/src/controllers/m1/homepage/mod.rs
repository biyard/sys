mod contacts;
mod members;
mod news;
mod subscription;
mod updates;

use bdk::prelude::{sqlx::postgres::PgPoolOptions, *};

use crate::config;
use common::*;

pub async fn route() -> Result<by_axum::axum::Router> {
    let conf = config::get();

    let pool = PgPoolOptions::new()
        .max_connections(conf.pool_size)
        .connect(conf.homepage_database)
        .await?;

    Ok(by_axum::axum::Router::new()
        .nest(
            "/contacts",
            contacts::ContactController::new(pool.clone()).route()?,
        )
        .nest(
            "/members",
            members::MemberController::new(pool.clone()).route()?,
        )
        .nest(
            "/updates",
            updates::UpdateController::new(pool.clone()).route()?,
        )
        .nest("/news", news::NewsController::new(pool.clone()).route()?)
        .nest(
            "/subscriptions",
            subscription::SubscriptionController::new(pool).route()?)
        )
}
