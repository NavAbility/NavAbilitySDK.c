
use ::core::slice;

use std::{
    // ptr,
    os::raw::{
        c_char,
        // c_void, 
    },
    ffi::{
        CString,
        // CStr
    },
};

use crate::{
    RVec,
    Agent, 
    BlobEntry,
    NavAbilityBlobStore,
    NavAbilityDFG,
    GetLabel,
    convert_str,
};


// ========================= Accessors =======================

#[no_mangle] pub unsafe extern "C" 
fn length(rv_agent: &RVec<crate::Agent>) -> usize {
    return rv_agent.len
}

#[no_mangle] pub unsafe extern "C" 
fn getIndex_Agent(
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


pub unsafe fn free_rvec<T> (
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


#[no_mangle] pub unsafe extern "C" 
fn free_cstr(pointer: *mut c_char) -> () {
    drop(CString::from_raw(pointer));
}

#[no_mangle] pub unsafe extern "C" 
fn free_RVec_Agent (
    rvec: Box<RVec<crate::Agent>>
) {
    free_rvec::<crate::Agent>(*rvec)
}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_Agent(
    _: Option<Box<crate::Agent>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_VariableDFG(
    _: Option<Box<crate::VariableDFG>>
) {}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_PriorPoint2(
    _: Option<Box<crate::PriorPoint2<crate::FullNormal>>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_PriorPoint3(
    _: Option<Box<crate::PriorPoint3<crate::FullNormal>>>
) {}

// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_PriorPose2(
    _: Option<Box<crate::PriorPose2<crate::FullNormal>>>
) {}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_PriorPose3(
    _: Option<Box<crate::PriorPose3<crate::FullNormal>>>
) {}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_Point2Point2(
    _: Option<Box<crate::Point2Point2<crate::FullNormal>>>
) {}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_Point3Point3(
    _: Option<Box<crate::Point3Point3<crate::FullNormal>>>
) {}


// Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_Pose2Pose2(
    _: Option<Box<crate::Pose2Pose2<crate::FullNormal>>>
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