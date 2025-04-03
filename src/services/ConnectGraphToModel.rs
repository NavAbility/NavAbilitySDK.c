
use uuid::Uuid;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Error,
  GraphQLQuery,
  NavAbilityClient,
  post_to_nvaapi,
  to_console_debug,
  to_console_error,
  parse_str_utc,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_connect_graph_model(
  nvacl: &NavAbilityClient,
  graph: &str,
  model: &str,
) -> Result<crate::connect_graph_model::ResponseData, Box<dyn Error>> {
  
  let oid = Uuid::parse_str(&nvacl.user_label).expect("cannot parse org_id");
  let gid = Uuid::new_v5(&oid, graph.as_bytes()).to_string();
  let mid = Uuid::new_v5(&oid, model.as_bytes()).to_string();
  let request_body = crate::ConnectGraphModel::build_query(crate::connect_graph_model::Variables {
    fg_id: gid,
    model_id: mid,
  });
  
  return post_to_nvaapi::<
    crate::connect_graph_model::Variables,
    crate::connect_graph_model::ResponseData,
    crate::connect_graph_model::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}



#[cfg(feature = "wasm")]
pub fn connectGraphModel(
  nvacl: &NavAbilityClient,
  graph: &str,
  model: &str,
) {
  let nvacl_ = nvacl.clone();
  let graph_ = graph.to_string();
  let model_ = model.to_string();
  crate::execute(async move {
    let _ = post_connect_graph_model(
      &nvacl_,
      &graph_,
      &model_
    ).await;
  });
}
