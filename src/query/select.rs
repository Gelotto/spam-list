use crate::{error::ContractError, msg::SelectResponse, state::OWNER};
use cosmwasm_std::{Addr, Deps};
use cw_repository::client::Repository;

pub fn select(
  deps: Deps,
  fields: Option<Vec<String>>,
  account: Option<Addr>,
) -> Result<SelectResponse, ContractError> {
  let loader = Repository::loader(deps.storage, &fields, &account);
  Ok(SelectResponse {
    owner: loader.get("owner", &OWNER)?,
  })
}
