

use std::{
  os::raw::{
    c_void,
    c_char,
  }
};

use uuid::Uuid;

use std::sync::mpsc::{
  Sender,
  Receiver,
};

use crate::{
  to_console_debug,
  to_console_error,
  NavAbilityClient,
  SubscriptionManager,
  convert_str,
  cstr_to_str,
  NavAbilityDFG,
};


use std::thread;
use std::time::Duration;

// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn start_SubscriptionManager(
  _nvacl: Option<&NavAbilityClient>,
  size: usize,
) -> Option<Box<SubscriptionManager>> {
  if _nvacl.is_none() {
    let msg = "assign_SubscriptionManager: the provided for *NavAbilityClient is NULL/None".to_owned();
    to_console_error(&msg);
    panic!("{}", msg);
    // return None;
  }

  let nvacl = _nvacl.unwrap().clone();

  // create the channels for non-blocking and blocking interfaces
  let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

  let nvasm = SubscriptionManager::from_parts(&nvacl.clone(), size, nonblocking_recv, blocking_into);

  thread::spawn( move || {
    SubscriptionManager::subscription_listener(
      nonblocking_into,
      &nvacl.clone(),
      blocking_recv,
    );
    ()
  });
  
  return Some(Box::new(nvasm));
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
  let res = _nvasm.unwrap().block_on(&wid, tout);
  
  to_console_debug(&format!("block_on: waiting for worker id {:?} with timeout {} ms, result: {:?}", wid, tout_millis, &res));
  return res.unwrap_or(false);
}


//