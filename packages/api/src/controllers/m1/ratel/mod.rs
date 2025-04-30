mod politicians;
mod presidential_candidates;

use bdk::prelude::*;

use common::*;
use sqlx::postgres::PgPoolOptions;

use crate::config;

pub async fn route() -> Result<by_axum::axum::Router> {
    let conf = config::get();

    let pool = PgPoolOptions::new()
        .max_connections(conf.pool_size)
        .connect(conf.ratel_database)
        .await?;

    Ok(by_axum::axum::Router::new()
        .nest(
            "/presidential-candidates",
            presidential_candidates::PresidentialCandidateController::new(pool.clone()).route()?,
        )
        .nest(
            "/politicians",
            politicians::PoliticianController::new(pool).route()?,
        ))
}
