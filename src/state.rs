use crate::models::{Appeal, Report, Reporter};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::Spam};
use cosmwasm_std::{Addr, Api, Deps, DepsMut, Env, MessageInfo, Order, Storage};
use cw_acl::client::Acl;
use cw_lib::models::Owner;
use cw_storage_plus::{Item, Map};

pub const MAX_TAG_LEN: usize = 50;
pub const MAX_TAG_COUNT: usize = 5;

pub const OWNER: Item<Owner> = Item::new("owner");
pub const SPAM: Map<Addr, Spam> = Map::new("spam");
pub const REPORTERS: Map<Addr, Reporter> = Map::new("reporters");
pub const REPORTS: Map<(Addr, Addr), Report> = Map::new("reports");
pub const APPEALS: Map<(Addr, Addr), Appeal> = Map::new("appeals");

// indexes for search functionality
pub const IX_TAG: Map<(String, Addr), bool> = Map::new("ix_tag");
pub const IX_TIME: Map<(u64, Addr), bool> = Map::new("ix_time");
pub const IX_REPORT_COUNT: Map<(u32, Addr), bool> = Map::new("ix_report_count");
pub const IX_REPORTER: Map<(Addr, Addr), bool> = Map::new("ix_reporter");

pub fn load_spam(
  storage: &dyn Storage,
  address: &Addr,
) -> Result<Spam, ContractError> {
  if let Some(spam) = SPAM.may_load(storage, address.clone())? {
    Ok(spam)
  } else {
    Err(ContractError::SpamEntryNotFound)
  }
}

pub fn load_appeals(
  storage: &dyn Storage,
  address: &Addr,
) -> Result<Vec<Appeal>, ContractError> {
  Ok(
    APPEALS
      .prefix(address.clone())
      .range(storage, None, None, Order::Ascending)
      .map(|r| {
        let (appealed_by, mut appeal) = r.unwrap();
        appeal.appealed_by = Some(appealed_by);
        appeal
      })
      .collect(),
  )
}

pub fn load_reports(
  storage: &dyn Storage,
  address: &Addr,
) -> Result<Vec<Report>, ContractError> {
  Ok(
    REPORTS
      .prefix(address.clone())
      .range(storage, None, None, Order::Ascending)
      .map(|r| {
        let (reported_by, mut report) = r.unwrap();
        report.reported_by = Some(reported_by);
        report
      })
      .collect(),
  )
}

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(
    deps.storage,
    &msg
      .owner
      .clone()
      .unwrap_or(Owner::Address(info.sender.clone())),
  )?;
  Ok(())
}

pub fn require_sender_is_owner(
  deps: &Deps,
  principal: &Addr,
  action: &str,
) -> Result<(), ContractError> {
  if !match OWNER.load(deps.storage)? {
    Owner::Address(addr) => *principal == addr,
    Owner::Acl(acl_addr) => {
      let acl = Acl::new(&acl_addr);
      acl.is_allowed(&deps.querier, principal, action)?
    },
  } {
    Err(ContractError::NotAuthorized {})
  } else {
    Ok(())
  }
}

pub fn require_authorized_reporter(
  storage: &dyn Storage,
  addr: &Addr,
) -> Result<(), ContractError> {
  if !REPORTERS.has(storage, addr.clone()) {
    Err(ContractError::NotAuthorized)
  } else {
    Ok(())
  }
}

pub fn require_valid_tags(tags: &Vec<String>) -> Result<(), ContractError> {
  if tags.len() > MAX_TAG_COUNT {
    return Err(ContractError::TooManyTags);
  }
  for tag in tags.iter() {
    let trimmed_tag = tag.trim();
    if trimmed_tag.len() > 1 && trimmed_tag.len() > MAX_TAG_LEN {
      return Err(ContractError::InvalidTag);
    }
  }
  Ok(())
}

pub fn require_valid_addresses(
  api: &dyn Api,
  addresses: Vec<Addr>,
) -> Result<(), ContractError> {
  for addr in addresses.iter() {
    api.addr_validate(addr.to_string().as_str())?;
  }
  Ok(())
}
