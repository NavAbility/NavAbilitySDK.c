


#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::{
  Uuid,
  GraphQLQuery,
  // to_console_debug, 
  to_console_error, 
  // GetLabel, 
  NavAbilityClient,
  // services::Blob,
  BlobEntry,
  // parse_str_utc,
  FindModelBlobEntries,
  // Sender,
  Error,
  // find_model_blob_entries,
};


// #[macro_use]
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::{
  BlobEntryFieldsImporters,
  BlobEntry_importers,
};


#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::find_model_blob_entries::blobEntry_fields as FM_BlobEntryFields;
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
BlobEntry_importers!(FM_BlobEntryFields);


#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
pub async fn post_find_model_blob_entries(
  nvacl: &NavAbilityClient,
  model_label: &str,
  bentry_lbl_contains: &str,
) -> Result<Vec<BlobEntry>, Box<dyn Error>> {

  let variables = crate::find_model_blob_entries::Variables {
    model_label: Some(model_label.to_string()),
    be_label_contains: Some(bentry_lbl_contains.to_string()),
  };
  
  let request_body = FindModelBlobEntries::build_query(variables);
  
  return crate::post_to_nvaapi::<
    crate::find_model_blob_entries::Variables,
    crate::find_model_blob_entries::ResponseData,
    Vec<BlobEntry>
  >(
    nvacl,
    request_body, 
    |s| {
      let mut v = Vec::new();
      for md in &s.models {
        for be in &md.blob_entries {
          v.push(BlobEntry::from_gql(be));
        }
      }
      return v;
    },
    Some(3)
  ).await;
}


// initially used for wasm only, maybe relax for native, see SDK.rs for non-wasm examples
#[cfg(target_arch = "wasm32")]
pub fn q_findModelBlobEntries(
  send_into: crate::Sender<Vec<BlobEntry>>, 
  nvacl: &NavAbilityClient,
  model_label: &str,
  bentry_lbl_contains: &str,
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = nvacl.clone();
  let send_into_ = send_into.clone();
  let model_label_ = model_label.to_string();
  let bentry_lbl_contains_ = bentry_lbl_contains.to_string();
  crate::execute(async move {
    let _ = crate::send_api_result(
      send_into_, 
      post_find_model_blob_entries(
        &nvacl_, 
        &model_label_, 
        &bentry_lbl_contains_
      ).await,
    );
  });
}