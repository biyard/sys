use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use common::*;
use ratel::{
    AssemblyMemberQuery, AssemblyMemberQueryActionType, AssemblyMemberSummary, PoliticianStances,
    PoliticianStancesAction, PoliticianStancesChangeStancesRequest, PoliticianStancesRepository,
    PoliticianStancesRepositoryUpdateRequest,
};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PoliticianPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct PoliticianController {
    repo: PoliticianStancesRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PoliticianController {
    async fn change_stances(
        &self,
        auth: Option<Authorization>,
        PoliticianStancesChangeStancesRequest { ids, stance }: PoliticianStancesChangeStancesRequest,
    ) -> Result<PoliticianStances> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let mut tx = self.pool.begin().await?;

        for id in ids.clone() {
            self.repo
                .update_with_tx(
                    &mut *tx,
                    id,
                    PoliticianStancesRepositoryUpdateRequest::new().with_stance(stance),
                )
                .await?;
        }

        tx.commit().await?;

        Ok(PoliticianStances { ids, stance })
    }
}

impl PoliticianController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = PoliticianStances::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_politicians))
            .route("/stances", post(Self::act_politician))
            .with_state(self.clone()))
    }

    pub async fn act_politician(
        State(ctrl): State<PoliticianController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<PoliticianStancesAction>,
    ) -> Result<Json<PoliticianStances>> {
        tracing::debug!("act_politician {:?}", body);
        match body {
            PoliticianStancesAction::ChangeStances(param) => {
                let res = ctrl.change_stances(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_politicians(
        State(ctrl): State<PoliticianController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(body): Query<AssemblyMemberQuery>,
    ) -> Result<Json<QueryResponse<AssemblyMemberSummary>>> {
        tracing::debug!("act_politician {:?}", body);

        let items = match body {
            b if b.action == Some(AssemblyMemberQueryActionType::ListByStance) => {
                AssemblyMemberSummary::query_builder()
                    .stance_equals(b.stance.unwrap_or_default())
                    .query()
                    .map(AssemblyMemberSummary::from)
                    .fetch_all(&ctrl.pool)
                    .await?
            }
            _ => {
                AssemblyMemberSummary::query_builder()
                    .query()
                    .map(AssemblyMemberSummary::from)
                    .fetch_all(&ctrl.pool)
                    .await?
            }
        };

        Ok(Json(QueryResponse {
            total_count: items.len() as i64,
            items,
        }))
    }
}
