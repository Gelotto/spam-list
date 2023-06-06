use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, Timestamp};

#[cw_serde]
pub struct Spam {
  pub address: Addr,
  pub profile: SpamProfile,
  pub tags: Vec<String>,
  pub is_forgiven: bool,
  pub report_count: u32,
}

#[cw_serde]
pub struct SpamProfile {
  pub is_contract: bool,
  pub name: Option<String>,
  pub description: Option<String>,
  pub image_url: Option<String>,
  pub url: Option<String>,
}

#[cw_serde]
pub struct Reporter {
  pub address: Option<Addr>,
  pub name: String,
  pub description: Option<String>,
  pub url: Option<Vec<String>>,
}

#[cw_serde]
pub struct Report {
  pub address: Addr,
  pub reported_at: Timestamp,
  pub reported_by: Option<Addr>,
  pub tags: Vec<String>,
  pub comment: Option<String>,
  pub tx_hashes: Option<Vec<String>>,
  pub block: BlockInfo,
}

#[cw_serde]
pub struct Appeal {
  pub address: Addr,
  pub appealed_by: Option<Addr>,
  pub accepted_by: Vec<Addr>,
  pub reported_by: Vec<Addr>,
  pub argument: String,
}

#[cw_serde]
pub struct Endorsement {
  pub address: Addr,
  pub comment: Option<String>,
  pub tx_hashes: Option<Vec<String>>,
}

impl SpamProfile {
  pub fn new(is_contract: bool) -> Self {
    Self {
      is_contract,
      name: None,
      description: None,
      image_url: None,
      url: None,
    }
  }
}

impl Appeal {
  pub fn accept(
    &mut self,
    reporter: &Addr,
  ) -> bool {
    let is_valid_reporter = self.reported_by.contains(reporter);
    let has_accepted = self.accepted_by.contains(reporter);
    if is_valid_reporter && !has_accepted {
      self.accepted_by.push(reporter.clone());
      true
    } else {
      false
    }
  }

  pub fn is_satisfied(&self) -> bool {
    self.reported_by.len() == self.accepted_by.len()
  }
}
