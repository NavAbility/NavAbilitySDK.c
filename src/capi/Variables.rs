


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

// use ::libc::{
//     // size_t,
//     // c_char, 
//     // c_double,
// };

// use ffi_convert::{
//     CReprOf,
//     CDrop,
//     // CArray,
//     // AsRust,
//     // CReprOfError,
// };

use crate::{
  cstr_to_str,
  convert_str,
  // parse_str_utc, 
  // RVec,
  // vec_to_ffi,
  to_console_error, 
  // Agent,
};




#[no_mangle] pub unsafe extern "C" 
fn getVariable(
  nvafg: Option<&crate::NavAbilityDFG>,
  label: *const c_char,
) -> Option<Box<crate::VariableDFG>> {
  if nvafg.is_none() {
    to_console_error("getVariable: provided *NavAbilityDFG is NULL/None");
    return None;
  }
  
  let vari = crate::services::getVariable(
    nvafg.unwrap(), 
    &cstr_to_str(label),
    true,
    // false,
  );
  
  match vari {
    Some(vr) => {
      return Some(Box::new(vr));
    },
    None => {
      return None
    }
  }
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn addVariable(
  nvafg: Option<&crate::NavAbilityDFG>,
  label: *const c_char,
  variableType: *const c_char,
  _tags: Option<Vec<*const c_char>>,
  _solvable: Option<i64>,
  _timestamp: Option<*const c_char>,
  _nstime: Option<usize>,
  _metadata: Option<*const c_char>,
) -> Option<*mut c_char> {
  if nvafg.is_none() {
    to_console_error("addVariable: provided *NavAbilityDFG is NULL/None");
    return None;
  }
  
  let tags = if _tags.is_none() {
    None
  } else {
    let mut v = Vec::new();
    for t in _tags.unwrap() {
      v.push(cstr_to_str(t).to_string());
    }
    Some(v)
  };
  let timestamp = if _timestamp.is_none() {
    None
  } else {
    Some(crate::parse_str_utc(
      cstr_to_str(
        _timestamp.unwrap()
      ).to_string()
    ).expect("addVariable not able to parse timestamp string"))
  };
  
  let vari = crate::services::addVariable(
    nvafg.unwrap(), 
    &cstr_to_str(label).to_string(),
    &cstr_to_str(variableType).to_string(),
    tags,
    _solvable,
    timestamp,
    if _nstime.is_none() {None} else {Some(_nstime.unwrap() as usize)},
    if _metadata.is_none() {None} else {Some(cstr_to_str(_metadata.unwrap()).to_string())}
  );
  
  match vari {
    Ok(id) => {
      return Some(convert_str(&id.to_string()));
    },
    Err(e) => {
      to_console_error(&format!("Problem with addVariable {:?}",e));
      return None
    }
  }
}
