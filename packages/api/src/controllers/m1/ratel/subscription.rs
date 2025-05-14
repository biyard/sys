

use bdk::prelude::{*};
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::post,
    },
};
use by_types::QueryResponse;
use common::{error::Error, ratel::{ERPSubscribe, ERPSubscribeAction, ERPSubscribeCreateRequest, ERPSubscribeGetResponse, ERPSubscribeParam, ERPSubscribeQuery, ERPSubscribeRepository, ERPSubscribeSummary}};
use sqlx::postgres::PgRow;



#[derive(Clone, Debug)]
pub struct SubscriptionController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: ERPSubscribeRepository,
}
pub type Result<T> = std::result::Result<T, Error>;



impl SubscriptionController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: ERPSubscribeQuery,
    ) -> Result<QueryResponse<ERPSubscribeSummary>> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let mut total_count = 0;
        let items: Vec<ERPSubscribeSummary> = ERPSubscribeSummary::query_builder()
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

    async fn create(
        &self,
        _auth: Option<Authorization>,
        ERPSubscribeCreateRequest {
            email,
        }: ERPSubscribeCreateRequest,
    ) -> Result<ERPSubscribe> {
        self.repo
            .insert(email)
            .await
    }
}

impl SubscriptionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = ERPSubscribe::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
        .route("/", post(Self::act_subscriber).get(Self::get_subscribers))
        .with_state(self.clone()))
    }

    pub async fn act_subscriber(
        State(ctrl): State<SubscriptionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ERPSubscribeAction>,
    ) -> Result<Json<ERPSubscribe>> {
        tracing::debug!("act_subscriber {:?}", body);
        match body {
            ERPSubscribeAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_subscribers(
        State(ctrl): State<SubscriptionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<ERPSubscribeParam>,
    ) -> Result<Json<ERPSubscribeGetResponse>> {
        tracing::debug!("list_subscribers {:?}", q);

        match q {
            ERPSubscribeParam::Query(param) => Ok(Json(ERPSubscribeGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}
