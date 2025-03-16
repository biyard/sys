use bdk::prelude::*;

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
}
