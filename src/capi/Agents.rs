


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




#[no_mangle] pub unsafe extern "C" 
fn updateAgentMetadata(
    _nvacl: Option<&crate::NavAbilityClient>,
    agent_label: *const c_char,
    metadata: *const c_char,
) -> *const c_char {
    if _nvacl.is_none() {
        to_console_error("updateAgentMetadata: provided *NavAbilityClient is NULL/None");
        return convert_str("");
    }

    match crate::services::updateAgentMetadata(
        _nvacl.unwrap(),
        &cstr_to_str(agent_label).to_string(),
        &cstr_to_str(metadata).to_string(),
    ) {
        Ok(metadata) => {
            return convert_str(&metadata);
        }
        Err(e) => {
            to_console_error(&format!("NvaSDK.rs error during updateAgentMetadata: {:?}", e));
            return convert_str("");
        }
    }
}