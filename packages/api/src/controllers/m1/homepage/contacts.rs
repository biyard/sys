use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use by_types::QueryResponse;
use common::{homepage::*, *};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ContactPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct ContactController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ContactController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: ContactQuery,
    ) -> Result<QueryResponse<ContactSummary>> {
        let mut total_count = 0;
        let items: Vec<ContactSummary> = ContactSummary::query_builder()
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

impl ContactController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_contact))
            .with_state(self.clone()))
    }

    pub async fn get_contact(
        State(ctrl): State<ContactController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<ContactParam>,
    ) -> Result<Json<ContactGetResponse>> {
        tracing::debug!("list_contact {:?}", q);

        match q {
            ContactParam::Query(param) => Ok(Json(ContactGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // ContactParam::Read(param)
            //     if param.action == Some(ContactReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(ContactGetResponse::Read(res)))
            // }
        }
    }
}
