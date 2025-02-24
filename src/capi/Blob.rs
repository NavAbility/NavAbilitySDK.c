

use chrono::Utc;
use uuid::Uuid;

use core::slice;

use std::{
    ptr,
    os::raw::{
        c_char,
        c_uchar,
        // c_void, 
    },
    // ffi::{
    //     CString,
    //     CStr
    // },
};

use crate::{
    convert_str,
    cstr_to_str,
    parse_str_utc,
    to_console_error,
    to_console_debug
};

// use crate::post_blob_singlepart;
// use ::safer_ffi::prelude::*;

fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
  // ideally we'd use Vec::into_raw_parts, but it's unstable,
  // so we have to do it manually:

  // first, make sure v's destructor doesn't free the data
  // it thinks it owns when it goes out of scope
  let mut v = std::mem::ManuallyDrop::new(v);

  // then, pick apart the existing Vec
  let p = v.as_mut_ptr();
  let len = v.len();
  let cap = v.capacity();
  
  // finally, adopt the data into a new Vec
  unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn addBlob(
  nvacl_: Option<&crate::NavAbilityClient>,
  filename: *const c_char,
  mime: *const c_char,
  data: *const c_char,
  nbytes: usize,
) -> *const c_char {
  if nvacl_.is_none() {
    to_console_error("addBlob: the provided *NavAbilityClient is NULL");
    return convert_str("");
  }

  let bytes = slice::from_raw_parts(data, nbytes);

  let nvacl = (*nvacl_.unwrap()).clone();

  let blobId = Uuid::new_v4();
  let blobId_ = blobId.clone();
  let retbid = blobId.to_string();
  let filename_ = cstr_to_str(filename); 
  let mime_ = cstr_to_str(mime);
  let timestamp = Utc::now();
  // use bytes; if ownership is required, then:
  let owned_bytes = bytes.to_vec();
  let data_bytes: std::sync::Arc<[u8]> = vec_i8_into_u8(owned_bytes).into();

  crate::services::addBlob(
    nvacl,
    blobId,
    filename_,
    mime_,
    &timestamp,
    data_bytes
  );

  return convert_str(&retbid);
}
