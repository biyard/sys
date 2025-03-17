mod ratel;

use bdk::prelude::*;

use by_axum::{
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
    },
};
use by_types::Role;
use common::*;
use reqwest::StatusCode;

pub async fn route() -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest("/ratel", ratel::route().await?)
        .layer(middleware::from_fn(authorize_organization)))
}

pub async fn authorize_organization(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    let auth = req.extensions().get::<Option<Authorization>>();
    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.unwrap();

    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.clone().unwrap();

    match auth {
        Authorization::Bearer { claims } => {
            if claims.role != Role::User {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Authorization::ServerKey => {
            tracing::debug!("Authorization header is ServerKey");
        }
        _ => {
            tracing::debug!("Authorization header is not Bearer");
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    return Ok(next.run(req).await);
}
