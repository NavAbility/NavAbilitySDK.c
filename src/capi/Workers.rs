

use std::{
  os::raw::{
    c_char,
  }
};

use uuid::Uuid;

use crate::{
  convert_str,
  cstr_to_str,
  NavAbilityDFG,
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn solveGraphParametric(
  nvafg: Option<&NavAbilityDFG>,
  variableLabel: *const c_char,
) -> *mut c_char {
  
  let mut map = serde_json::Map::<String,serde_json::Value>::new();
  map.insert("lambda".to_string(), serde_json::json!("solveGraphParametricConnected!"));
  map.insert("agentLabel".to_string(), serde_json::json!(nvafg.unwrap().agent.label));
  map.insert("graphLabel".to_string(), serde_json::json!(nvafg.unwrap().fg.label));
  map.insert("variableLabel".to_string(), serde_json::json!(cstr_to_str(variableLabel)));
  map.insert("auth_token".to_string(), serde_json::json!(nvafg.unwrap().client.nva_api_token));

  let wrk_id = crate::services::startWorker(
      &nvafg.unwrap().client.clone(),
      map,
      crate::start_worker::WorkerLabelEnum::rome,
  );

  return convert_str(&wrk_id.expect("start worker failed to return a uuid").to_string());
}


// startWorker_LidarRegistration(nvafg, "x1", "left_lidar.las", "x5", "left_lidar.las");
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn computeLidarRegistration(
  nvafg: Option<&NavAbilityDFG>,
  v1_lbl: *const c_char,
  be1_lbl: *const c_char,
  v2_lbl: *const c_char,
  be2_lbl: *const c_char
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("computeLidarRegistration called, under construction, id: {:?}", &wrk_id);
  return convert_str(&wrk_id.to_string());
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn computeImageWhitebalance(
  nvafg: Option<&NavAbilityDFG>,
  v_lbl: *const c_char,
  be_lbl: *const c_char,
  be_out_lbl: *const c_char,
) -> *mut c_char {
  // see navabilitysdk::startWorker(args) for details
  let wrk_id = Uuid::new_v4();
  println!("computeImageWhitebalance called, under construction, id: {:?}", &wrk_id);
  return convert_str(&wrk_id.to_string());
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn addAffordance_kNNvisual(
  nvafg: Option<&NavAbilityDFG>,
  variableLabel: *const c_char,
  mapsessions: *const c_char,
  n_matches: usize,
  total_n_matches: usize,
) -> *mut c_char {

  let mapsessions = cstr_to_str(mapsessions).split(";").collect::<Vec<&str>>();
  
  let mut map = serde_json::Map::<String,serde_json::Value>::new();
  map.insert("robotLabel".to_string(), serde_json::json!(nvafg.unwrap().agent.label));
  map.insert("sessionLabel".to_string(), serde_json::json!(nvafg.unwrap().fg.label));
  map.insert("variableLabel".to_string(), serde_json::json!(cstr_to_str(variableLabel)));
  map.insert("n_matches".to_string(), serde_json::json!(format!("{}",n_matches)));
  map.insert("total_n_matches".to_string(), serde_json::json!(format!("{}",total_n_matches)));
  map.insert("mapSessionLabels".to_string(), serde_json::json!(mapsessions));
  map.insert("auth_token".to_string(), serde_json::json!(nvafg.unwrap().client.nva_api_token));

  let wrk_id = crate::services::startWorker(
      &nvafg.unwrap().client.clone(),
      map,
      crate::start_worker::WorkerLabelEnum::addAffordance_kNNvisual,
  );

  return convert_str(&wrk_id.expect("start worker failed to return a uuid").to_string());
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn deriveRobotConfig(
  nvafg: Option<&NavAbilityDFG>,
) -> *mut c_char {
  
  let mut map = serde_json::Map::<String,serde_json::Value>::new();
  map.insert("lambda".to_string(), serde_json::json!("updateAgentMetadata_Kalibr"));
  map.insert("agentLabel".to_string(), serde_json::json!(nvafg.unwrap().agent.label));
  map.insert("auth_token".to_string(), serde_json::json!(nvafg.unwrap().client.nva_api_token));

  let wrk_id = crate::services::startWorker(
    &nvafg.unwrap().client.clone(),
    map,
    crate::start_worker::WorkerLabelEnum::accel,
  );

  return convert_str(&wrk_id.expect("start worker failed to return a uuid").to_string());
}