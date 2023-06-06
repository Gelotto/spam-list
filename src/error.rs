use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("SpamEntryNotFound")]
  SpamEntryNotFound,

  #[error("AppealNotFound")]
  AppealNotFound,

  #[error("AppealAlreadyAccepted")]
  AppealAlreadyAccepted,

  #[error("NotAuthorized")]
  NotAuthorized,

  #[error("ValidationError")]
  ValidationError,

  #[error("DuplicateReport")]
  DuplicateReport,

  #[error("TooManyTags")]
  TooManyTags,

  #[error("InvalidTag")]
  InvalidTag,
}
