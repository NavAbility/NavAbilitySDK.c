
use regex::Regex;

use serde_json;

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
  input: &str,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> QueryBody<crate::start_worker::Variables>{
  return StartWorker::build_query(
    crate::start_worker::Variables {
        input: input.to_string(),
        worker_label
    }
  );
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: &str,
  worker_label: crate::start_worker::WorkerLabelEnum
) -> Result<Uuid, Box<dyn Error>> {
  
  let request_body = start_worker_query(input, worker_label);
  // let variables = crate::start_worker::Variables {
  //   input: input.to_string(),
  //   worker_label
  // };
  // let request_body = StartWorker::build_query(variables);
  
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
      // to_console_debug(&format!("post_start_worker response: {:?}", &res));
      return Uuid::parse_str(&res.start_worker.unwrap().to_string()).map_err(|e_| Box::new(e_) as Box<dyn Error>);
    },
    Err(e) => {
      let re = Regex::new("[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap();
      let checkstr = format!("{}", &e);
      if let Some(mat) = re.find(&checkstr) {
        return Uuid::parse_str(&mat.as_str())
          .map_err(|e_| {
            to_console_error("Failed in attempt to regex->parse->Uuid given upstream start_worker reponse error.");
            return Box::new(e_) as Box<dyn Error>;
          }
        );
      } else {
        to_console_error(&format!("Unable to regex extract UUID from checkstr: {}", &checkstr));
      }
      return Err(e);
    }
  }
}

    // |s| {
    //   // let sm = serde_json::from_str(s).unwrap();
    //   if let Some(sw) = &s.start_worker {
    //     to_console_debug(&format!("start worker response: {:?}", &sw));
    //     match serde_json::from_str::<serde_json::Map<String,serde_json::Value>>(sw) {
    //       Ok(res) => {
    //         //FIXME, make more robust -- ensure fields are present etc.
    //         return res["id"].to_string();
    //       },
    //       Err(e) => {
    //         to_console_error(&format!("start worker parse response error: {:?}", e));
    //         return "".to_string();
    //       }
    //     }
    //   }
    //   return "".to_string();
    // },





