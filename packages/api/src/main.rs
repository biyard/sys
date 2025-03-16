use bdk::prelude::*;
use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::middleware,
};
use by_types::DatabaseConfig;
use common::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers {
    pub mod m1;
    pub mod v1;
}

pub mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);
    set_auth_config(conf.auth.clone());

    let _pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let app = app
        .nest(
            "/v1/users",
            controllers::v1::users::UserController::new().route()?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
