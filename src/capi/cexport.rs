

use uuid::Uuid;

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

use ::core::slice;

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
    to_console_error, 
    Agent, 
    NavAbilityBlobStore,
    GetLabel,
};


// ==================================================


// https://users.rust-lang.org/t/passing-vector-of-vectors-buffer-to-c/37345/9
// https://users.rust-lang.org/t/preparing-an-array-of-structs-for-ffi/33411
#[repr(C)]
#[derive(Debug)]
pub struct RVec<T> {
    pub ptr: *mut T,
    pub len: usize, // number of elems
}
// alt use safer_ffi (still experimental) https://users.rust-lang.org/t/pass-a-vec-from-rust-to-c/59184/6



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


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn BlobEntry_basic(    
    label: *const c_char,
    mimeType: *const c_char,
) -> Box<crate::BlobEntry> {
        // let _id = cstr_to_str(id);    
        let mut be = crate::BlobEntry::new();
        be.blobId = Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("");
        be.label = cstr_to_str(label).to_string();
        be.blobstore = "default".to_owned();
        be.mimeType = cstr_to_str(mimeType).to_string();
        return Box::new(be)
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn BlobEntry_new(    
    blobId: *const c_char,
    label: *const c_char,
    blobstore: *const c_char,
    hash: *const c_char,
    origin: *const c_char,
    size: i64,
    description: *const c_char,
    mimeType: *const c_char,
    metadata: *const c_char,
    timestamp: *const c_char,
) -> Box<crate::BlobEntry> {
    // let _id = cstr_to_str(id);
    let _blobId = cstr_to_str(blobId);
    let _timestamp = cstr_to_str(timestamp).to_string();

    let mut be = crate::BlobEntry::new();
    be.blobId = Uuid::parse_str(_blobId).expect(&format!("new_BlobEntry unable to parse blobId uuid: {:?}",_blobId));
    be.label = cstr_to_str(label).to_string();
    be.blobstore = cstr_to_str(label).to_string();
    be.hash = cstr_to_str(hash).to_string();
    be.origin = cstr_to_str(origin).to_string();
    be.size = Some(size);
    be.description = cstr_to_str(description).to_string();
    be.mimeType = cstr_to_str(mimeType).to_string();
    be.metadata = cstr_to_str(metadata).to_string();
    be.timestamp = parse_str_utc(_timestamp.clone()).expect(&format!("new_BlobEntry unable to parse timestamp {:?}",_timestamp));
    return Box::new(be)
}


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


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn Pose3Pose3<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Pose3Pose3<crate::FullNormal<'a>>> {
    return Box::new(crate::Pose3Pose3::new(Z.clone()))
}


// ==================================== SERVICES LOGIC =======================================

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getAgents(
    _nvacl: Option<&crate::NavAbilityClient>,
) -> Option<Box<RVec<crate::Agent>>> {
    if _nvacl.is_none() {
        to_console_error("listAgents: provided *NavAbilityClient is NULL/None");
        return None;
    }

    match crate::services::getAgents(_nvacl.unwrap()) {
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
fn addAgentBlobEntry(
    _nvacl: Option<&crate::NavAbilityClient>,
    agent_label: *const c_char,
    _entry: Option<&crate::BlobEntry>,
) -> *const c_char {
    if _nvacl.is_none() {
        to_console_error("addAgentBlobEntry: provided *NavAbilityClient is NULL/None");
        return convert_str("");
    }

    match crate::services::addAgentBlobEntry(
        _nvacl.unwrap(),
        &cstr_to_str(agent_label).to_string(),
        _entry.unwrap()
    ) {
        Ok(id) => {
            return convert_str(&id);
        }
        Err(e) => {
            to_console_error(&format!("NvaSDK.rs error during addAgentBlobEntry: {:?}", e));
            return convert_str("");
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
        false,
        false,
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

// ========================= Accessors =======================

#[no_mangle] pub unsafe extern "C" 
fn length(rv_agent: &RVec<crate::Agent>) -> usize {
    return rv_agent.len
}

#[no_mangle] pub unsafe extern "C" 
fn get_index(
    rv_agent: &RVec<crate::Agent>,
    index: usize
) -> *mut crate::Agent {
    return rv_agent.ptr.wrapping_add(index)
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_Agent(
    agent: &crate::Agent,
) -> *const c_char {
    return convert_str(&((*agent).label));
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_BlobEntry(
    bentry: &crate::BlobEntry,
) -> *const c_char {
    return convert_str(&((*bentry).label));
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NavAbilityBlobStore(
    store: &crate::NavAbilityBlobStore,
) -> *const c_char {
    return convert_str(&((*store).label));
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NavAbilityClient(
    input: &crate::Agent,
) -> *const c_char {
    convert_str(&input.label)
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NvaNode<T>(
    input: &crate::NvaNode<T>,
) -> *const c_char {
    convert_str(&input.getLabel())
}

#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NavAbilityDFG(
    input: &crate::NavAbilityDFG,
) -> *const c_char {
    return convert_str(&(input.getLabel()));
}


// ============================== Free / Drop =================================

#[no_mangle] pub unsafe extern "C" 
fn free_cstr(pointer: *mut c_char) -> () {
    drop(CString::from_raw(pointer));
}

#[no_mangle] pub unsafe extern "C" 
fn free_rvecagent (
    rvec: Box<RVec<crate::Agent>>
) {
    free_rvec::<crate::Agent>(*rvec)
}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_VariableDFG(
    _: Option<Box<crate::VariableDFG>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_Pose3Pose3(
    _: Option<Box<crate::Pose3Pose3<crate::FullNormal>>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FullNormal(
    _: Option<Box<crate::FullNormal>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_BlobEntry(
    _: Option<Box<crate::BlobEntry>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_NavAbilityClient(
    _: Option<Box<crate::NavAbilityClient>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_NavAbilityDFG(
    _: Option<Box<crate::NavAbilityDFG>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_NavAbilityBlobStore(
    _: Option<Box<crate::NavAbilityBlobStore>>
) {}

// ===================== Drop overrides to see =====================

// impl Drop for crate::NavAbilityClient {
//     fn drop(&mut self) {
//         println!("See NavAbilityClient drop.");
//     }
// }

// impl Drop for crate::NavAbilityDFG<'_> {
//     fn drop(&mut self) {
//         println!("See NavAbilityDFG drop.");
//     }
// }

// impl Drop for crate::NavAbilityBlobStore {
//     fn drop(&mut self) {
//         println!("See NavAbilityBlobStore drop.");
//     }
// }

// impl Drop for crate::FullNormal<'_> {
//     fn drop(&mut self) {
//         println!("See FullNormal drop.");
//     }
// }

// impl<T> Drop for crate::Pose3Pose3<T> {
//     fn drop(&mut self) {
//         println!("See Pose3Pose3 drop.");
//     }
// }

// ============================ ADDITIONAL UTILS ==================================


fn vec_to_ffi<T> (
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

unsafe fn free_rvec<T> (
    rvec: RVec<T>
) {
    let ptr = rvec.ptr;
    let len = rvec.len;

    // println!("dropping RVec");
    if ptr.is_null() {
        eprintln!("free_rvec() errored: got NULL ptr!");
        ::std::process::abort();
        // return ();
    }
    let slice: &mut [T] =
        slice::from_raw_parts_mut(ptr, len)
    ;
    drop(Box::from_raw(slice));
}

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


// #[no_mangle] pub unsafe extern "C" 
// fn get_label(
//     agent: *mut crate::Agent,
// ) -> *mut c_char {
//     return convert_str(&((*agent).label))
// }

// pub struct TestType {
//     code: isize,
// }
// #[no_mangle]
// pub unsafe extern "C" fn error_create_with_result(_e: *mut *mut TestType) -> isize {
//     let e = Box::new(example_error());
//     *_e = Box::into_raw(e);
//     0
// }
// #[no_mangle]
// pub unsafe extern "C" fn error_free_with_result(e: *mut *mut TestType) -> i32 {
//     if e.is_null() || (*e).is_null() {
//         return libc::EINVAL;
//     }
//     // Reconstruct the Error into a box and then drop it so that it's freed.
//     drop(Box::from_raw(*e));
//     *e = 0 as *mut TestType;
//     0
// }
// // Our example "getter" methods which work on the Error type.  The value
// // returned is only valid as long as the Error has not been freed.  If C
// // caller needs a longer lifetime they need to copy the value.
// #[no_mangle]
// pub extern "C" fn error_code_get(e: &TestType) -> isize {
//     e.code
// }
// use libc;
