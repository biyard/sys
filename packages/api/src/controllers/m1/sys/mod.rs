mod users;

use bdk::prelude::*;

use common::*;
use sqlx::postgres::PgPoolOptions;

use crate::config;
use sys::tables::*;

pub async fn route() -> Result<by_axum::axum::Router> {
    let conf = config::get();

    let pool = PgPoolOptions::new()
        .max_connections(conf.pool_size)
        .connect(conf.sys_database)
        .await?;
    let u = User::get_repository(pool.clone());
    let c = GitCommit::get_repository(pool.clone());
    let r = GitRepository::get_repository(pool.clone());

    u.create_this_table().await?;
    c.create_this_table().await?;
    r.create_this_table().await?;

    u.create_table().await?;
    c.create_table().await?;
    r.create_table().await?;

    Ok(by_axum::axum::Router::new().nest(
        "/user",
        users::UserController::new(pool).route()?,
    ))
}
