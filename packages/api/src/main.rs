use bdk::prelude::*;
use by_axum::{
    auth::{authorization_middleware, set_auth_config},
    axum::middleware,
};
use common::*;
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

    let app = app
        .nest("/m1", controllers::m1::route().await?)
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
