

use std::{
  os::raw::{
    c_char,
  }
};



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_solveParametric(
  nvafg: Option<&crate::NavAbilityDFG>,
) {
  // see navabilitysdk::startWorker(args) for details
  println!("startWorker_solveParametric called, but not implemented");
}


// startWorker_LidarRegistration(nvafg, "x1", "left_lidar.las", "x5", "left_lidar.las");
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_LidarRegistration(
  nvafg: Option<&crate::NavAbilityDFG>,
  v1_lbl: *const c_char,
  be1_lbl: *const c_char,
  v2_lbl: *const c_char,
  be2_lbl: *const c_char
) {
  // see navabilitysdk::startWorker(args) for details
  println!("startWorker_solveParametric called, but not implemented");
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_ImageWhitebalance(
  nvafg: Option<&crate::NavAbilityDFG>,
  v_lbl: *const c_char,
  be_lbl: *const c_char,
  be_out_lbl: *const c_char,
) {
  // see navabilitysdk::startWorker(args) for details
  println!("startWorker_solveParametric called, but not implemented");
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn startWorker_VisualAffordancePriors(
  nvafg: Option<&crate::NavAbilityDFG>,
  v_lbl: *const c_char,
  be_lbl: *const c_char,
) {
  // see navabilitysdk::startWorker(args) for details
  println!("startWorker_solveParametric called, but not implemented");
}