use crate::config;
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::{Authorization, generate_jwt},
    axum::{Extension, Json, extract::State, routing::post},
};
use by_types::{Claims, JsonWithHeaders, Role};
use common::Result;
use common::sys::tables::*;
use common::*;
use reqwest::Client;
#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UserController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: UserRepository,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct GoogleUserInfo {
    email: String,
}
const GOOGLE_USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

impl UserController {
    async fn sign_in(
        &self,
        _auth: Option<Authorization>,
        UserLoginRequest { id_token }: UserLoginRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let client = Client::new();
        let response = client
            .get(GOOGLE_USERINFO_ENDPOINT)
            .bearer_auth(&id_token)
            .send()
            .await?
            .json::<GoogleUserInfo>()
            .await?;

        let domain = response.email.split('@').last().unwrap_or_default();

        if domain != "biyard.co" && !config::get().allowed_emails.contains(&domain) {
            tracing::error!("Unauthorized access: {:?}", response);
            return Err(Error::Unauthorized);
        }

        let user = match User::query_builder()
            .email_equals(response.email.clone())
            .query()
            .map(User::from)
            .fetch_optional(&self.pool)
            .await?
        {
            Some(user) =>  user,
            None => self.repo.insert(Role::User, response.email, None).await?,
        };

        let mut claim = Claims::new(user.email.clone(), user.role.clone());
        let user = JsonWithHeaders::new(user);

        let token = generate_jwt(&mut claim).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            Error::Unauthorized
        })?;

        Ok(user.with_bearer_token(&token))
    }
}

impl UserController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = User::get_repository(pool.clone());
        Self { pool, repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_user))
            .with_state(self.clone()))
    }

    pub async fn act_user(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act_user {:?}", body);
        let res = match body {
            UserAction::Login(param) => ctrl.sign_in(auth, param).await?,
            _ => unimplemented!(),
        };

        Ok(res)
    }
}
