
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Utc,
  Uuid,
  GraphQLQuery,
  Response,
  Error,
  Sender,
  SDK_VERSION,
  NavAbilityClient,
  CreateDownload,
  create_download,
  CreateUpload,
  create_upload,
  CompleteUpload,
  complete_upload,
  DeleteBlob,
  delete_blob,
  check_deser,
  send_query_result,
  to_console_debug,
  to_console_error,
};



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_create_download(
  nvacl: NavAbilityClient,
  blob_id: Uuid,
  store: Option<String>,
) -> Result<Response<create_download::ResponseData>, Box<dyn Error>> {
  
  let variables = create_download::Variables {
    blob_id: blob_id.to_string(),
    store: store.unwrap_or("default".to_string()).to_string(),
  };
  
  let request_body = CreateDownload::build_query(variables);
  
  let req_res = nvacl.client
  .post(&nvacl.apiurl)
  .json(&request_body)
  .send().await;
  
  if let Err(ref re) = req_res {
    to_console_error(&format!("API request error: {:?}", re));
  }
  
  return check_deser::<create_download::ResponseData>(
    req_res?.json().await
  )
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn create_download_send(
  send_into: Sender<create_download::ResponseData>,
  nvacl: NavAbilityClient,
  blob_id: Uuid,
  store: Option<String>
) {
  let resp = post_create_download(nvacl,blob_id,store).await;
  let _ = send_query_result::<
  create_download::ResponseData,
  create_download::ResponseData
  >(send_into,resp,|s| {s});
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_create_upload(
  nvacl: NavAbilityClient,
  // filename: String,
  // blob_size: i64,
  blob_id: Uuid,
  parts: Option<i64>,
) -> Result<Response<create_upload::ResponseData>, Box<dyn Error>> {
  
  let variables = create_upload::Variables {
    // label: label.to_string(),
    blob_id: blob_id.to_string(),
    parts: parts.unwrap_or(1),
  };
  
  let request_body = CreateUpload::build_query(variables);
  
  let req_res = nvacl.client
  .post(&nvacl.apiurl)
  .json(&request_body)
  .send().await;
  
  if let Err(ref re) = req_res {
    to_console_error(&format!("API request error: {:?}", re));
  }
  
  return check_deser::<create_upload::ResponseData>(
    req_res?.json().await
  )
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn create_upload_send(
  send_into: Sender<create_upload::ResponseData>, 
  client: &NavAbilityClient,
  filename: &String,
  blob_size: i64,
  nparts: Option<i64>,
  blob_id: Option<Uuid>, // doenst work yet, leave None
) {
  let result = post_create_upload(
    client.clone(), 
    blob_id.expect("Must provide blob_id to create_upload_send"),
    nparts,
  ).await;
  let _ = send_query_result(send_into, result, |s| {s});
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_complete_upload(
  nvacl: NavAbilityClient,
  blob_id: Uuid,
  upload_id: String,
  etags: Vec<String>,
  // completed_upload: complete_upload::CompletedUploadInput,
) -> Result<Response<complete_upload::ResponseData>, Box<dyn Error>> {
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
  
  let req_res = nvacl.client
  .post(&nvacl.apiurl)
  .json(&request_body)
  .send().await;
  
  if let Err(ref re) = req_res {
    to_console_error(&format!("API request error: {:?}", re));
  }
  
  return check_deser::<complete_upload::ResponseData>(
    req_res?.json().await
  )
}




// TODO , feature = "blocking"
#[cfg(any(feature = "tokio", feature = "wasm"))]
#[allow(non_snake_case)]
pub async fn post_blob_singlepart(
  _nvacl: &NavAbilityClient,
  blobId: Uuid,
  filename: &str,
  file_mime: &str,
  file_timestamp: &chrono::DateTime<Utc>,
  file_bytes: std::sync::Arc<[u8]>,
) {
  let upl = post_create_upload(
    _nvacl.clone(), // change to allow borrow 
    blobId,
    Some(1) // one part upload
  ).await;
  
  // send the single part blob
  let upld = upl.unwrap().data.unwrap();
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
        url
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
}




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_delete_blob(
  nvacl: &NavAbilityClient,
  blob_id: Uuid,
  label: Option<&str>,
) -> Result<delete_blob::ResponseData, Box<dyn Error>> {
  
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



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
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

