use crate::{
  error::ContractError,
  models::{Report, Spam, SpamProfile},
  state::{
    require_authorized_reporter, require_valid_addresses, require_valid_tags, IX_REPORTER,
    IX_REPORT_COUNT, IX_TAG, IX_TIME, REPORTS, SPAM,
  },
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

pub fn report(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  address: Addr,
  comment: Option<String>,
  tags: Option<Vec<String>>,
  tx_hashes: Option<Vec<String>>,
) -> Result<Response, ContractError> {
  let action = "report";
  let tags = tags.unwrap_or(vec![]).clone();

  require_authorized_reporter(deps.storage, &info.sender)?;
  require_valid_addresses(deps.api, vec![address.clone()])?;
  require_valid_tags(&tags)?;

  // Upsert a Spam entry
  let spam = SPAM.update(
    deps.storage,
    address.clone(),
    |maybe_entry| -> Result<_, ContractError> {
      if let Some(mut entry) = maybe_entry {
        entry.report_count += 1;
        Ok(entry)
      } else {
        Ok(Spam {
          profile: SpamProfile::new(address.to_string().len() == 63),
          address: address.clone(),
          tags: tags.clone(),
          is_forgiven: false,
          report_count: 1,
        })
      }
    },
  )?;

  // Create a report that references the Spam address
  REPORTS.update(
    deps.storage,
    (address.clone(), info.sender.clone()),
    |maybe_report| -> Result<_, ContractError> {
      if maybe_report.is_some() {
        Err(ContractError::DuplicateReport)
      } else {
        Ok(Report {
          address: address.clone(),
          reported_at: env.block.time,
          reported_by: None,
          comment,
          tx_hashes,
          tags: tags.clone(),
          block: env.block.clone(),
        })
      }
    },
  )?;

  // Update tag index
  for tag in tags.iter() {
    IX_TAG.save(
      deps.storage,
      (tag.to_lowercase().trim().to_owned(), address.clone()),
      &true,
    )?;
  }

  // Update report count index for searching by reports by report count
  IX_REPORT_COUNT.remove(deps.storage, (spam.report_count - 1, address.clone()));
  IX_REPORT_COUNT.save(deps.storage, (spam.report_count, address.clone()), &true)?;

  // Update block time index
  IX_TIME.save(
    deps.storage,
    (env.block.time.nanos(), address.clone()),
    &true,
  )?;

  // Update reporter index for searching for reports by reporter
  IX_REPORTER.update(
    deps.storage,
    (info.sender.clone(), address.clone()),
    |maybe_bool| -> Result<_, ContractError> { Ok(maybe_bool.unwrap_or(true)) },
  )?;

  Ok(Response::new().add_attributes(vec![attr("action", action)]))
}
