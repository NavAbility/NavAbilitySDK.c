

use std::{
  os::raw::{
    c_char,
  }
};

use uuid::Uuid;

use crate::{
  to_console_debug,
  to_console_error,
  NavAbilityClient,
  SubscriptionManager,
  convert_str,
  cstr_to_str,
  NavAbilityDFG,
};


//

// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_SubscriptionManager(
  _nvacl: Option<&NavAbilityClient>,
  size: usize,
) -> Option<Box<SubscriptionManager>> {
  if _nvacl.is_none() {
    to_console_error("new_SubscriptionManager: provided for *NavAbilityClient is NULL/None");
    return None;
  }

  return Some(Box::new(SubscriptionManager::new(
      _nvacl.unwrap(),
      size,
  )));
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn block_on(
  _nvasm: Option<&mut SubscriptionManager>,
  wrk_id: *const c_char,
  tout_millis: usize,
) -> bool {
    if _nvasm.is_none() {
    to_console_error("block_on: provided *SubscriptionManager is NULL");
    return false;
  }
  let wid = Uuid::parse_str(cstr_to_str(wrk_id)).expect("Cannot parse wrk_id string to uuid");

  let tout = std::time::Duration::from_millis(tout_millis as u64);
  return _nvasm.unwrap().block_on(&wid, tout).unwrap_or(false);
}


//