use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Party {
    #[default]
    #[translate(en = "Party", ko = "정당")]
    None = 0,
    #[translate(en = "DP", ko = "더불어민주당")]
    DemocraticParty = 1,
    #[translate(en = "PPP", ko = "국민의힘")]
    PeoplePowerParty = 2,
    #[translate(en = "RKP", ko = "조국혁신당")]
    RebuildingKoreaParty = 3,
    #[translate(en = "Jinbo", ko = "진보당")]
    JinboParty = 4,
    #[translate(en = "Reform", ko = "개혁신당")]
    ReformParty = 5,
    #[translate(en = "Basic Income", ko = "기본소득당")]
    BasicIncomeParty = 6,
    #[translate(en = "SDP", ko = "사회민주당")]
    SocialDemocraticParty = 7,
    #[translate(en = "", ko = "무소속")]
    Independent = 8,
}
