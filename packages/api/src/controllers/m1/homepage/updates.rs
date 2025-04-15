use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use common::{homepage::*, *};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UpdatePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UpdateController {
    repo: UpdateRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl UpdateController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: UpdateQuery,
    ) -> Result<QueryResponse<UpdateSummary>> {
        let mut total_count = 0;
        let items: Vec<UpdateSummary> = UpdateSummary::query_builder()
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

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Update> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     UpdateReadAction { action, .. }: UpdateReadAction,
    // ) -> Result<Update> {
    //     todo!()
    // }
}

impl UpdateController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Update::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_update_by_id))
            .with_state(self.clone())
            .route("/", get(Self::get_update))
            .with_state(self.clone()))
    }

    // pub async fn act_update(
    //     State(ctrl): State<UpdateController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Json(body): Json<UpdateAction>,
    // ) -> Result<Json<Update>> {
    //     tracing::debug!("act_update {:?}", body);
    //     match body {
    //         UpdateAction::Create(param) => {
    //             let res = ctrl.create(auth, param).await?;
    //             Ok(Json(res))
    //         }
    //     }
    // }

    pub async fn act_update_by_id(
        State(ctrl): State<UpdateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(UpdatePath { id }): Path<UpdatePath>,
        Json(body): Json<UpdateByIdAction>,
    ) -> Result<Json<Update>> {
        tracing::debug!("act_update_by_id {:?} {:?}", id, body);
        match body {
            UpdateByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    // pub async fn get_update_by_id(
    //     State(ctrl): State<UpdateController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(UpdatePath { id }): Path<UpdatePath>,
    // ) -> Result<Json<Update>> {
    //     tracing::debug!("get_update {:?}", id);

    //     Ok(Json(
    //         Update::query_builder()
    //             .id_equals(id)
    //             .query()
    //             .map(Update::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_update(
        State(ctrl): State<UpdateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<UpdateParam>,
    ) -> Result<Json<UpdateGetResponse>> {
        tracing::debug!("list_update {:?}", q);

        match q {
            UpdateParam::Query(param) => Ok(Json(UpdateGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // UpdateParam::Read(param)
            //     if param.action == Some(UpdateReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(UpdateGetResponse::Read(res)))
            // }
        }
    }
}
