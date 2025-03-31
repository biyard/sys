use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CryptoStance {
    #[translate(en = "Strongly Against")]
    StronglyAgainst = 0,
    #[translate(en = "Against")]
    Against = 1,
    #[default]
    #[translate(en = "Neutral")]
    Neutral = 2,
    #[translate(en = "Supportive")]
    Supportive = 3,
    #[translate(en = "Strongly Supportive")]
    StronglySupportive = 4,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AssemblyMemberChangeStance {
    pub id: i64,
    pub stance: CryptoStance,
}

#[api_model(base = "/m1/ratel/politicians", table = assembly_members)]
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
