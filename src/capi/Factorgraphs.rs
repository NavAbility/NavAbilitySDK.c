


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


