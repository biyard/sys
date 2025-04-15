use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/m1/homepage/contacts", table = contacts)]
pub struct Contact {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    pub last_name: String,
    pub first_name: String,
    #[api_model(unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(summary)]
    pub company_name: String,
    #[api_model(type = INTEGER)]
    #[api_model(summary)]
    pub needs: Need,
    #[api_model(summary)]
    pub help: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Need {
    #[default]
    #[translate(en = "General Inquiry", ko = "일반 문의")]
    GeneralInquiry = 1,
    #[translate(en = "Technical Support", ko = "기술 지원")]
    TechnicalSupport = 2,
    #[translate(en = "Partnership & Collaboration", ko = "제휴 및 협업")]
    PartnershipCollaboration = 3,
    #[translate(en = "Investment & Funding", ko = "투자 및 자금")]
    InvestmentFunding = 4,
    #[translate(en = "Feedback & Suggestions", ko = "피드백 및 제안")]
    FeedbackSuggestions = 5,
}
