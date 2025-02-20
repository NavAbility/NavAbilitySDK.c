
use crate::{
  BlobEntry, 
  Utc, 
  Uuid,
  SDK_VERSION
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  chrono::ParseError, 
  get_variable, 
  parse_str_utc, 
  post_to_nvaapi,
  check_query_response_data,
  send_api_result,
  // send_api_response,
  // send_query_result, 
  to_console_debug, 
  to_console_error, 
  update_blobentry_metadata, 
  Error, 
  Response, 
  UpdateBlobentryMetadata, 
  check_deser, 
  DeleteBlobEntry,
  delete_blob_entry,
  GetBlobEntry, 
  get_blob_entry, 
  get_agents,
  GraphQLQuery, 
  NavAbilityClient,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[macro_use]
use crate::{
  BlobEntrySummaryImporters,
  BlobEntry_importers_summary,
  BlobEntryFieldsImporters,
  BlobEntry_importers,
  SameBlobEntryFields, // DEPRECATING
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use get_blob_entry::blobEntry_fields as GB_BlobEntryFields;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use get_variable::blobEntry_fields as GV_BlobEntryFields;

// duplication in blobEntry_fields GQL fragments in different queries
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_importers!(GB_BlobEntryFields);
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_importers!(GV_BlobEntryFields);


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use get_agents::blobEntry_fields_summary as GA_BlobEntrySummary;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_importers_summary!(GA_BlobEntrySummary);



impl BlobEntry {
  pub fn new() -> Self {
    let mut be = BlobEntry::default();
    be.blobId = Uuid::new_v4();
    be.blobstore = "default".to_string();
    be.origin = "NvaSDK.rs".to_string();
    be.metadata = "e30=".to_string();
    be.createdTimestamp = Some(Utc::now());
    be.lastUpdatedTimestamp = be.createdTimestamp.clone();
    be._type = "BlobEntry".to_string(); // for self assemply typed usage elsewhere
    be._version = SDK_VERSION.to_string(); // FIXME dont hardcode, pull from common source
    return be
  }
  
  #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
  pub fn from_gql_summary(
    begql: &impl BlobEntrySummaryImporters,
  ) -> Self {
    let mut be = BlobEntry::default();
    be.id = begql.id();
    be.label = begql.label();
    be.size = begql.size();
    be.mimeType = begql.mimeType();
    be.lastUpdatedTimestamp = begql.lastUpdatedTimestamp();
    
    return be;
  }
  
  #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
  pub fn from_gql(
    begql: &impl BlobEntryFieldsImporters,
  ) -> Self {
    let mut be = BlobEntry::default();
    be.id = begql.id();
    be.blobId = begql.blobId();
    be.originId = begql.originId();
    be.label = begql.label();
    be.blobstore = begql.blobstore();
    be.hash = begql.hash();
    be.origin = begql.origin();
    be.size = begql.size();
    be.description = begql.description();
    be.mimeType = begql.mimeType();
    be.metadata = begql.metadata();
    be.timestamp = begql.timestamp().unwrap_or(Utc::now());
    be.createdTimestamp = begql.createdTimestamp();
    be.lastUpdatedTimestamp = begql.lastUpdatedTimestamp();        
    be._type = begql._type();
    be._version = begql._version();
    
    return be;
  }
  
  #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
  pub fn try_from_receiver(
    rx: &std::sync::mpsc::Receiver<Vec<BlobEntry>>, //get_blob_entry::ResponseData>
  ) -> Option<Vec<Self>> {
    
    match rx.try_recv() {
      Ok(gqle) => {
        return Some(gqle);
      }
      Err(_e) => {
        // to_console_debug(&"BlobEntry::try_from_receive nothing in channel");
      }
    }
    
    return None
  }
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_get_blob_entry(
  nvacl: &NavAbilityClient,
  id: Uuid
) -> Result<Vec<BlobEntry>, Box<dyn Error>> {
  
  let variables = get_blob_entry::Variables {
    entry_id: id.to_string(),
  };
  
  let request_body = GetBlobEntry::build_query(variables);
  
  return post_to_nvaapi::<
  get_blob_entry::Variables,
  get_blob_entry::ResponseData,
  Vec<BlobEntry>
  >(
    nvacl,
    request_body, 
    |s| {
      let mut bes = Vec::new();
      for be in &s.blob_entries {
        bes.push(BlobEntry::from_gql(be));
      }
      return bes
    },
    Some(1)
  ).await;
}
// Alt GQL input
// # BlobEntryCreateInput
// # Had difficulty with auto-gen BlobEntryCreateInput.parent
// # mutation AddBlobEntries(
// #   $blob_entries: [BlobEntryCreateInput!]!
// # ) {
// #   addBlobEntries(
// #     input: $blob_entries
// #   ) {
// #     blobEntries {
// #       ...blobEntry_fields
// #     }
// #   }
// # }



#[cfg(any(feature = "tokio", feature = "wasm"))]
pub async fn get_blob_entry_send(
  send_into: std::sync::mpsc::Sender<Vec<BlobEntry>>, //get_blob_entry::ResponseData>,
  nvacl: &NavAbilityClient,
  id: Uuid
) -> Result<(),Box<dyn Error>> {
  
  return send_api_result(
    send_into, 
    post_get_blob_entry(nvacl, id).await,
  );
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_delete_blobentry(
  nvacl: &NavAbilityClient,
  id: Uuid,
) -> Result<delete_blob_entry::ResponseData, Box<dyn Error>> {
  
  let variables = delete_blob_entry::Variables {
    id: id.to_string(),
  };
  let request_body = DeleteBlobEntry::build_query(variables);
  
  return post_to_nvaapi::<
  delete_blob_entry::Variables,
  delete_blob_entry::ResponseData,
  delete_blob_entry::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn delete_blobentry_send(
  send_into: std::sync::mpsc::Sender<delete_blob_entry::ResponseData>, //get_blob_entry::ResponseData>,
  nvacl: &NavAbilityClient,
  id: Uuid,
) -> Result<(), Box<dyn Error>> {
  return send_api_result(
    send_into, 
    post_delete_blobentry(nvacl, id).await,
  );
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn deleteBlobEntry(
  nvacl: &NavAbilityClient,
  id: Uuid,
) -> Result<delete_blob_entry::ResponseData, Box<dyn Error>> {
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(
    post_delete_blobentry(
      nvacl,
      id,
    )
  );
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_update_blobentry_metadata(
  nvacl: &NavAbilityClient,
  id: &Uuid,
  metadata_b64: &str
) -> Result<update_blobentry_metadata::ResponseData,Box<dyn Error>> {
  
  let variables = update_blobentry_metadata::Variables {
    id: id.to_string(),
    metadata: metadata_b64.to_string(),
  };
  
  let request_body = UpdateBlobentryMetadata::build_query(variables);
  
  return post_to_nvaapi::<
  update_blobentry_metadata::Variables,
  update_blobentry_metadata::ResponseData,
  update_blobentry_metadata::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}


// =============== FUTURE IDEAS ==============


// impl add_blob_entries::BlobEntryCreateInput {
//     pub fn new(
//         org_id: Uuid,
//         entry: &BlobEntry,
//         agent_label: String,
//     ) -> Self {

//         let name = format!("{}{}",&agent_label,&entry.label).to_string();
//         let id = Uuid::new_v5(&org_id, name.as_bytes());

//         let node = add_blob_entries::AgentWhere::new_agent(Some(agent_label.to_string()));

//         let agent_connect_where = add_blob_entries::AgentConnectWhere {
//             node: Box::new(node),
//         };

//         let agent = add_blob_entries::BlobEntryParentAgentFieldInput {
//             create: None,
//             connect_or_create: None,
//             connect: Some(add_blob_entries::BlobEntryParentAgentConnectFieldInput {
//                 connect: None,
//                 where_: Some(agent_connect_where),
//             })
//         };

//         let parent = add_blob_entries::BlobEntryParentCreateInput {
//             agent: Some(agent),
//             model: None,
//             factorgraph: None,
//             variable: None,
//             factor: None
//         };


//         Self {
//             id: id.to_string(),
//             origin_id: Some(entry.blobId.to_string()),
//             blob_id: entry.blobId.to_string(),
//             label: entry.label.to_string(),
//             blobstore: Some(entry.blobstore.to_string()),
//             origin: Some(entry.origin.to_string()),
//             description: Some(entry.description.to_string()),
//             mime_type: Some(entry.mimeType.to_string()),
//             hash: Some(entry.hash.to_string()),
//             size: entry.size,
//             metadata: Some(entry.metadata.to_string()),
//             timestamp: Some(entry.timestamp.to_string()),
//             // type_: entry._type.to_string(),
//             version: entry._version.to_string(),
//             parent: Some(parent),
//         }
//     }
// }
// 
// impl add_blob_entries::AgentWhere {
//     pub fn new_agent(
//         agent_label: Option<String>
//     ) -> Self {
//         Self {
//             id: None,
//             id_in: None,
//             id_contains: None,
//             id_starts_with: None,
//             id_ends_with: None,
//             org: Box::new(None),
//             org_aggregate: Box::new(None),
//             org_connection: Box::new(None),
//             org_connection_not: Box::new(None),
//             org_not: Box::new(None),
//             tags: None,
//             tags_includes: None,
//             version_ends_with: None,
//             version_starts_with: None,
//             version: None,
//             version_contains: None,
//             version_in: None,
//             and: Box::new(None),
//             not: Box::new(None),
//             or: Box::new(None),
//             blob_entries_aggregate: Box::new(None),
//             blob_entries_all: Box::new(None),
//             blob_entries_connection_all: Box::new(None),
//             blob_entries_connection_none: Box::new(None),
//             blob_entries_connection_single: Box::new(None),
//             blob_entries_connection_some: Box::new(None),
//             blob_entries_none: Box::new(None),
//             blob_entries_single: Box::new(None),
//             blob_entries_some: Box::new(None),
//             created_timestamp: None,
//             created_timestamp_gt: None,
//             created_timestamp_gte: None,
//             created_timestamp_in: None,
//             created_timestamp_lt: None,
//             created_timestamp_lte: None,
//             last_updated_timestamp: None,
//             last_updated_timestamp_gt: None,
//             last_updated_timestamp_gte: None,
//             last_updated_timestamp_lt: None,
//             last_updated_timestamp_lte: None,
//             last_updated_timestamp_in: None,
//             description: None,
//             description_contains: None,
//             description_ends_with: None,
//             description_in: None,
//             description_starts_with: None,
//             fgs_aggregate: Box::new(None),
//             fgs_all: Box::new(None),
//             fgs_single: Box::new(None),
//             fgs_connection_all: Box::new(None),
//             fgs_connection_single: Box::new(None),
//             fgs_connection_none: Box::new(None),
//             fgs_connection_some: Box::new(None),
//             fgs_none: Box::new(None),
//             fgs_some: Box::new(None),
//             label: agent_label,
//             label_in: None,
//             label_contains: None,
//             label_starts_with: None,
//             label_ends_with: None,
//             metadata: None,
//             metadata_in: None,
//             models_aggregate: Box::new(None),
//             models_all: Box::new(None),
//             models_none: Box::new(None),
//             models_some: Box::new(None),
//             models_single: Box::new(None),
//             models_connection_all: Box::new(None),
//             models_connection_none: Box::new(None),
//             models_connection_some: Box::new(None),
//             models_connection_single: Box::new(None),
//         }
//     }
// }




