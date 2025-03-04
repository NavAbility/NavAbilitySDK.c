


use std::{
  ptr,
  os::raw::{
      c_char,
      // c_void, 
  },
};



use crate::{
  cstr_to_str,
  convert_str,
  RVec,
  vec_to_ffi,
  to_console_error, 
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
fn listVariables(
    _nvafg: Option<&crate::NavAbilityDFG>,
) -> Option<Box<RVec<String>>> {
  if _nvafg.is_none() {
    to_console_error("listVariables: provided *NavAbilityDFG is NULL/None");
    return None;
  }

  match crate::services::listVariables(_nvafg.unwrap()) {
    Ok(vari) => {
      return Some(Box::new(crate::vec_to_ffi(vari)));
    },
    Err(e) => {
      to_console_error(&format!("Problem with listVariables {:?}",e));
      return None;
    }
  }
}



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn addVariable(
  nvafg: Option<&crate::NavAbilityDFG>,
  label: *const c_char,
  variableType: *const c_char,
  _tags: *const c_char,
  _timestamp: *const c_char,
  _nstime: usize,
  _solvable: usize,
) -> *mut c_char {
  if nvafg.is_none() {
    to_console_error("addVariable: provided *NavAbilityDFG is NULL/None");
    return convert_str("");
  }
  
  let mut vtags = Vec::new();
  let tags = cstr_to_str(_tags).to_string();
  tags.split(";").for_each(|t| vtags.push(t.to_string()));

  let timestamp = crate::parse_utc_or_none(cstr_to_str(_timestamp).to_string());

  let vari = crate::services::addVariable(
    nvafg.unwrap(), 
    &cstr_to_str(label).to_string(),
    &cstr_to_str(variableType).to_string(),
    Some(vtags),
    timestamp,
    Some(_nstime),
    Some(_solvable as i64),
    None,
  );
  
  match vari {
    Ok(id) => {
      return convert_str(&id.to_string());
    },
    Err(e) => {
      to_console_error(&format!("Problem with addVariable {:?}",e));
      return convert_str("");
    }
  }
}
