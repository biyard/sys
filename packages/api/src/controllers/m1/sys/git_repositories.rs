#[allow(unused_imports)]
use bdk::prelude::{by_axum::ApiError, *};

use by_axum::{
    // aide,
    axum::{
        // Extension, 
        Json, 
        extract::State, 
        routing::{
            // get, 
            post
        }},
};

use common::sys::tables::*;
use common::*;

#[derive(Debug, Clone)]
pub struct GitRepositoryController {
    // pool: sqlx::Pool<sqlx::Postgres>,
    repo: GitRepositoryRepository,
}

impl GitRepositoryController {
    pub async fn create(
        &self,
        GitRepositoryCreateRequest { url, name, owner, user_id }: GitRepositoryCreateRequest,
    ) -> Result<Json<GitRepository>> {
        let new_git_repo = self.repo.insert(name, url, owner, user_id).await?;
        Ok(Json(new_git_repo))
    }
}

impl GitRepositoryController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = GitRepository::get_repository(pool.clone());
        Self { 
            // pool, 
            repo }
    }

    pub fn route(&self) -> common::Result<by_axum::axum::Router> {
           Ok(by_axum::axum::Router::new()
            .route("/", post(Self::create_new_git_repository))
            .with_state(self.clone()))
    }

    pub async fn create_new_git_repository(
        State(ctrl): State<GitRepositoryController>,
        Json(body): Json<GitRepositoryAction>,
    ) -> Result<Json<GitRepository>> {
        tracing::debug!("create new git repository with these request body {:?}", body);
        let res = match body {
            GitRepositoryAction::Create(request_body) => ctrl.create(request_body).await?,
            // _ => unimplemented!(),
        };

        Ok(res)
    }
}
