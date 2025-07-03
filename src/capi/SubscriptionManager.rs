

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


#[repr(C)]
pub struct SubscriptionManagerI {
  pub blocking_recv: Receiver<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
  pub nonblocking_into: Sender<crate::default_subscription::ResponseData>,
  pub nvacl: NavAbilityClient, // clone of the NavAbilityClient for this SubscriptionManager
}

// // #[repr(C)]
// pub struct SubscriptionManagerII {
//   pub nonblocking_recv: Receiver<crate::default_subscription::ResponseData>,
//   pub blocking_into: Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
// }


// pub struct Tuple {
//   pub smi:  Option<Box<SubscriptionManagerI>>,
//   pub smii: Option<Box<SubscriptionManagerII>>,
//   pub nvasm: Option<Box<SubscriptionManager>>,
// }

#[no_mangle] pub unsafe extern "C" 
fn dummy() -> Option<Box<SubscriptionManagerI>> {None}


// use tokio::runtime::Runtime;

// static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
//     Runtime::new().unwrap()
// });

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


// // ref. https://doc.rust-lang.org/std/boxed/
// #[allow(non_snake_case)]
// #[no_mangle] pub unsafe extern "C" 
// fn new_SubsChannels() -> *mut Tuple {
//     // create the channels for non-blocking and blocking interfaces
//   let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

//   let smi = SubscriptionManagerI {
//     blocking_recv,
//     nonblocking_into,
//   };

//   let smii = SubscriptionManagerII {
//     nonblocking_recv,
//     blocking_into,
//   };

//   let tup = Tuple {
//     smi:  Some(Box::new(smi)),
//     smii: Some(Box::new(smii)),
//     nvasm: None, // will be set later
//   };

//   return Box::into_raw(Box::new(tup));
// }


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

// use once_cell::sync::Lazy;
// use futures::future::Lazy;

use std::thread;
use std::time::Duration;

// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn assign_SubscriptionManager(
  _nvacl: Option<&NavAbilityClient>,
  size: usize,
  // tup_: Option<&mut Tuple>,
  callback: extern "C" fn(*mut c_void),
) -> Option<Box<SubscriptionManager>> {
  if _nvacl.is_none() {
    let msg = "assign_SubscriptionManager: the provided for *NavAbilityClient is NULL/None".to_owned();
    to_console_error(&msg);
    panic!("{}", msg);
    // return None;
  }

  let nvacl__ = _nvacl.unwrap().clone();

  let nvacl = NavAbilityClient::similar(
    &nvacl__,
    true, // do_events
  );

  // create the channels for non-blocking and blocking interfaces
  let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = SubscriptionManager::new_channels();

  let smi = SubscriptionManagerI {
    blocking_recv,
    nonblocking_into,
    nvacl: nvacl.clone(), // clone the NavAbilityClient for this SubscriptionManager
  };

  let nvasm = SubscriptionManager::from_parts(&nvacl.clone(), size, nonblocking_recv, blocking_into);

  // SubscriptionManager::subscription_listener(
  //   nonblocking_into,
  //   _nvacl.unwrap(),
  //   blocking_recv,
  // );

  to_console_debug(&format!("before callback, smi.nvacl.api_url={:?}",&smi.nvacl.apiurl));

  // //https://stackoverflow.com/questions/24191249/working-with-c-void-in-an-ffi
  // let state_ptr: *mut c_void = &mut smi as *mut _ as *mut c_void;
  // callback(state_ptr);
  // listen_ffi(Some(Box::new(smi)));
  
  to_console_debug("assign_SubscriptionManager: before spawning subscription listener thread");
  let nvacl_sm = nvacl__.clone();
  // let handles = (0..1).map(|worker_id| {
    // let handle: thread::JoinHandle<()> = 
    thread::spawn( move || {
        // listenSubscriptions(smi)
      // to_console_debug(&format!("assign_SubMan future: starting subscription listener, nvacl.api_url={:?}", &nvacl.apiurl));
      println!("assign_SubMan future: starting subscription listener, nvacl.api_url={:?}", &nvacl.apiurl);
      SubscriptionManager::subscription_listener(
        smi.nonblocking_into,
        &nvacl_sm,
        smi.blocking_recv,
      );
      ()
    });
  //   handle
  // }); //.collect::<Vec<thread::JoinHandle<()>>>();


  // let handles = (0..1).map(|worker_id| {
  //   println!("Spawning worker {}", worker_id);
  //   let handle = thread::spawn(move || {
  //     println!("Worker {} is running", worker_id);
  //     for _ in 0..2 {
  //       thread::sleep(Duration::from_millis(10));
  //       println!("Worker {} did some work", worker_id);
  //     };
  //   });
  //   println!("Worker {} has spawned", worker_id);
  //   handle
  // }).collect::<Vec<thread::JoinHandle<()>>>();
  to_console_debug("assign_SubscriptionManager: subscription listener thread spawned");
  

  // RUNTIME.spawn(future);


  // return Box::into_raw(Box::new(tup2));
  return Some(Box::new(nvasm));
}


// use async_ffi::{FfiFuture, FutureExt};

// #[no_mangle]
// pub extern "C" fn listen_ffi(smi: Option<Box<SubscriptionManagerI>>) -> FfiFuture<()> {
//     async move {
//         listenSubscriptions(smi)
//     }
//     .into_ffi()
// }


// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn listenSubscriptions(
  smi_: Option<Box<SubscriptionManagerI>>,
) {
  if smi_.is_none() {
    to_console_error("listenSubscriptions: provided *SubscriptionManagerI is NULL/None");
    return;
  }
  let smi = smi_.unwrap();
  let nvacl = smi.nvacl.clone();
  to_console_debug(&format!("listenSubscriptions: starting subscription listener, nvacl.api_url={:?}", &nvacl.apiurl));
  SubscriptionManager::subscription_listener(
    smi.nonblocking_into,
    &nvacl,
    smi.blocking_recv,
  );
  to_console_debug("listenSubscriptions started");
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