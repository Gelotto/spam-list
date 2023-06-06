use std::marker::PhantomData;

use crate::{
  error::ContractError,
  models::Spam,
  msg::{Cursor, PaginationResponse},
  state::{IX_REPORTER, IX_REPORT_COUNT, IX_TAG, IX_TIME, SPAM},
};
use cosmwasm_std::{Addr, Deps, Order, Timestamp};
use cw_storage_plus::PrefixBound;

pub fn paginate(
  deps: Deps,
  cursor: Cursor,
  maybe_limit: Option<u32>,
  maybe_reversed: Option<bool>,
) -> Result<PaginationResponse, ContractError> {
  let limit = maybe_limit.unwrap_or(50).clamp(1, 50) as usize;
  let reversed = maybe_reversed.unwrap_or_default();
  let order = if reversed {
    Order::Descending
  } else {
    Order::Ascending
  };

  // Get the next "page" of spam address from the queried index.
  let mut next_cursor: Option<Cursor> = None;
  let addresses: Vec<Addr> = match cursor {
    Cursor::ReportCount(maybe_report_count) => IX_REPORT_COUNT
      .prefix_range(
        deps.storage,
        if reversed {
          None
        } else {
          maybe_report_count
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        },
        if reversed {
          maybe_report_count
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        } else {
          None
        },
        order,
      )
      .take(limit)
      .map(|result| {
        let ((k, addr), _v) = result.unwrap();
        next_cursor = Some(Cursor::ReportCount(Some(k)));
        addr
      })
      .collect(),

    Cursor::Reporter(maybe_addr) => IX_REPORTER
      .prefix_range(
        deps.storage,
        if reversed {
          None
        } else {
          maybe_addr
            .clone()
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        },
        if reversed {
          maybe_addr
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        } else {
          None
        },
        order,
      )
      .take(limit)
      .map(|result| {
        let ((k, addr), _v) = result.unwrap();
        next_cursor = Some(Cursor::Reporter(Some(k)));
        addr
      })
      .collect(),

    Cursor::Tag(maybe_tag) => IX_TAG
      .prefix_range(
        deps.storage,
        if reversed {
          None
        } else {
          maybe_tag
            .clone()
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        },
        if reversed {
          maybe_tag
            .and_then(|x| Some(PrefixBound::Exclusive((x, PhantomData))))
            .or(None)
        } else {
          None
        },
        order,
      )
      .take(limit)
      .map(|result| {
        let ((k, addr), _v) = result.unwrap();
        next_cursor = Some(Cursor::Tag(Some(k)));
        addr
      })
      .collect(),

    Cursor::Timestamp(maybe_time) => IX_TIME
      .prefix_range(
        deps.storage,
        if reversed {
          None
        } else {
          maybe_time
            .and_then(|x| Some(PrefixBound::Exclusive((x.nanos(), PhantomData))))
            .or(None)
        },
        if reversed {
          maybe_time
            .and_then(|x| Some(PrefixBound::Exclusive((x.nanos(), PhantomData))))
            .or(None)
        } else {
          None
        },
        order,
      )
      .take(limit)
      .map(|result| {
        let ((k, addr), _v) = result.unwrap();
        next_cursor = Some(Cursor::Timestamp(Some(Timestamp::from_nanos(k))));
        addr
      })
      .collect(),
  };

  // Load all spam entries in the address vec
  let mut page: Vec<Spam> = Vec::with_capacity(addresses.len());
  for addr in addresses {
    page.push(SPAM.load(deps.storage, addr)?);
  }

  Ok(PaginationResponse {
    next: next_cursor,
    page,
  })
}
