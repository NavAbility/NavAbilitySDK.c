


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
    label_contains: *const c_char,
) -> Option<Box<RVec<crate::Agent>>> {
    if _nvacl.is_none() {
        to_console_error("getAgents: provided *NavAbilityClient is NULL/None");
        return None;
    }

    let lbl_cont = cstr_to_str(label_contains);
    match crate::services::getAgents(_nvacl.unwrap(), lbl_cont.into()) {
        Ok(agents) => {
            return Some(Box::new(vec_to_ffi(agents)))
        }
        Err(e) => {
            to_console_error(&format!("NvaSDK.c error during getAgents: {:?}", e));
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
            to_console_error(&format!("NvaSDK.c error during updateAgentMetadata: {:?}", e));
            return convert_str("");
        }
    }
}



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getAgentMetadata(
    _nvacl: Option<&crate::NavAbilityClient>,
    label: *const c_char,
) -> *const c_char {
    if _nvacl.is_none() {
        to_console_error("getAgents: provided *NavAbilityClient is NULL/None");
        return convert_str("");
    }

    let lbl_cont = cstr_to_str(label);
    match crate::services::getAgentMetadata(_nvacl.unwrap(), lbl_cont.into()) {
        Ok(metadata) => {
            return convert_str(&metadata);
        }
        Err(e) => {
            to_console_error(&format!("NvaSDK.c error during getAgentMetadata: {:?}", e));
            return convert_str("");
        }
    }
}