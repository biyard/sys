use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CryptoStance {
    #[default]
    #[translate(en = "No Stance")]
    NoStance = 0,
    #[translate(en = "Pro-Crypto")]
    ProCrypto = 1,
    #[translate(en = "Neutral")]
    Neutral = 2,
    #[translate(en = "Anti-Crypto")]
    AntiCrypto = 3,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AssemblyMemberChangeStance {
    pub id: i64,
    pub stance: CryptoStance,
}

#[api_model(base = "/v1/assembly-members", table = assembly_members, iter_type = QueryResponse, action_by_id = [change_stance(code = String, stance = CryptoStance), send_verify_email])]
pub struct AssemblyMember {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique)]
    pub code: String,
    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub party: String,
    #[api_model(summary)]
    pub district: String,

    #[api_model(summary)]
    pub en_name: String,
    #[api_model(summary)]
    pub en_party: String,
    #[api_model(summary)]
    pub en_district: Option<String>,

    #[api_model(summary, type = INTEGER, query_action = list_by_stance)]
    pub stance: CryptoStance,
    #[api_model(summary)]
    pub image_url: String,
    pub email: Option<String>,
    // pub email_verified: bool, // check email verified logic
}
