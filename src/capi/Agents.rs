


// use uuid::Uuid;

use std::{
    ptr,
    // os::raw::{
    //     c_char,
    //     // c_void, 
    // },
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
    // get_agents, 
    parse_str_utc, 
    RVec,
    vec_to_ffi,
    to_console_error, 
    Agent,
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getAgents(
    _nvacl: Option<&crate::NavAbilityClient>,
) -> Option<Box<RVec<crate::Agent>>> {
    if _nvacl.is_none() {
        to_console_error("listAgents: provided *NavAbilityClient is NULL/None");
        return None;
    }

    match crate::services::getAgents(_nvacl.unwrap(), "".into()) {
        Ok(agents) => {
            return Some(Box::new(vec_to_ffi(agents)))
        }
        Err(e) => {
            to_console_error(&format!("NvaSDK.rs error during getAgents: {:?}", e));
            // return None;
            return Some(Box::new(RVec::<crate::Agent> { 
                ptr: ptr::null_mut(), 
                len: 0 as usize 
            }))
        }
    }
}