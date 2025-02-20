
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  // Serialize,
  GraphQLQuery,
  QueryBody,
  Error,
  GQLRequestError,
  GQLResponseEmptyError,
  check_deser,
  check_query_response_data,
  to_console_error,
  NavAbilityClient,
  post_to_nvaapi,
  StartWorker, // start_worker
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_start_worker(
  nvacl: &NavAbilityClient,
  input: &str,
  worker_label: crate::start_worker::mutationInput_post_startWorker_workerLabel
) -> Result<String, Box<dyn Error>> {
  
  let variables = crate::start_worker::Variables {
    input: input.to_string(),
    worker_label
  };
  
  let request_body = StartWorker::build_query(variables);
  
  return post_to_nvaapi::<
    crate::start_worker::Variables,
    crate::start_worker::ResponseData,
    String
  >(
    nvacl,
    request_body, 
    |s|{
      return s.start_worker.unwrap_or("{}".to_owned());
    },
    Some(1)
  ).await;
}







