

use crate::GetLabel;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Utc,
  Uuid,
  Sender,
  GraphQLQuery,
  Response,
  Error,
  NavAbilityClient,
  post_to_nvaapi,
  ListGraphs,
  list_graphs,
  AddFactorgraph,
  AddFactorgraphBlobEntry,
  add_factorgraph_blob_entry,
  FindOrgModelGraphs,
  to_console_debug,
  to_console_error,
  SDK_VERSION
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_graphs(
  nvacl: &NavAbilityClient,
  id: &Uuid,
) -> Result<list_graphs::ResponseData, Box<dyn Error>> {
  
  let request_body = ListGraphs::build_query(list_graphs::Variables {
    id: id.to_string()
  });
  
  return post_to_nvaapi::<
    list_graphs::Variables,
    list_graphs::ResponseData,
    list_graphs::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_factorgraph(
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<crate::add_factorgraph::ResponseData, Box<dyn Error>> {
  
  let oid = Uuid::parse_str(&nvacl.user_label).expect("cannot parse org_id");
  let request_body = AddFactorgraph::build_query(crate::add_factorgraph::Variables {
    org_id: Some(nvacl.user_label.to_string()),
    id: Some(Uuid::new_v5(&oid, label.as_bytes()).to_string()),
    label: Some(label.to_string()),
    description: Some(description.to_string()),
    metadata: Some(metadata.to_string()),
    version: Some(SDK_VERSION.to_string()),
  });
  
  return post_to_nvaapi::<
    crate::add_factorgraph::Variables,
    crate::add_factorgraph::ResponseData,
    crate::add_factorgraph::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn add_factorgraph_send(
  send_into: std::sync::mpsc::Sender<crate::add_factorgraph::ResponseData>, //get_blob_entry::ResponseData>,
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<(), Box<dyn Error>> {
  return crate::send_api_result(
    send_into, 
    post_add_factorgraph(nvacl, label, description, metadata).await,
  );
}



#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn addFactorgraph(
  nvacl: &NavAbilityClient,
  label: &str,
  description: &str,
  metadata: &str,
) -> Result<crate::add_factorgraph::ResponseData, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(
    post_add_factorgraph(
      nvacl,
      label,
      description,
      metadata
    )
  );
}





// FIXME return Uuid (not string)
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_graph_entry(
  nvacl: &NavAbilityClient,
  fg_label: &String,
  entry: &crate::BlobEntry,
) -> Result<String,Box<dyn Error>> {
  
  let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
  let name = format!("{}{}",&fg_label,&entry.label).to_string();
  let entry_id = Uuid::new_v5(&org_id, name.as_bytes());
  
  let mut size_s: Option<String> = None;
  if let Some(sz) = entry.size {
    size_s = Some(format!("{}",sz));
  }
  let mut metadata = entry.metadata.to_string();
  if metadata.is_empty() {
    metadata = "e30=".to_string();
  }
  
  let variables = crate::add_factorgraph_blob_entry::Variables {
    fg_label: fg_label.to_string(),
    entry_id: entry_id.to_string(),
    entry_label: entry.label.to_string(),
    blob_id: entry.blobId.to_string(),
    blobstore: Some(entry.blobstore.to_string()),
    origin: Some(entry.origin.to_string()),
    mime_type: Some(entry.mimeType.to_string()),
    metadata: metadata,
    description: Some(entry.description.to_string()),
    hash: entry.hash.to_string(),
    size: size_s,
    timestamp: Some(entry.timestamp.to_string()),
  };
  
  let request_body = AddFactorgraphBlobEntry::build_query(variables);
  
  return post_to_nvaapi::<
  crate::add_factorgraph_blob_entry::Variables,
  crate::add_factorgraph_blob_entry::ResponseData,
  String
  >(
    nvacl,
    request_body, 
    |s| {
      s.add_blob_entries.blob_entries[0].id.clone()
    },
    Some(1)
  ).await;
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_connect_graph_agent(
  nvacl: &NavAbilityClient,
  graph: &str,
  agent: &str,
) -> Result<crate::connect_graph_agent::ResponseData, Box<dyn Error>> {
  
  let oid = Uuid::parse_str(&nvacl.user_label).expect("cannot parse org_id");
  let gid = Uuid::new_v5(&oid, graph.as_bytes()).to_string();
  let aid = Uuid::new_v5(&oid, agent.as_bytes()).to_string();
  let request_body = crate::ConnectGraphAgent::build_query(crate::connect_graph_agent::Variables {
    fg_id: gid,
    agent_id: aid,
  });
  
  return post_to_nvaapi::<
    crate::connect_graph_agent::Variables,
    crate::connect_graph_agent::ResponseData,
    crate::connect_graph_agent::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_find_org_model_fgs(
  nvacl: &NavAbilityClient,
  model_label_contains: Option<String>,
  fg_label_contains: Option<String>,
) -> Result<crate::find_org_model_graphs::ResponseData, Box<dyn Error>> {
  
  // let oid = Uuid::parse_str(&nvacl.user_label).expect("cannot parse org_id");
  let request_body = crate::FindOrgModelGraphs::build_query(
    crate::find_org_model_graphs::Variables {
      org_id: nvacl.user_label.clone(),
      model_label_contains,
      fg_label_contains,
    }
  );
  
  return post_to_nvaapi::<
    crate::find_org_model_graphs::Variables,
    crate::find_org_model_graphs::ResponseData,
    crate::find_org_model_graphs::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn find_org_model_fgs_send(
  send_into: std::sync::mpsc::Sender<crate::find_org_model_graphs::ResponseData>, //get_blob_entry::ResponseData>,
  nvacl: &NavAbilityClient,
  model_label_contains: Option<String>,
  fg_label_contains: Option<String>,
) -> Result<(), Box<dyn Error>> {
  return crate::send_api_result(
    send_into, 
    post_find_org_model_fgs(
      nvacl, 
      model_label_contains,
      fg_label_contains
    ).await,
  );
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn findFactorgraphs(
  nvacl: &NavAbilityClient,
  model_label_contains: Option<String>,
  fg_label_contains: Option<String>,
) -> Result<crate::find_org_model_graphs::ResponseData, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(
    post_find_org_model_fgs(
      nvacl,
      model_label_contains,
      fg_label_contains,
    )
  );
}