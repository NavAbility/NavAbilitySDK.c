

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

#[no_mangle] pub unsafe extern "C" 
fn dummy() -> Option<Box<SubscriptionManagerI>> {None}

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


// // ref. https://doc.rust-lang.org/std/boxed/
// #[allow(non_snake_case)]
// #[no_mangle] pub unsafe extern "C" 
// fn get_NvaSubsMan(
//   tup_: Option<&Tuple>,
// ) -> Option<Box<SubscriptionManager>> {
//   if tup_.is_none() {
//     let msg = "get_NvaSubsMan: the provided for *SubscriptionManager is NULL/None".to_owned();
//     to_console_error(&msg);
//     panic!("{}", msg);
//     // return None;
//   }

//   let tup = tup_.unwrap();
//   // TODO if None
//   return Some(tup.nvasm.unwrap());
// }

// #[no_mangle] pub unsafe extern "C" 
// fn set_on_data_callback(
//   callback: extern "C" fn(*mut c_void),
// ) {
//   callback(buffer.len() as *mut c_void);
// }

// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn assign_SubscriptionManager(
  _nvacl: Option<&NavAbilityClient>,
  size: usize,
  tup_: Option<&mut Tuple>,
  callback: extern "C" fn(*mut c_void),
) -> *mut Tuple {
  if _nvacl.is_none() {
    let msg = "assign_SubscriptionManager: the provided for *NavAbilityClient is NULL/None".to_owned();
    to_console_error(&msg);
    panic!("{}", msg);
    // return None;
  }

  // // create the channels for non-blocking and blocking interfaces
  // let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

  // let smii = SubscriptionManagerII {
  //   nonblocking_recv,
  //   blocking_into,
  // };

  let tup = Box::from_raw(tup_.unwrap());

  let smii = (tup.smii.unwrap()); // take ownership of the SubscriptionManagerII
  let mut smi = (tup.smi.unwrap()); // take ownership of the SubscriptionManagerII

  let nvasm = SubscriptionManager::from_parts(_nvacl.unwrap(), size, smii.nonblocking_recv, smii.blocking_into);

  // rebuild the Tuple with the new SubscriptionManager but without the box to SubscriptionManagerII channels
  let tup2 = Tuple {
    smi:  None, // Some(tup.smi.unwrap()), // keep the SubscriptionManagerI
    smii: None, // we no longer need this
    nvasm: Some(Box::new(nvasm)), // set the SubscriptionManager we just created
  };

  // *nvasm = nvasm_;

  // SubscriptionManager::subscription_listener(
  //   nonblocking_into,
  //   _nvacl.unwrap(),
  //   blocking_recv,
  // );
  let state_ptr: *mut c_void = &mut smi as *mut _ as *mut c_void;
  callback(state_ptr);

  return Box::into_raw(Box::new(tup2));
  // return Some(Box::new(nvasm));
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