use crate::{
  error::ContractError,
  models::Appeal,
  state::{load_spam, APPEALS, REPORTS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Order, Response};

pub fn appeal(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  address: Addr,
  argument: String,
) -> Result<Response, ContractError> {
  let action = "appeal";
  let spam = load_spam(deps.storage, &address)?;

  // If the spam address belongs to a smart contract, then anyone can appeal;
  // otherwise, only the spam address itself can appeal.
  if !spam.profile.is_contract || address != info.sender {
    return Err(ContractError::NotAuthorized);
  }

  // Get list of current reporter addresses
  let reported_by: Vec<Addr> = REPORTS
    .prefix(address.clone())
    .keys(deps.storage, None, None, Order::Ascending)
    .map(|r| r.unwrap())
    .collect();

  // Error out if an appeal by the sender already exists for the given report,
  // or create the appeal.
  APPEALS.update(
    deps.storage,
    (address.clone(), info.sender.clone()),
    |maybe_appeal| -> Result<_, ContractError> {
      if maybe_appeal.is_some() {
        Err(ContractError::NotAuthorized)
      } else {
        Ok(Appeal {
          address: address.clone(),
          argument: argument.clone(),
          reported_by,
          appealed_by: None,
          accepted_by: vec![],
        })
      }
    },
  )?;

  Ok(Response::new().add_attributes(vec![attr("action", action)]))
}
