use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_lib::models::Owner;

use crate::models::{Appeal, Report, Spam};

#[cw_serde]
pub struct InstantiateMsg {
  pub owner: Option<Owner>,
}

#[cw_serde]
pub enum ExecuteMsg {
  Report {
    address: Addr,
    comment: Option<String>,
    tx_hashes: Option<Vec<String>>,
    tags: Option<Vec<String>>,
  },
  Appeal {
    address: Addr,
    argument: String,
  },
  Forgive {
    appellant: Addr,
    address: Addr,
  },
}

#[cw_serde]
pub enum QueryMsg {
  Select {
    fields: Option<Vec<String>>,
    wallet: Option<Addr>,
  },
  Details {
    address: Addr,
  },
  Paginate {
    cursor: Cursor,
    limit: Option<u32>,
    reversed: Option<bool>,
  },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum Cursor {
  Timestamp(Option<Timestamp>),
  ReportCount(Option<u32>),
  Reporter(Option<Addr>),
  Tag(Option<String>),
}

#[cw_serde]
pub struct SelectResponse {
  pub owner: Option<Owner>,
}

#[cw_serde]
pub struct SpamResponse {
  pub spam: Spam,
  pub reports: Vec<Report>,
  pub appeals: Vec<Appeal>,
}

#[cw_serde]
pub struct PaginationResponse {
  pub page: Vec<Spam>,
  pub next: Option<Cursor>,
}
