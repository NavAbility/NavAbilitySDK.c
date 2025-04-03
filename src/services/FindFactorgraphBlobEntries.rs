

use std::{
  error::Error,
  sync::mpsc::Sender,
};

use graphql_client::GraphQLQuery;



use chrono::{
  Utc,
  ParseError
};
use uuid::Uuid;

#[macro_use]
use crate::{
  BlobEntryFieldsImporters,
  BlobEntry_importers,
  FindFactorgraphBlobEntries,
  NavAbilityClient,
  BlobEntry,
  parse_str_utc,
  to_console_error,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::find_factorgraph_blob_entries::blobEntry_fields as FG_BlobEntryFields;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
BlobEntry_importers!(FG_BlobEntryFields);



pub async fn post_find_factorgraph_blob_entries(
  nvacl: &NavAbilityClient,
  label: &str,
  bentry_lbl_contains: &str,
) -> Result<Vec<BlobEntry>, Box<dyn Error>> {

  let variables = crate::find_factorgraph_blob_entries::Variables {
    label: Some(label.to_string()),
    be_label_contains: Some(bentry_lbl_contains.to_string()),
  };
  
  let request_body = FindFactorgraphBlobEntries::build_query(variables);
  
  return crate::post_to_nvaapi::<
    crate::find_factorgraph_blob_entries::Variables,
    crate::find_factorgraph_blob_entries::ResponseData,
    Vec<BlobEntry>
  >(
    nvacl,
    request_body, 
    |s| {
      let mut v = Vec::new();
      for md in &s.factorgraphs {
        for be in &md.blob_entries {
          v.push(BlobEntry::from_gql(be));
        }
      }
      return v;
    },
    Some(3)
  ).await;
}

#[cfg(target_arch = "wasm32")]
pub async fn q_findFactorgraphBlobEntries(
  send_into: Sender<Vec<BlobEntry>>, 
  nvacl: &NavAbilityClient,
  label: &str,
  bentry_lbl_contains: &str,
) {

  let nvacl_ = nvacl.clone();
  let send_into_ = send_into.clone();
  let label_ = label.to_string();
  let bentry_lbl_contains_ = bentry_lbl_contains.to_string();
  crate::execute(async move {
    let _ = crate::send_api_result(
      send_into_, 
      post_find_factorgraph_blob_entries(
        &nvacl_, 
        &label_, 
        &bentry_lbl_contains_
      ).await,
    );
  });
}


