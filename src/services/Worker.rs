
use regex::Regex;

use serde_json;
use serde::Serialize;

#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
  Uuid,
  // Serialize,
  GraphQLQuery,
  QueryBody,
  Error,
  // GQLRequestError,
  // GQLResponseEmptyError,
  NavAbilityClient,
  post_to_nvaapi,
  StartWorker, // start_worker
  to_console_debug, 
  to_console_error,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn start_worker_query(
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> QueryBody<crate::start_worker::Variables>{

  // let res = serde_json::from_str::<serde_json::Map<String,serde_json::Value>>(input).unwrap();
  return StartWorker::build_query(
    crate::start_worker::Variables {
      input,
      worker_label
    }
  );
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<Uuid, Box<dyn Error>> {
  
  let request_body = start_worker_query(input, worker_label);
  
  let bad_json_on_resp = post_to_nvaapi::<
    crate::start_worker::Variables,
    crate::start_worker::ResponseData,
    crate::start_worker::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;

  match bad_json_on_resp {
    Ok(res) => {
      let rstr = &res.start_worker.unwrap();
      let idstr = rstr["id"].as_str().expect(&format!("Unable to estract 'id' from post_start_worker response {:?}",&rstr["id"]));
      let id = Uuid::parse_str(&idstr).map_err(|e_| Box::new(e_) as Box<dyn Error>);
      return id;
    },
    Err(e) => {
      return Err(e);
    }
  }
}



#[cfg(any(feature = "tokio", feature = "thread"))]
pub fn startWorker(
  nvacl: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<Uuid, Box<dyn Error>> {
  return crate::execute(post_start_worker(
    nvacl,
    input,
    worker_label
  ));
}
    
#[cfg(feature = "wasm")]
pub fn startWorker(
  nvacl_: &NavAbilityClient,
  input: serde_json::Map<String,serde_json::Value>,
  worker_label_: crate::start_worker::WorkerLabelEnum
) {

  let nvacl = nvacl_.clone();
  // let input = input_.to_string();
  let worker_label = worker_label_;
  return crate::execute(async move {
      let _ = post_start_worker(
      &nvacl,
      input,
      worker_label
    ).await;
  });
}

