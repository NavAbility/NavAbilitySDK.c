

use ::core::slice;

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





#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn FullNormal_new<'a>(
    dim: usize,
    array_mean: *const libc::c_double,
    array_covr: *const libc::c_double,
) -> Box<crate::FullNormal<'a>> {
    // https://stackoverflow.com/a/29183118
    let normal = crate::FullNormal {
        mean: std::slice::from_raw_parts(array_mean as *const f64, dim as usize),
        covr: std::slice::from_raw_parts(array_covr as *const f64, dim*dim as usize)
    };

    return Box::new(normal)
}
