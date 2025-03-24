use bdk::prelude::*;
use by_axum::{
    aide,
    auth::{Authorization, generate_jwt},
    axum::{Extension, Json, extract::State, routing::post},
};
use by_types::{Claims, JsonWithHeaders, Role};
use common::*;
use dto::*;
use reqwest::Client;

use crate::config;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct UserPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct UserController {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct GoogleUserInfo {
    email: String,
}
const GOOGLE_USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/userinfo";

impl UserController {
    async fn sign_in(
        &self,
        _auth: Option<Authorization>,
        UserSignInRequest { id_token }: UserSignInRequest,
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

        let user = JsonWithHeaders::new(User {
            email: response.email.clone(),
            role: Role::User,
        });
        let mut claim = Claims::new(response.email, Role::User);

        let token = generate_jwt(&mut claim).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            Error::Unauthorized
        })?;

        Ok(user.with_bearer_token(&token))
    }
}

impl UserController {
    pub fn new() -> Self {
        Self {}
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
            UserAction::SignIn(param) => ctrl.sign_in(auth, param).await?,
        };

        Ok(res)
    }
}
