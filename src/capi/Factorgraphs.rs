


// use uuid::Uuid;

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


use crate::{
  cstr_to_str,
  convert_str,
  RVec,
  vec_to_ffi,
  to_console_error,
  NvaNode,
  Factorgraph 
};




#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getFactorgraphs(
  _nvacl: Option<&crate::NavAbilityClient>,
  label_contains: *const c_char,
) -> Option<Box<RVec<NvaNode<Factorgraph>>>> {
  if _nvacl.is_none() {
    to_console_error("getFactorgraphs: provided *NavAbilityClient is NULL/None");
    return None;
  }
  
  let lbl_cont = cstr_to_str(label_contains);
  match crate::services::getFactorgraphs(_nvacl.unwrap(), lbl_cont.into()) {
    Ok(fgs) => {
      return Some(Box::new(vec_to_ffi(fgs)));
    }
    Err(e) => {
      to_console_error(&format!("NvaSDK.c error during getFactorgraphs: {:?}", e));
      // return None;
      return Some(
        Box::new(RVec::<NvaNode<Factorgraph>> { 
            ptr: ptr::null_mut(),
            len: 0 as usize 
          }
        )
      );
    }
  }
}