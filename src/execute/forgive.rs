use crate::{
  error::ContractError,
  state::{APPEALS, SPAM},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn forgive(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  address: Addr,
  appellant: Addr,
) -> Result<Response, ContractError> {
  let action = "forgive";
  let reporter = &info.sender;

  if let Some(mut spam) = SPAM.may_load(deps.storage, address.clone())? {
    let appeal_key = (address.clone(), appellant.clone());
    if let Some(mut appeal) = APPEALS.may_load(deps.storage, appeal_key.to_owned())? {
      if appeal.accept(reporter) {
        if appeal.is_satisfied() {
          spam.is_forgiven = true;
          SPAM.save(deps.storage, address.clone(), &spam)?;
        }
        APPEALS.save(deps.storage, appeal_key, &appeal)?;
      } else {
        return Err(ContractError::AppealAlreadyAccepted);
      }
    } else {
      return Err(ContractError::AppealNotFound);
    }
  } else {
    return Err(ContractError::SpamEntryNotFound);
  }

  Ok(Response::new().add_attributes(vec![attr("action", action)]))
}
