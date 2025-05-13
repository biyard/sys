use bdk::prelude::*;
use validator::ValidationErrors;

#[derive(Debug, serde::Serialize, PartialEq, Eq, serde::Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Error {
    #[translate(
        en = "An unknown error occurred. Please try again later.",
        ko = "알 수 없는 에러가 발생했습니다. 잠시 후 다시 시도해주세요."
    )]
    Unknown(String),

    #[translate(
        en = "Only Biyard accounts or allowed accounts can log in.",
        ko = "바이야드 계정 또는 허용된 계정만 로그인할 수 있습니다."
    )]
    Unauthorized,

    #[translate(
        ko = "요청에 실패했습니다. 네트워크 상태를 점검해주세요.",
        en = "Request failed. Please check your network status."
    )]
    Reqwest(String),

    #[translate(
        ko = "데이터베이스 오류가 발생했습니다. 관리자에게 문의해주세요.",
        en = "A database error has occurred. Please contact the administrator."
    )]
    DatabaseError(String),

    #[translate(
        ko = "입력데이터가 모두 정상인지 확인해주세요.",
        en = "Please check if all input data is correct."
    )]
    ValidationError(String),

    // Ratel
    // PresidentialCandidate
    #[translate(
        ko = "대선후보 등록에 실패했습니다. 관리자에게 문의해주세요.",
        en = "Failed to register presidential candidate. Please contact the administrator."
    )]
    PresidentialCandidateCreateError,

    #[translate(
        ko = "대선후보 수정에 실패했습니다",
        en = "Failed to update presidential candidate."
    )]
    PresidentialCandidateUpdateError,

    // ElectionPledge
    #[translate(
        ko = "공약 등록에 실패했습니다. 관리자에게 문의해주세요.",
        en = "Failed to register election pledge. Please contact the administrator."
    )]
    ElectionPledgeCreateError,
    #[translate(
        ko = "공약 수정에 실패했습니다. 관리자에게 문의해주세요.",
        en = "Failed to update election pledge. Please contact the administrator."
    )]
    ElectionPledgeUpdateError,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}

impl From<gloo_net::Error> for Error {
    fn from(e: gloo_net::Error) -> Self {
        Self::Reqwest(e.to_string())
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Self::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for Error {
    fn into_response(self) -> by_axum::axum::response::Response {
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
