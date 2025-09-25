

use uuid::Uuid;

use std::{
  ptr,
  os::raw::{
    c_char,
    // c_void, 
  },
  // ffi::{
  //     CString,
  //     CStr
  // },
};

// use ::core::slice;

use crate::{
  convert_str,
  cstr_to_str,
  parse_str_utc,
  to_console_error,
  GetId,
};



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn BlobEntry_basic(    
  label: *const c_char,
  mimeType: *const c_char,
) -> Box<crate::BlobEntry> {
  // let _id = cstr_to_str(id);    
  let mut be = crate::BlobEntry::new();
  be.blobId = Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("");
  be.label = cstr_to_str(label).to_string();
  be.blobstore = "default".to_owned();
  be.mimeType = cstr_to_str(mimeType).to_string();
  return Box::new(be)
}



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_BlobEntry(
  blobId: *const c_char,
  label: *const c_char,
  blobstore: *const c_char,
  origin: *const c_char,
  size: i64,
  description: *const c_char,
  mimeType: *const c_char,
  metadata: *const c_char,
  timestamp: *const c_char,
) -> Box<crate::BlobEntry> {
  // let _id = cstr_to_str(id);
  let _blobId = cstr_to_str(blobId);
  let _timestamp = crate::parse_utc_or_none(cstr_to_str(timestamp).to_string()).unwrap_or(chrono::Utc::now());
  
  let mut be = crate::BlobEntry::new();
  be.blobId = Uuid::parse_str(_blobId).expect(&format!("new_BlobEntry unable to parse blobId uuid: {:?}",_blobId));
  be.label = cstr_to_str(label).to_string();
  be.blobstore = cstr_to_str(blobstore).to_string();
  be.hash = "".to_owned();
  be.origin = cstr_to_str(origin).to_string();
  be.size = Some(size);
  be.description = cstr_to_str(description).to_string();
  be.mimeType = cstr_to_str(mimeType).to_string();
  be.metadata = cstr_to_str(metadata).to_string();
  be.timestamp = _timestamp;
  return Box::new(be)
}



#[no_mangle] pub unsafe extern "C" 
fn addAgentBlobEntry(
  nvacl_: Option<&crate::NavAbilityClient>,
  agent_label: *const c_char,
  entry_: Option<&crate::BlobEntry>,
) -> *const c_char {
  if nvacl_.is_none() {
    to_console_error("addAgentBlobEntry: provided *NavAbilityClient is NULL/None");
    return convert_str("");
  }
  
  match crate::services::addAgentBlobEntry(
    nvacl_.unwrap(),
    &cstr_to_str(agent_label).to_string(),
    entry_.unwrap()
  ) {
    Ok(id) => {
      return convert_str(&id);
    }
    Err(e) => {
      to_console_error(&format!("NvaSDK.rs error during addAgentBlobEntry: {:?}", e));
      return convert_str("");
    }
  }
}


#[no_mangle] pub unsafe extern "C" 
fn addVariableBlobEntry(
  nvafg_: Option<&crate::NavAbilityDFG>,
  variable_label: *const c_char,
  entry_: Option<&crate::BlobEntry>,
) -> *mut c_char {
  if nvafg_.is_none() {
    to_console_error("addVariableBlobEntry: provided *NavAbilityDFG is NULL/None");
    return convert_str("");
  }
  
  match crate::services::addVariableBlobEntry(
    nvafg_.unwrap(),
    &cstr_to_str(variable_label).to_string(),
    entry_.unwrap()
  ) {
    Ok(id) => {
      return convert_str(&id.to_string());
    }
    Err(e) => {
      to_console_error(&format!("NvaSDK.rs error during addVariableBlobEntry: {:?}", e));
      return convert_str("");
    }
  }
}


// TODO upstream the various BlobEntry parent function to services
#[no_mangle] pub unsafe extern "C" 
fn deleteAgentBlobEntry(
  nvacl_: Option<&crate::NavAbilityClient>,
  agent_label: *const c_char,
  bentry_label: *const c_char,
) {
  if nvacl_.is_none() {
    to_console_error("addAgentBlobEntry: provided *NavAbilityClient is NULL/None");
    return ();
  }

  let nvacl = nvacl_.unwrap();
  let aglb = cstr_to_str(agent_label);
  let belb = cstr_to_str(bentry_label);

  let beid = nvacl.getId(&(aglb.to_string() + belb));

  let _ = crate::services::deleteBlobEntry(
    nvacl,
    beid,
  );

  ()
}

