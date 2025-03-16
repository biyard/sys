use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use common::*;
use ratel::{
    PoliticianStances, PoliticianStancesAction, PoliticianStancesChangeStancesRequest,
    PoliticianStancesRepository, PoliticianStancesRepositoryUpdateRequest,
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
}
