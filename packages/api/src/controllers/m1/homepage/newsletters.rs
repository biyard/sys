use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Json,
        Extension,
        extract::{Query, State},
        routing::{post, get},
    },
};
use by_types::*;
use common::*;
use common::homepage::*;
use common::homepage::tables::{Newsletter, NewsletterRepository, NewsletterAction};
use sqlx::postgres::PgRow;

// #[derive(serde::Serialize, serde::Deserialize)]
// pub enum NewsletterGetResponse {
//     Query(QueryResponse<NewsletterSummary>),
//     Single(Newsletter),
//     List(Vec<Newsletter>),
// }
#[derive(Clone, Debug)]
pub struct NewsletterController {
    repo: NewsletterRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl NewsletterController {
    async fn subscribe(&self, email: String) -> Result<Json<Newsletter>> {
        let res = self.repo.insert(email, true).await?;
        Ok(Json(res))
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: NewsletterQuery,
    ) -> Result<QueryResponse<NewsletterSummary>> {
        let mut total_count = 0;
        let items: Vec<NewsletterSummary> = NewsletterSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl NewsletterController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Newsletter::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_newsletter))
            .with_state(self.clone())
            .route("/", get(Self::get_newsletters))
            .with_state(self.clone()))
    }
 
    pub async fn act_newsletter(
        State(ctrl): State<NewsletterController>,
        Json(body): Json<NewsletterAction>
    ) -> Result<Json<Newsletter>> {        
        match body {
            NewsletterAction::Create(_) => Ok(Json(Newsletter::default())),
            NewsletterAction::Subscribe(v) => ctrl.subscribe(v.email).await,
        }
    }

    pub async fn get_newsletters(
        State(ctrl): State<NewsletterController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<NewsletterParam>,
    ) -> Result<Json<NewsletterGetResponse>> {
        tracing::debug!("list_newsletter {:?}", q);

        match q {
            NewsletterParam::Query(param) => Ok(Json(NewsletterGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}