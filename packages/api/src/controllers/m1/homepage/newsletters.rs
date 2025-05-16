use bdk::prelude::*;
use by_axum::{
    axum::{
        Json,
        extract::State,
        routing::post,
    },
};
use common::*;
use common::homepage::tables::{Newsletter, NewsletterRepository, NewsletterAction};

#[derive(Clone, Debug)]
pub struct NewsletterController {
    repo: NewsletterRepository,
}

impl NewsletterController {
    async fn subscribe(&self, email: String) -> Result<Json<Newsletter>> {
        let res = self.repo.insert(email, true).await?;
        Ok(Json(res))
    }
}

impl NewsletterController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Newsletter::get_repository(pool.clone());

        Self { repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_newsletter))
            .with_state(self.clone()))
    }

    pub async fn act_newsletter(
        State(ctrl): State<NewsletterController>,
        Json(body): Json<NewsletterAction>
    ) -> Result<Json<Newsletter>> {
        // tracing::debug!("Susbcriber's email {:?}", body.email.clone());
        
        match body {
            NewsletterAction::Create(_) => Ok(Json(Newsletter::default())),
            NewsletterAction::Subscribe(v) => ctrl.subscribe(v.email).await,
        }
    }
}