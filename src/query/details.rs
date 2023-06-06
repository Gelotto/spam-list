use crate::{
  error::ContractError,
  msg::SpamResponse,
  state::{load_appeals, load_reports, load_spam},
};
use cosmwasm_std::{Addr, Deps};

pub fn details(
  deps: Deps,
  address: Addr,
) -> Result<SpamResponse, ContractError> {
  Ok(SpamResponse {
    spam: load_spam(deps.storage, &address)?,
    reports: load_reports(deps.storage, &address)?,
    appeals: load_appeals(deps.storage, &address)?,
  })
}
