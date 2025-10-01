
use ::core::slice;

use std::{
    // ptr,
    os::raw::{
        c_char,
        c_double, 
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
    NvaNode,
    Factorgraph,
    NavAbilityBlobStore,
    NavAbilityDFG,
    GetLabel,
    convert_str,
};


// ========================= Accessors =======================


// ------------- Agent -----------------


#[no_mangle] pub unsafe extern "C" 
fn length_RVec_Agent(
    rv_agent: &RVec<crate::Agent>
) -> usize {
    return rv_agent.len
}


#[no_mangle] pub unsafe extern "C" 
fn getIndex_RVec_Agent(
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


// ------------- Factorgraph -----------------


#[no_mangle] pub unsafe extern "C" 
fn length_RVec_NvaNode_Factorgraph(
    rv_fgs: Option<&RVec<NvaNode<Factorgraph>>>
) -> usize {
    if rv_fgs.is_none() {
        return 0;
    }
    return rv_fgs.unwrap().len
}


#[no_mangle] pub unsafe extern "C" 
fn getIndex_RVec_NvaNode_Factorgraph(
    rv_fgs: &RVec<NvaNode<Factorgraph>>,
    index: usize
) -> *mut NvaNode<Factorgraph> {
    return rv_fgs.ptr.wrapping_add(index)
}



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NvaNode_Factorgraph(
    input: &NvaNode<Factorgraph>,
) -> *const c_char {
    convert_str(&input.getLabel())
}


// TODO see if this can be used with macro _Generic
// #[allow(non_snake_case)]
// #[no_mangle] pub unsafe extern "C" 
// fn getLabel_NvaNode<T>(
//     input: &NvaNode<T>,
// ) -> *const c_char {
//     convert_str(&input.getLabel())
// }


// ------------- Variables -----------------


#[no_mangle] pub unsafe extern "C" 
fn length_RVec_String(
    rv_s: Option<&RVec<String>>
) -> usize {
    if rv_s.is_none() {
        return 0;
    }
    return rv_s.unwrap().len
}

#[no_mangle] pub unsafe extern "C" 
fn getIndex_RVec_String(
    rv_s: &RVec<String>,
    index: usize
) -> *mut c_char {
    return convert_str(&(*(rv_s.ptr.wrapping_add(index))));
}


#[no_mangle] pub unsafe extern "C" 
fn length_RVec_f64(
    rv_s: Option<&RVec<c_double>>
) -> usize {
    if rv_s.is_none() {
        return 0;
    }
    return rv_s.unwrap().len;
}

#[no_mangle] pub unsafe extern "C" 
fn getIndex_RVec_f64(
    rv_s: &RVec<f64>,
    index: usize
) -> *const c_double {
    return rv_s.ptr.wrapping_add(index);
}



// ------------- BlobEntry -----------------


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_BlobEntry(
    bentry: &crate::BlobEntry,
) -> *const c_char {
    return convert_str(&((*bentry).label));
}


// ------------- BlobStore -----------------


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NavAbilityBlobStore(
    store: &crate::NavAbilityBlobStore,
) -> *const c_char {
    match &store.label {
        crate::NvaStoreLabel::Cloud(label) => {
            return convert_str(label);
        },
        crate::NvaStoreLabel::Onprem(label) => {
            return convert_str(label);
        },
    }
}


// ------------- Client / DFG -----------------


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn getLabel_NavAbilityClient(
    input: &crate::Agent,
) -> *const c_char {
    convert_str(&input.label)
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

#[no_mangle] pub unsafe extern "C" 
fn free_RVec_NvaNode_Factorgraph (
    rvec: Box<RVec<NvaNode<Factorgraph>>>
) {
    free_rvec::<NvaNode<Factorgraph>>(*rvec)
}

#[no_mangle] pub unsafe extern "C" 
fn free_RVec_f64 (
    rvec: Option<Box<RVec<f64>>>
) {
    if rvec.is_none() {
        return;
    }
    free_rvec::<f64>(*(rvec.unwrap()))
}

#[no_mangle] pub unsafe extern "C" 
fn free_RVec_String (
    rvec: Option<Box<RVec<String>>>
) {
    if rvec.is_none() {
        return;
    }
    free_rvec::<String>(*(rvec.unwrap()))
}

#[no_mangle] pub unsafe extern "C" 
fn free_StateValue (
    sv: Option<Box<crate::StateValue>>
) {}

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
fn free_FullNormal(
    _: Option<Box<crate::FullNormal>>
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
fn free_SubscriptionManager(
    _: Option<Box<crate::SubscriptionManager>>
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