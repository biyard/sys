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
pub struct MemberPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct MemberController {
    repo: MemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl MemberController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: MemberQuery,
    ) -> Result<QueryResponse<MemberSummary>> {
        let mut total_count = 0;
        let items: Vec<MemberSummary> = MemberSummary::query_builder()
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
        MemberCreateRequest {
            name,
            image,
            role,
            email,
            web,
            linkedin,
            github,
            description,
        }: MemberCreateRequest,
    ) -> Result<Member> {
        self.repo
            .insert(name, image, role, email, web, linkedin, github, description)
            .await
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: MemberUpdateRequest,
    ) -> Result<Member> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Member> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     MemberReadAction { action, .. }: MemberReadAction,
    // ) -> Result<Member> {
    //     todo!()
    // }
}

impl MemberController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Member::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_member_by_id).post(Self::act_member_by_id),
            )
            .with_state(self.clone())
            .route("/", post(Self::act_member).get(Self::get_member))
            .with_state(self.clone()))
    }

    pub async fn act_member(
        State(ctrl): State<MemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<MemberAction>,
    ) -> Result<Json<Member>> {
        tracing::debug!("act_member {:?}", body);
        match body {
            MemberAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_member_by_id(
        State(ctrl): State<MemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(MemberPath { id }): Path<MemberPath>,
        Json(body): Json<MemberByIdAction>,
    ) -> Result<Json<Member>> {
        tracing::debug!("act_member_by_id {:?} {:?}", id, body);
        match body {
            MemberByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            MemberByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_member_by_id(
        State(ctrl): State<MemberController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(MemberPath { id }): Path<MemberPath>,
    ) -> Result<Json<Member>> {
        tracing::debug!("get_member {:?}", id);

        Ok(Json(
            Member::query_builder()
                .id_equals(id)
                .query()
                .map(Member::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_member(
        State(ctrl): State<MemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<MemberParam>,
    ) -> Result<Json<MemberGetResponse>> {
        tracing::debug!("list_member {:?}", q);

        match q {
            MemberParam::Query(param) => Ok(Json(MemberGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // MemberParam::Read(param)
            //     if param.action == Some(MemberReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(MemberGetResponse::Read(res)))
            // }
        }
    }
}
