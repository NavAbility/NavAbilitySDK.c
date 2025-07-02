

use std::{
  os::raw::{
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

// #[repr(C)]
pub struct SubscriptionManagerI {
  pub blocking_recv: Receiver<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
  pub nonblocking_into: Sender<crate::default_subscription::ResponseData>,
}

// #[repr(C)]
pub struct SubscriptionManagerII {
  pub nonblocking_recv: Receiver<crate::default_subscription::ResponseData>,
  pub blocking_into: Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
}


pub struct Tuple {
  pub smi:  Option<Box<SubscriptionManagerI>>,
  pub smii: Option<Box<SubscriptionManagerII>>,
  pub nvasm: Option<Box<SubscriptionManager>>,
}

// use std::convert::From;

// impl From<
//   SubscriptionManagerII
// > for (
//   Receiver<crate::default_subscription::ResponseData>,
//   Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
// ) {
//   fn from(
//     tup: SubscriptionManagerII
//   ) -> (
//     Receiver<crate::default_subscription::ResponseData>,
//     Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
//   ) {
//     return (
//       tup.nonblocking_recv, 
//       tup.blocking_into
//     )
//   }
// }


// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_SubsChannels() -> *mut Tuple {
    // create the channels for non-blocking and blocking interfaces
  let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

  let smi = SubscriptionManagerI {
    blocking_recv,
    nonblocking_into,
  };

  let smii = SubscriptionManagerII {
    nonblocking_recv,
    blocking_into,
  };

  let tup = Tuple {
    smi:  Some(Box::new(smi)),
    smii: Some(Box::new(smii)),
    nvasm: None, // will be set later
  };

  return Box::into_raw(Box::new(tup));
}



// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn assign_SubscriptionManager(
  _nvacl: Option<&NavAbilityClient>,
  size: usize,
  tup_: Option<&mut Tuple>,
) -> Option<Box<SubscriptionManager>> {
  if _nvacl.is_none() {
    to_console_error("new_SubscriptionManager: provided for *NavAbilityClient is NULL/None");
    return None;
  }

  // // create the channels for non-blocking and blocking interfaces
  // let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

  // let smii = SubscriptionManagerII {
  //   nonblocking_recv,
  //   blocking_into,
  // };

  let tup = Box::from_raw(tup_.unwrap());

  let smii = (tup.smii.unwrap()); // take ownership of the SubscriptionManagerII

  let nvasm = SubscriptionManager::from_parts(_nvacl.unwrap(), size, smii.nonblocking_recv, smii.blocking_into);


  // *nvasm = nvasm_;

  // SubscriptionManager::subscription_listener(
  //   nonblocking_into,
  //   _nvacl.unwrap(),
  //   blocking_recv,
  // );

  return Some(Box::new(nvasm));
}



// // ref. https://doc.rust-lang.org/std/boxed/
// #[allow(non_snake_case)]
// #[no_mangle] pub unsafe extern "C" 
// fn listenSubscriptions(
//   _nvacl: Option<&NavAbilityClient>,
//   _sms: *mut SubscriptionManagerStart,
// ) {
//   if _nvacl.is_none() {
//     to_console_error("listenSubscriptions: provided *NavAbilityClient is NULL/None");
//     return;
//   }
//   // let sms = _sms.unwrap();
//   let sms = Box::from_raw(_sms);
//   // take ownership of the nonblocking channel and the blocking receiver
//   to_console_debug("listenSubscriptions: starting subscription listener");
//   SubscriptionManager::subscription_listener(
//     sms.nonblocking_into,
//     _nvacl.unwrap(),
//     sms.blocking_recv,
//   );
// }


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