use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::post,
    },
};
use by_types::QueryResponse;
use common::{ratel::*, *};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PresidentialCandidatePath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct PresidentialCandidateController {
    repo: PresidentialCandidateRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PresidentialCandidateController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: PresidentialCandidateQuery,
    ) -> Result<QueryResponse<PresidentialCandidateSummary>> {
        let mut total_count = 0;
        let items: Vec<PresidentialCandidateSummary> =
            PresidentialCandidateSummary::query_builder()
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
        PresidentialCandidateCreateRequest {
            name,
            crypto_stance,
            party,
            election_pledges,
            image,
        }: PresidentialCandidateCreateRequest,
    ) -> Result<PresidentialCandidate> {
        let mut tx = self.pool.begin().await?;

        let mut doc = self
            .repo
            .insert_with_tx(&mut *tx, name, image, crypto_stance, party)
            .await?
            .ok_or(Error::PresidentialCandidateCreateError)?;

        let repo = ElectionPledge::get_repository(self.pool.clone());

        for promise in election_pledges {
            tracing::debug!("create election pledge {:?}", promise);
            let promise = repo
                .insert_with_tx(&mut *tx, doc.id, promise)
                .await?
                .ok_or(Error::ElectionPledgeCreateError)?;
            doc.election_pledges.push(promise);
        }
        tx.commit().await?;

        Ok(doc)
    }

    async fn update(
        &self,
        id: i64,
        _auth: Option<Authorization>,
        PresidentialCandidateUpdateRequest {
            name,
            crypto_stance,
            party,
            election_pledges,
            image,
        }: PresidentialCandidateUpdateRequest,
    ) -> Result<PresidentialCandidate> {
        let mut tx = self.pool.begin().await?;

        self.repo
            .update_with_tx(
                &mut *tx,
                id,
                PresidentialCandidateRepositoryUpdateRequest::new()
                    .with_name(name)
                    .with_image(image)
                    .with_crypto_stance(crypto_stance)
                    .with_party(party),
            )
            .await?
            .ok_or(Error::PresidentialCandidateUpdateError)?;

        let repo = ElectionPledge::get_repository(self.pool.clone());
        for ElectionPledgeUpdateRequest { id, promise } in election_pledges {
            repo.update_with_tx(
                &mut *tx,
                id,
                ElectionPledgeRepositoryUpdateRequest::new().with_promise(promise),
            )
            .await?
            .ok_or(Error::ElectionPledgeUpdateError)?;
        }
        tx.commit().await?;

        let res = PresidentialCandidate::query_builder()
            .id_equals(id)
            .query()
            .map(PresidentialCandidate::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<PresidentialCandidate> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     PresidentialCandidateReadAction { action, .. }: PresidentialCandidateReadAction,
    // ) -> Result<PresidentialCandidate> {
    //     todo!()
    // }
}

impl PresidentialCandidateController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = PresidentialCandidate::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                post(Self::act_presidential_candidate_by_id)
                    .get(Self::get_presidential_candidate_by_id),
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_presidential_candidate).get(Self::get_presidential_candidate),
            )
            .with_state(self.clone()))
    }

    pub async fn act_presidential_candidate(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<PresidentialCandidateAction>,
    ) -> Result<Json<PresidentialCandidate>> {
        tracing::debug!("act_presidential_candidate {:?}", body);
        match body {
            PresidentialCandidateAction::Create(param) => {
                tracing::debug!("create {:?}", param);
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_presidential_candidate_by_id(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(PresidentialCandidatePath { id }): Path<PresidentialCandidatePath>,
        Json(body): Json<PresidentialCandidateByIdAction>,
    ) -> Result<Json<PresidentialCandidate>> {
        tracing::debug!("act_presidential_candidate_by_id {:?} {:?}", id, body);
        match body {
            PresidentialCandidateByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            PresidentialCandidateByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_presidential_candidate_by_id(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(PresidentialCandidatePath { id }): Path<PresidentialCandidatePath>,
    ) -> Result<Json<PresidentialCandidate>> {
        tracing::debug!("get_presidential_candidate {:?}", id);

        Ok(Json(
            PresidentialCandidate::query_builder()
                .id_equals(id)
                .query()
                .map(PresidentialCandidate::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_presidential_candidate(
        State(ctrl): State<PresidentialCandidateController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<PresidentialCandidateParam>,
    ) -> Result<Json<PresidentialCandidateGetResponse>> {
        tracing::debug!("list_presidential_candidate {:?}", q);

        match q {
            PresidentialCandidateParam::Query(param) => Ok(Json(
                PresidentialCandidateGetResponse::Query(ctrl.query(auth, param).await?),
            )),
            // PresidentialCandidateParam::Read(param)
            //     if param.action == Some(PresidentialCandidateReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(PresidentialCandidateGetResponse::Read(res)))
            // }
        }
    }
}
