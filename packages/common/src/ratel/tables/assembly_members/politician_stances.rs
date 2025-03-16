use bdk::prelude::*;

use super::CryptoStance;

#[api_model(base = "/m1/politicians/stances", table = assembly_members)]
pub struct PoliticianStances {
    #[api_model(skip, action = change_stances)]
    pub ids: Vec<i64>,

    #[api_model(type = INTEGER, action = change_stances)]
    pub stance: CryptoStance,
}
