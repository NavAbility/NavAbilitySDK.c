


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
  Factorgraph 
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn listGraphs(
    _nvacl: Option<&crate::NavAbilityClient>,
) { //-> Option<Box<RVec<*mut c_char>>> {
  if _nvacl.is_none() {
    to_console_error("listGraphs: provided *NavAbilityClient is NULL/None");
    return ();
  }

  match crate::services::listGraphs(_nvacl.unwrap()) {
    Ok(graphs) => {
      for g in &graphs {
        crate::to_console_debug(&format!("got graph {}",g));
      }
      let mut vcs = Vec::new();
      for g in &graphs {
        vcs.push(convert_str(g));
      } 
      Some(Box::new(vec_to_ffi(vcs)));
    }
    Err(e) => {
      to_console_error(&format!("NvaSDK.rs error during listAgents: {:?}", e));
      // return None;
       Some(Box::new(RVec::<*mut c_char> { 
        ptr: ptr::null_mut(), 
        len: 0 as usize 
      }));
    }
  }


}