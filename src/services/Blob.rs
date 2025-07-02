
use serde::Serialize;

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use base64::{
  Engine as _, 
  engine::general_purpose, 
  // read,
  // alphabet,
};

// use std::os::linux::raw;

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::{
  Utc,
  Uuid,
  GraphQLQuery,
  // Response,
  Error,
  Sender,
  // SDK_VERSION,
  NavAbilityClient,
  NavAbilityBlobStore,
  CreateDownload,
  create_download,
  CreateUpload,
  create_upload,
  CompleteUpload,
  complete_upload,
  DeleteBlob,
  delete_blob,
  post_to_nvaapi,
  send_api_result,
  // to_console_debug,
  to_console_error,
};



#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_create_download(
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  store: Option<String>,
) -> Result<create_download::ResponseData, Box<dyn Error>> {
  
  let variables = create_download::Variables {
    blob_id: blob_id.to_string(),
    store: store.unwrap_or("default".to_string()).to_string(),
  };
  
  let request_body = CreateDownload::build_query(variables);

  return post_to_nvaapi::<
    create_download::Variables,
    create_download::ResponseData,
    create_download::ResponseData
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}



#[cfg(any(feature = "tokio"))] // , feature = "thread"
pub fn q_createDownload(
  send_into: Sender<create_download::ResponseData>, 
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  store: Option<String>
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = nvacl.clone();
  let send_into_ = send_into.clone();
  let blob_id_ = blob_id.clone();
  let store_ = store.clone();
  crate::execute(async move {
    let _ = send_api_result(
      send_into_, 
      post_create_download(
        &nvacl_,
        blob_id_,
        store_
      ).await,
    );
  });
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_create_upload(
  nvacl: NavAbilityClient,
  // filename: String,
  // blob_size: i64,
  blob_id: Uuid,
  parts: Option<i64>,
) -> Result<create_upload::ResponseData, Box<dyn Error>> {
  
  let variables = create_upload::Variables {
    // label: label.to_string(),
    blob_id: blob_id.to_string(),
    parts: parts.unwrap_or(1),
  };
  
  let request_body = CreateUpload::build_query(variables);

  return post_to_nvaapi::<
    create_upload::Variables,
    create_upload::ResponseData,
    create_upload::ResponseData
  >(
    &nvacl,
    request_body, 
    |s| s,
    Some(1)
  ).await;
}


#[cfg(any(feature = "tokio"))] // , feature = "thread"
pub fn q_createUpload(
  send_into: Sender<create_upload::ResponseData>, 
  nvacl: &NavAbilityClient,
  _filename: &String,
  _blob_size: i64,
  nparts: Option<i64>,
  blob_id: Option<Uuid>, // doenst work yet, leave None
) -> Result<(), Box<dyn Error>> {
  return crate::execute(async { send_api_result(
      send_into, 
      post_create_upload(
          nvacl.clone(),
          blob_id.expect("Must provide blob_id to create_upload_send"),
          nparts
      ).await,
    )
  });
}

#[cfg(feature = "wasm")]
pub fn q_createUpload(
  send_into: Sender<create_upload::ResponseData>, 
  nvacl: &NavAbilityClient,
  _filename: &String,
  _blob_size: i64,
  nparts: Option<i64>,
  blob_id: Option<Uuid>, // doenst work yet, leave None
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = (*nvacl).clone();
  let send_into_ = send_into.clone();
  let blob_id_ = blob_id.clone();
  let nparts_ = nparts.clone();
  crate::execute(async move {
    let _ = send_api_result(
      send_into_, 
      post_create_upload(
        nvacl_.clone(),
        blob_id_.expect("Must provide blob_id to create_upload_send"),
        nparts_
      ).await,
    );
  });
}




// TODO update to new query/mutation pattern
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_complete_upload(
  nvacl: NavAbilityClient,
  blob_id: Uuid,
  upload_id: String,
  etags: Vec<String>,
  // completed_upload: complete_upload::CompletedUploadInput,
) -> Result<complete_upload::ResponseData, Box<dyn Error>> {
  let mut parts: Vec<Option<complete_upload::CompletedUploadPartInput>> = vec![];
  for (i,et) in etags.iter().enumerate() {
    parts.push(
      Some(
        complete_upload::CompletedUploadPartInput {
          part_number: (i + 1) as i64,
          e_tag: Some(et.to_string()),
        }
      )
    )
  }
  
  let cupl = complete_upload::CompletedUploadInput {
    upload_id: upload_id.to_string(),
    parts
  };
  
  let variables = complete_upload::Variables {
    blob_id: blob_id.to_string(),
    completed_upload: cupl,
  };
  
  let request_body = CompleteUpload::build_query(variables);
  
  return crate::post_to_nvaapi::<
    complete_upload::Variables,
    complete_upload::ResponseData,
    complete_upload::ResponseData
  >(
    &nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}




// TODO , feature = "blocking"
#[cfg(any(feature = "tokio", feature = "thread"))]
#[allow(non_snake_case)]
pub async fn post_blob_singlepart(
  nvabs: &NavAbilityBlobStore,
  blobId: Uuid,
  filename: &str,
  file_mime: &str,
  _file_timestamp: Option<&chrono::DateTime<Utc>>,
  file_bytes: std::sync::Arc<[u8]>,
) -> Result<(), Box<dyn Error>> {
  let _nvacl = &nvabs.client;
  let upl = post_create_upload(
    _nvacl.clone(), // change to allow borrow 
    blobId,
    Some(1) // one part upload
  ).await;
  
  // send the single part blob
  let upld = upl.unwrap();
  if let Some(crup) = upld.create_upload {
    let uploadId = &crup.upload_id.to_string();
    if let Some(st_url) = &crup.parts[0] {
      // let file = &cache.dropped_files[0];
      let filename = filename.to_string();
      let url = st_url.url.as_ref().unwrap().to_string();
      // if let Some(bytes_) = &file.bytes {
      let bytes = file_bytes.to_vec();
      let mut fu = crate::FileUploader::new(
        _nvacl.clone(),
        0,
        filename,
        blobId,
        Some(bytes.len() as u64),
      );
      let upload_result = fu.upload_file(
        bytes,
        url,
        file_mime.to_string(),
      ).await;
      
      let mut etags = Vec::new();
      match upload_result {
        Ok(et_res) => {
          etags.push(et_res);
        }
        Err(e) => {
          to_console_error(&format!("upload of part did not produce required eTag {:?}", &e));
        }
      }
      // handle complete upload
      let _ = post_complete_upload(
        _nvacl.clone(), 
        blobId.clone(), 
        uploadId.to_string(),
        etags,
      ).await;
    }
  }
  return Ok(());
}


#[derive(Serialize)]
#[allow(non_snake_case)]
struct PostOnPrem {
  storeLabel: String,
  blobId: String,
  input: String,
}


#[cfg(any(feature = "tokio", feature = "thread"))]
#[allow(non_snake_case)]
pub async fn post_blob_onprem(
  nvabs: &NavAbilityBlobStore,
  blobId: Uuid,
  _filename: &str,
  _file_mime: &str,
  _file_timestamp: Option<&chrono::DateTime<Utc>>,
  file_bytes: std::sync::Arc<[u8]>,
) -> Result<(), Box<dyn Error>> {

  let input = general_purpose::STANDARD.encode(file_bytes.to_vec());
  let request_body = crate::QueryBody::<PostOnPrem> {
    query: "addBlobFS",
    variables: PostOnPrem {
      storeLabel: "".to_owned(),
      blobId: blobId.to_string(),
      input,
    },
    operation_name: "addBlobFS",
  };
  
  let req_res = nvabs.client.client
  .post(&nvabs.client.apiurl)
  .json(&request_body)
  .send().await;

  if let Err(ref re) = req_res {
    to_console_error(&format!("Error in upload request to NavAbilityBlobStoreOnPrem: {:?}", re));
  }
  // TODO extract blobId from response and better error handling

  return Ok(());
}
// b64blob = base64encode(blob)
// response = NvaSDK.GQL.mutate(
//     store.client.client,
//     "addBlobFS",
//     Dict("storeLabel" => string(store.label), "blobId" => string(blobId), "input" => b64blob);
//     throw_on_execution_error = true,
// )
// blobId_str = response.data["addBlobFS"]
// blobId = tryparse(UUID, blobId_str)
// isnothing(blobId) && error(blobId_str)



// TODO , feature = "blocking"
#[cfg(any(feature = "tokio", feature = "thread"))]
#[allow(non_snake_case)]
pub async fn post_blob_store(
  nvabs: &NavAbilityBlobStore,
  blobId: Uuid,
  filename: &str,
  file_mime: &str,
  file_timestamp: &chrono::DateTime<Utc>,
  file_bytes: std::sync::Arc<[u8]>,
) -> Result<(), Box<dyn Error>> {
  match &nvabs.label {
    crate::NvaStoreLabel::Cloud(_store) => {
      return post_blob_singlepart(
        nvabs,
        blobId,
        filename,
        file_mime,
        Some(file_timestamp),
        file_bytes
      ).await;
    }
    crate::NvaStoreLabel::Onprem(_store) => {
      return post_blob_onprem(
        nvabs,
        blobId,
        filename,
        file_mime,
        Some(file_timestamp),
        file_bytes
      ).await;
    }
  }
}

// TODO , feature = "blocking"
#[cfg(any(feature = "tokio"))] // , feature = "thread"
#[allow(non_snake_case)]
pub fn addBlob(
  nvabs: NavAbilityBlobStore,
  blobId: Uuid,
  filename: &str,
  file_mime: &str,
  file_timestamp: &chrono::DateTime<Utc>,
  file_bytes: std::sync::Arc<[u8]>,
) {
  // TODO, multiple clones and likely unnecessary for non-wasm case
  let nvabs_ = nvabs.clone();
  // let nvacl = nvabs.nvacl.clone();
  let blobId_ = blobId.clone();
  let filename_ = filename.to_string();
  let mime_  = file_mime.to_string();
  let timestamp_ = file_timestamp.clone();
  let nbytes = (*file_bytes).len();
  let mut bytes = vec![0u8];
  bytes.resize(nbytes, 0x00);
  bytes[..nbytes].clone_from_slice(&file_bytes);

  let _ = crate::execute(crate::services::post_blob_store(
    &nvabs_,
    blobId_.clone(),
    &filename_,
    &mime_,
    &timestamp_,
    bytes.into(),
  ));
}



#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_delete_blob(
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  label: Option<&str>,
) -> Result<delete_blob::ResponseData, Box<dyn Error>> {
  
  // TODO use NvaBlobStore::cloud(lb) instead
  let mut store = "default".to_owned();
  if let Some(lb) = label {
    store = lb.to_owned();
  }
  
  let variables = delete_blob::Variables {
    blob_id: blob_id.to_string(),
    label: Some(store)
  };
  let request_body = DeleteBlob::build_query(variables);
  
  return crate::post_to_nvaapi::<
    delete_blob::Variables,
    delete_blob::ResponseData,
    delete_blob::ResponseData,
  >(
    nvacl,
    request_body, 
    |s| s,
    Some(3)
  ).await;
}



#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn delete_blob_send(
  send_into: std::sync::mpsc::Sender<delete_blob::ResponseData>,
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  label: Option<&str>,
) -> Result<(), Box<dyn Error>> {
  return crate::send_api_result(
    send_into, 
    post_delete_blob(nvacl, blob_id, label).await,
  );
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn deleteBlob(
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  label: Option<&str>,
) -> Result<delete_blob::ResponseData, Box<dyn Error>> {
  // // TODO
  // crate::execute( async {
  //   let _ = post_delete_blob(
  //     nvacl,
  //     blob_id,
  //     label,
  //   ).await;
  // });
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(
    post_delete_blob(
      nvacl,
      blob_id,
      label,
    )
  );
}


#[cfg(any(feature = "tokio", feature = "thread"))]
pub async fn download_blob(
  // nvacl: &NavAbilityClient,
  url: String
  // blob_id: Uuid,
  // store: Option<String>,
) -> Result<Vec<u8>, Box<dyn Error>> {
  
  // let dwurl  =post_create_download(nvacl, blob_id, store).await;
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN, reqwest::header::HeaderValue::from_static("*.amazonaws.com"));
  // headers.insert(CONTENT_TYPE, reqwest::header::HeaderValue::from_static("image/png"));

  // if let Ok(dw) = dwurl {
  //   if let Some(url) = dw.create_download {
    let client = reqwest::Client::new();
    let req_res = client
    .get(url)
    .headers(headers)
    // .header("Access-Control-Allow-Origin", "*.amazonaws.com")
    .send()
    .await;
    if let Err(ref re) = req_res {
      let msg = format!("Error in download request from NavAbilityBlobStore: {:?}", re);
      to_console_error(&msg);
      return Err(msg.into());
    }
    let bytes = req_res?.bytes().await?;
    return Ok(bytes.to_vec());
  //   }
  // }
}