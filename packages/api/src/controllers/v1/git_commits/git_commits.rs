use bdk::prelude::*;
use by_axum::{
    aide,
    axum::{self,Extension, Json, extract::State, routing::{get, post}},
};

use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::config;

#[derive(Debug, Clone)]
pub struct GitCommitController {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommit {
    pub sha: String,
    pub commit: CommitDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetail {
    pub message: String,
    pub author: CommitAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub date: String,
}

impl GitCommitController {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn route(&self) -> create::Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/commits/:owner/:repo", post(Self::fetch_commits))
            .with_state(self.clone()))
    }

    pub async fn fetch_commits(
        State(ctrl): State<GitCommitController>,
        axum::extract::Path((owner, repo)): axum::extract::Path<(String, String)>,
    ) -> Result<Json<Vec<GitCommit>>, String> {
        tracing::debug!("fetching gitHub commits of {:?} of repo: {:?}", owner, repo);
        if !config::get().github_api_base_url == "https://api.github.com" {
            let message = "GitHub API was not defined!";
            tracing::error!(message);
            return Err(message.to_string());
        }

        let github_api_base_url = config::get().github_api_base_url;
        let url = format!("{}/repos/{}/{}/commits", github_api_base_url, owner, repo);

        let response = ctrl
            .client
            .get(&url)
            .header("User-Agent", "Rust-Axum-App")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch commits: {}", e))?
            .json::<Vec<GitCommit>>()
            .await
            .map_err(|_| "Invalid response format")?;

        Ok(Json(response))
    }


    pub async fn fetch_user_commits(
        State(ctrl): State<GitCommitController>,
        axum::extract::Path((owner, repo)): axum::extract::Path<(String, String)>,
    ) -> Result<Json<Vec<GitCommit>>, String> {
        tracing::debug!("fetching gitHub commits of {:?} of repo: {:?}", owner, repo);
        if !config::get().github_api_base_url == "https://api.github.com" {
            let message = "GitHub API was not defined!";
            tracing::error!(message);
            return Err(message.to_string());
        }

        let github_api_base_url = config::get().github_api_base_url;
        let url = format!("{}/repos/{}/{}/commits", github_api_base_url, owner, repo);

        let response = ctrl
            .client
            .get(&url)
            .header("User-Agent", "Rust-Axum-App")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch commits: {}", e))?
            .json::<Vec<GitCommit>>()
            .await
            .map_err(|_| "Invalid response format")?;

        Ok(Json(response))
    }
}
