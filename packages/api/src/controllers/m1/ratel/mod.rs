mod politicians;

use bdk::prelude::*;

use by_types::DatabaseConfig;
use common::*;
use sqlx::postgres::PgPoolOptions;

use crate::config;

pub async fn route() -> Result<by_axum::axum::Router> {
    let conf = config::get();

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.ratel_database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    Ok(by_axum::axum::Router::new().nest(
        "/politicians",
        politicians::PoliticianController::new(pool).route()?,
    ))
}
