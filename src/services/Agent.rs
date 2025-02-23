
use crate::{
  Uuid,
  Utc,
  Agent,
};

use std::collections::HashMap;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[macro_use]
use crate::{
  Sender,
  GraphQLQuery,
  Response,
  Error,
  SDK_VERSION,
  GetId,
  BlobEntry,
  to_console_error,
  parse_str_utc,
  NavAbilityClient,
  NvaNode,
  Factorgraph,
  Model,
  check_deser,
  post_to_nvaapi,
  send_query_result,
  send_api_result,
  // send_api_response,
  check_query_response_data,
  AddAgent,
  add_agent,
  GetAgents, // query vs fn, unique crate::get_agents,
  ListAgents, // query vs fn, unique crate::post_list_agents,
  AgentFieldImportersSummary,
  Agent_importers_summary,
  AgentFieldImportersFull,
  Agent_importers_full,
  get_agent_entries_metadata,
  GetAgentEntriesMetadata,
  UpdateAgentMetadata,
  AddBlobEntryAgent,
  // add_blob_entry_agent,
  GQLRequestError,
};

#[cfg(feature = "wasm")]
use crate::to_console_debug;


// ===================== HELPERS ========================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::get_agents::agent_fields_summary as GA_AgentFieldsSummary;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
Agent_importers_summary!(GA_AgentFieldsSummary);
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::get_agents::agent_fields_full as GA_AgentFieldsFull;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
Agent_importers_full!(GA_AgentFieldsFull);


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl Agent {
  pub fn from_gql_summary(
    aggql: &impl AgentFieldImportersSummary,
  ) -> Self {
    let mut ag = Agent::default();
    ag.id = aggql.id();
    ag.label = aggql.label();
    ag.description = aggql.description();
    ag._version = aggql._version();
    ag.createdTimestamp = aggql.createdTimestamp();
    ag.lastUpdatedTimestamp = aggql.lastUpdatedTimestamp();
    
    return ag
  }
  
  pub fn from_gql_full(
    aggql: &impl AgentFieldImportersFull,
    ag: &mut Self,
  ) {
    ag.metadata = aggql.metadata();
    ag.blobEntries = aggql.blobEntries();
    ag.models = aggql.models();
    ag.fgs = aggql.fgs();
    
    return ();
  }
}


// ===================== QUERIES ========================


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_list_agents(
  nvacl: &NavAbilityClient,
) -> Result<Vec<String>, Box<dyn Error>> {
  // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
  let variables = crate::list_agents::Variables {
    org_id: nvacl.user_label.to_string(),
  };
  
  let request_body = ListAgents::build_query(variables);
  
  return post_to_nvaapi::<
  crate::list_agents::Variables,
  crate::list_agents::ResponseData,
  Vec<String>
  >(
    nvacl,
    request_body, 
    |s| {
      let mut ags = Vec::new();
      for oa in s.orgs {
        for a in oa.agents {
          ags.push(a.label);
        }
      }
      return ags;
    },
    Some(3)
  ).await;
}




#[cfg(any(feature = "tokio", feature = "thread"))] // feature = "thread", 
pub fn listAgents(
  nvacl: &NavAbilityClient,
) -> Result<Vec<String>, Box<dyn Error>> {
  return crate::execute(post_list_agents(nvacl));
}

#[cfg(any(feature = "tokio", feature = "thread"))] // feature = "thread", 
pub fn q_listAgents(
  send_into: Sender<Vec<String>>, 
  nvacl: &NavAbilityClient,
) -> Result<(), Box<dyn Error>> {
  crate::execute(async {
    return send_api_result(
      send_into, 
      post_list_agents(&nvacl).await,
    );
  })
}

#[cfg(feature = "wasm")]
pub fn q_listAgents(
  send_into: Sender<Vec<String>>, 
  nvacl: &NavAbilityClient,
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = nvacl.clone();
  let send_into_ = send_into.clone();
  crate::execute(async move {
    let _ = send_api_result(
      send_into_, 
      post_list_agents(&nvacl_).await,
    );
  });
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_get_agents(
  nvacl: &NavAbilityClient,
  agent_label: Option<&str>
) -> Result<Vec<Agent>, Box<dyn Error>> {
  
  let mut agent_id = None;
  if let Some(agl) = agent_label {
    agent_id = Some(nvacl.getId(agl).to_string());
  }

  // https://github.com/graphql-rust/graphql-client/blob/3090e0add5504ed31df74c32c2bda203793a890a/examples/github/examples/github.rs#L45C1-L48C7
  let variables = crate::get_agents::Variables {
    org_id: nvacl.user_label.to_string(),
    agent_id,
    full: Some(true)
  };
  
  let request_body = GetAgents::build_query(variables);
  
  return post_to_nvaapi::<
  crate::get_agents::Variables,
  crate::get_agents::ResponseData,
  Vec<Agent>
  >(
    nvacl,
    request_body, 
    |s| {
      let mut ags = Vec::new();
      for a in s.agents {
        let mut agent = Agent::from_gql_summary(&a.agent_fields_summary);
        Agent::from_gql_full(&a.agent_fields_full, &mut agent);
        ags.push(agent);
      };
      return ags;
    },
    Some(3)
  ).await;
}


#[cfg(feature = "tokio")]
pub fn getAgents(
  nvacl: &NavAbilityClient,
  agent_label: Option<&str>,
) -> Result<Vec<Agent>, Box<dyn Error>> {
  // return crate::execute(post_get_agents(nvacl, agent_label));
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(post_get_agents(nvacl, agent_label));
}


#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn getAgents_send(
  send_into: Sender<Vec<Agent>>,
  nvacl: &NavAbilityClient,
  agent_label: Option<&str>,
) -> Result<(),Box<dyn Error>> {
  return send_api_result(
    send_into, 
    post_get_agents(nvacl, agent_label).await,
  );
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_agent(
  nvacl: &NavAbilityClient,
  agent_label: &String,
) -> Result<add_agent::ResponseData,Box<dyn Error>> {
  let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
  let name = format!("{}",&agent_label).to_string();
  let agent_id = Uuid::new_v5(&org_id, name.as_bytes());
  
  let variables = add_agent::Variables {
    agent_id: agent_id.to_string(),
    label: agent_label.to_string(),
    version: SDK_VERSION.to_string(),
    org_id: org_id.to_string(),
  };
  
  let request_body = AddAgent::build_query(variables);
  
  return post_to_nvaapi::<
  add_agent::Variables,
  add_agent::ResponseData,
  add_agent::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}



// ------------------------ Agent Entries Metadata ------------------------


            // // let add_data_cache = cache.add_data_window.as_mut().unwrap().cache.as_mut().unwrap();
            // cache.try_update_fields();
            // if let Some(agents) = &cache.agent_data {
            //     for agent in &agents.agents {
            //         if agent.label.eq(&cache.selected_agent) {
            //             cache.rtrs.clear();
            //             for entry in &agent.blob_entries {
            //                 if let Some(metadata) = &entry.metadata {
            //                     let mcof = crate::AgentConfigROS::from_json_b64(metadata);
            //                     cache.rtrs.insert(agent.label.to_string(),mcof);
            //                     cache.agent_entry = Some(entry.label.to_string());
            //                 }
            //             }
            //             cache.config_agent = cache.selected_agent.to_string();
            //         }
            //     }
            // }


// FIXME parse result to Vec<Agent> with metadata and Vec<BlobEntry_summary> populated
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_get_agent_entries_metadata(
  nvacl: &NavAbilityClient,
  agent_label: String,
  mime_type: Option<String>
) -> Result<get_agent_entries_metadata::ResponseData, Box<dyn Error>> {
  
  let mut mime_type_contains = Some("".to_string());
  if let Some(mt) = mime_type {
    mime_type_contains = Some(mt.to_string());
  }
  
  let variables = get_agent_entries_metadata::Variables {
    org_id: nvacl.user_label.to_string(),
    agent_label: agent_label.to_string(),
    mime_type_contains,
  };
  let request_body = GetAgentEntriesMetadata::build_query(variables);
  
  return post_to_nvaapi::<
  get_agent_entries_metadata::Variables,
  get_agent_entries_metadata::ResponseData,
  get_agent_entries_metadata::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn get_agent_entries_metadata_send(
  send_into: Sender<get_agent_entries_metadata::ResponseData>,
  nvacl: &NavAbilityClient,
  agent_label: String,
  mime_type: Option<String>
) -> Result<(), Box<dyn Error>> {
  
  return send_api_result(
    send_into, 
    post_get_agent_entries_metadata(
      nvacl, 
      agent_label,
      mime_type,
    ).await,
  );
}


// FIXME return Uuid (not string)
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_agent_entry(
  nvacl: &NavAbilityClient,
  agent_label: &String,
  entry: &BlobEntry,
  _legacy: Option<String>,
) -> Result<String,Box<dyn Error>> {
  //) -> Result<Response<crate::add_blob_entry_agent::ResponseData>, Box<dyn Error>> {
  
  let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
  let name = format!("{}{}",&agent_label,&entry.label).to_string();
  let entry_id = Uuid::new_v5(&org_id, name.as_bytes());
  
  let mut size_s: Option<String> = None;
  if let Some(sz) = entry.size {
    size_s = Some(format!("{}",sz));
  }
  let mut metadata = entry.metadata.to_string();
  if metadata.is_empty() {
    metadata = "e30=".to_string();
  }
  
  let variables = crate::add_blob_entry_agent::Variables {
    agent_label: agent_label.to_string(),
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
  
  let request_body = AddBlobEntryAgent::build_query(variables);
  
  return post_to_nvaapi::<
  crate::add_blob_entry_agent::Variables,
  crate::add_blob_entry_agent::ResponseData,
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

// let gqlentry = add_blob_entries_agent::BlobEntryCreateInput::new(
//     org_id,
//     entry,
//     agent_label.to_string(),
// );
// let mut blob_entries = Vec::new();
// blob_entries.push(gqlentry);
// let variables = add_blob_entries_agent::Variables {
//     blob_entries,
// };


#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn add_agent_entry_send(
  send_into: std::sync::mpsc::Sender<String>,
  nvacl: &NavAbilityClient,
  agent_label: &String,
  entry: &BlobEntry,
) -> Result<(),Box<dyn Error>> {
  
  return send_api_result(
    send_into, 
    post_add_agent_entry(
      nvacl, 
      agent_label,
      entry,
      None
    ).await,
  );
}


#[cfg(feature = "tokio")]
pub fn addAgentBlobEntry(
  nvacl: &NavAbilityClient,
  agent_label: &String,
  entry: &BlobEntry,
) -> Result<String, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(post_add_agent_entry(
    nvacl,
    agent_label,
    entry,
    None // legacy
  ));
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_update_agent_metadata(
  nvacl: &NavAbilityClient,
  agent_label: &String,
  metadata: &String,
) -> Result<String,Box<dyn Error>> {
  let variables = crate::update_agent_metadata::Variables {
    id: nvacl.getId(agent_label).to_string(),
    metadata: metadata.clone(),
  };
  
  let request_body = UpdateAgentMetadata::build_query(variables);
  
  return post_to_nvaapi::<
  crate::update_agent_metadata::Variables,
  crate::update_agent_metadata::ResponseData,
  String
  >(
    nvacl,
    request_body, 
    |s| {
      s.update_agents.agents[0].metadata.clone().unwrap_or("".to_string())
    },
    Some(1)
  ).await;
}


#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn update_agent_metadata_send(
  send_into: std::sync::mpsc::Sender<String>,
  nvacl: &NavAbilityClient,
  agent_label: &String,
  metadata: &String,
) -> Result<(),Box<dyn Error>> {
  
  return send_api_result(
    send_into, 
    post_update_agent_metadata(
      nvacl, 
      agent_label,
      metadata
    ).await,
  );
}


#[cfg(feature = "tokio")]
pub fn updateAgentMetadata(
  nvacl: &NavAbilityClient,
  agent_label: &String,
  metadata: &String,
) -> Result<String, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(post_update_agent_metadata(
    nvacl,
    agent_label,
    metadata
  ));
}