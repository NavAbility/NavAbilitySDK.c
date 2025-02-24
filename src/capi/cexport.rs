


use ::core::slice;

use std::{
    ptr,
    os::raw::{
        c_char,
        // c_void, 
    },
    ffi::{
        CString,
        CStr
    },
};


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

use uuid::Uuid;

use crate::{
    // get_agents, 
    parse_str_utc, 
    to_console_error, 
    Agent, 
    NavAbilityBlobStore,
    GetLabel,
};


// ==================================================


// https://users.rust-lang.org/t/passing-vector-of-vectors-buffer-to-c/37345/9
// https://users.rust-lang.org/t/preparing-an-array-of-structs-for-ffi/33411
// Alt C style, https://users.rust-lang.org/t/how-to-return-byte-array-from-rust-function-to-ffi-c/18136/4
#[repr(C)]
#[derive(Debug)]
pub struct RVec<T> {
    pub ptr: *mut T,
    pub len: usize, // number of elems
}
// (24Q4, passed since safer_ffi was still experimental) 
// alt `use safer_ffi` https://users.rust-lang.org/t/pass-a-vec-from-rust-to-c/59184/6



// ================================ STATE ENTITY STRUCTS =================================


// ref. https://doc.rust-lang.org/std/boxed/
#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn NavAbilityClient_new(
    api_url: *const c_char,
    orgid: *const c_char,
    api_token: *const c_char, 
) -> Box<crate::NavAbilityClient> {
    let capi_url = CStr::from_ptr(api_url);
    let url = capi_url.to_str().expect("Bad encoding url");
    let oid_cstr = CStr::from_ptr(orgid);
    let oid = oid_cstr.to_str().expect("Bad encoding oid");
    let atk_cstr = CStr::from_ptr(api_token);
    let atk = atk_cstr.to_str().expect("Bad encoding atk");

    return Box::new(crate::NavAbilityClient::new(
        &url.to_owned(),
        &oid.to_owned(),
        &atk.to_owned(),
    ))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn NavAbilityDFG_new<'a>(
    _nvacl: Option<&'a crate::NavAbilityClient>,
    fgLabel: *const c_char,
    agentLabel: *const c_char,
    storeLabel: Option<&'a c_char>,
    addAgentIfAbsent: Option<&'a bool>,
    addGraphIfAbsent: Option<&'a bool>,
) -> Option<Box<crate::NavAbilityDFG>> {
    if _nvacl.is_none() {
        to_console_error("NavAbilityDFG: provided *NavAbilityClient is NULL/None");
        return None;
    }
    let nvacl = _nvacl.unwrap();

    let fgl = cstr_to_str(fgLabel);
    let agl = cstr_to_str(agentLabel);

    let mut _storeLabel = None; 
    if !(storeLabel.is_none()) {
        _storeLabel = Some(cstr_to_str(storeLabel.unwrap()));
    }

    return Some(Box::new(
        crate::NavAbilityDFG::new(
            nvacl,
            fgl,
            agl,
            _storeLabel,
            addAgentIfAbsent.copied(),
            addGraphIfAbsent.copied(),
        )
    ));
}



#[no_mangle] pub unsafe extern "C" 
fn get_apiurl(
    nvacl: Option<&crate::NavAbilityClient>,
) -> *mut c_char {
    if nvacl.is_none() {
        to_console_error("get_apiurl: provided *NavAbilityClient is NULL/None");
        return ptr::null_mut();
    }
    return convert_str(&(nvacl.unwrap()).apiurl);
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn NavAbilityBlobStore_new(
    nvacl: Option<&crate::NavAbilityClient>,
    label: *const c_char,
) -> Option<Box<crate::NavAbilityBlobStore>> {
    if nvacl.is_none() {
        to_console_error("NavAbilityBlobStore_new: provided *NavAbilityClient is NULL/None");
        return None;
    }
    return Some(Box::new(crate::NavAbilityBlobStore {
        client: nvacl.unwrap().clone(),
        label: cstr_to_str(label).to_string()
    }));
}



// ============================ ADDITIONAL UTILS ==================================


pub fn vec_to_ffi<T> (
    v: Vec<T>
) -> RVec<T> {
    // Going from Vec<_> to Box<[_]> just drops the (extra) `capacity`
    let boxed_slice: Box<[T]> = v.into_boxed_slice();
    let len = boxed_slice.len();
    let fat_ptr: *mut [T] =
        Box::into_raw(boxed_slice)
    ;
    let slim_ptr: *mut T = fat_ptr as _;
    RVec::<T> { ptr: slim_ptr, len }
}

// fn vec_to_ffi_ (
//     v: Vec<crate::Agent>
// ) -> RVec<crate::Agent> {
//     // Going from Vec<_> to Box<[_]> just drops the (extra) `capacity`
//     let boxed_slice: Box<[crate::Agent]> = v.into_boxed_slice();
//     let len = boxed_slice.len();
//     let fat_ptr: *mut [crate::Agent] =
//         Box::into_raw(boxed_slice)
//     ;
//     let slim_ptr: *mut crate::Agent = fat_ptr as _;
//     RVec::<crate::Agent> { ptr: slim_ptr, len }
// }


pub unsafe fn convert_str(input: &str) -> *mut c_char {
    let c_str = CString::new(input).unwrap().into_raw();
    return c_str;
}

pub unsafe fn cstr_to_str(c_buf: *const i8) -> &'static str {
    let cstr = CStr::from_ptr(c_buf);
    return cstr.to_str().expect("bad *const c_char encoding");
}


fn format_type_of<T>(_: &T) {
    format!("{}", std::any::type_name::<T>());
}



// ===================== NEW IDEAS =====================
// ===================== Traits =====================


// pub trait CGetLabel {
//     unsafe extern "C" fn get_label(&self) -> *const c_char;
// }

// impl CGetLabel for Agent {
//     unsafe extern "C" fn get_label(&self) -> *const c_char { self.label }
// }

// macro_rules! getLabel { 
//     ($T:ident) => {
//         #[no_mangle]
//         impl CGetLabel for $T {
//             fn get_label(&self) -> *const c_char { self.label }
//         }
//     }
// }

// macro_rules! getLabel {
//     (fn( $($arg:ty),* $(,)? ) $( -> $ret:ty )?, $name:ident) => {
//         impl CGetLabel for $T {
//             #[no_mangle]
//             unsafe extern "C" fn( $($arg),* ) $( -> $ret )? ;
//         }
//     };
// }

// getLabel!(Agent);



// ============================== PREVIOUS PATTERNS =============================


