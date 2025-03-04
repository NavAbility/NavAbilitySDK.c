

use std::{
  os::raw::{
    c_char,
  }
};

use uuid::Uuid;

use crate::{
  convert_str,
  NavAbilityDFG,
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_solveParametric(
  nvafg: Option<&NavAbilityDFG>,
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("startWorker_solveParametric called, under construction, id: {:?}", &wrk_id);
  return convert_str(&wrk_id.to_string());
}


// startWorker_LidarRegistration(nvafg, "x1", "left_lidar.las", "x5", "left_lidar.las");
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_LidarRegistration(
  nvafg: Option<&NavAbilityDFG>,
  v1_lbl: *const c_char,
  be1_lbl: *const c_char,
  v2_lbl: *const c_char,
  be2_lbl: *const c_char
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("startWorker_LidarRegistration called, under construction, id: {:?}", &wrk_id);
  return convert_str(&wrk_id.to_string());
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_ImageWhitebalance(
  nvafg: Option<&NavAbilityDFG>,
  v_lbl: *const c_char,
  be_lbl: *const c_char,
  be_out_lbl: *const c_char,
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("startWorker_ImageWhitebalance called, under construction, id: {:?}", &wrk_id);
  return convert_str(&wrk_id.to_string());
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_VisualAffordancePriors(
  nvafg: Option<&NavAbilityDFG>,
  v_lbl: *const c_char,
  be_lbl: *const c_char,
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("startWorker_VisualAffordancePriors called, but not implemented");
  return convert_str(&wrk_id.to_string());
}