



use ::core::slice;

use std::{
  ptr,
  os::raw::{
      c_char,
      // c_void, 
  },
  ffi::{
  //     CString,
      CStr
  },
};



use crate::{
  FullNormal,
  entities::Factors::FactorType,
  cstr_to_str,
  convert_str,
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn deleteFactor(
  nvafg: Option<&crate::NavAbilityDFG>,
  label: *const c_char,
) -> i64 {
    if nvafg.is_none() {
        eprintln!("deleteFactor: provided *NavAbilityDFG is NULL/None");
        return -1 as i64;
    }

    let label_str = cstr_to_str(label);
    let idr = crate::services::deleteFactor(
        nvafg.unwrap(), 
        label_str
    );

    if let Err(ref e) = idr {
        eprintln!("deleteFactor error: {}", e);
    }
    match idr {
        Ok(id) => id as i64, // Return the id of the deleted factor
        Err(_) => -1 as i64, // Return 0 on error
    }
}



#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_PriorPoint2<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPoint2<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPoint2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_PriorPoint3<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPoint3<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPoint3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_PriorPose2<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPose2<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPose2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_PriorPose3<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPose3<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPose3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_Point2Point2<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Point2Point2<crate::FullNormal<'a>>> {
    return Box::new(crate::Point2Point2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_Point3Point3<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Point3Point3<crate::FullNormal<'a>>> {
    return Box::new(crate::Point3Point3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_Pose2Pose2<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Pose2Pose2<crate::FullNormal<'a>>> {
    return Box::new(crate::Pose2Pose2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn new_Pose3Pose3<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Pose3Pose3<crate::FullNormal<'a>>> {
    return Box::new(crate::Pose3Pose3::new(Z.clone()))
}


macro_rules! GenFactorDFG_Type {
  ($T:ident, $fnm:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle] pub unsafe extern "C" 
        fn $fnm<'a>(
            nvafg: Option<&crate::NavAbilityDFG>,
            _vlbls: *const c_char,
            fnc: Option<&crate::$T<FullNormal<'a>>>,
            _tags: *const c_char,
            _timestamp: *const char,
            _nstime: usize,
            _solvable: usize
        ) -> *const c_char {

            let mut vvlbls = Vec::new();
            let vlbls = cstr_to_str(_vlbls).to_string();
            vlbls.split(";").for_each(|t| {
                let t_ = t.trim();
                if !t_.is_empty() { vvlbls.push(t_.to_string()) }
            });

            let mut vtags = Vec::new();
            let tags = cstr_to_str(_tags).to_string();
            tags.split(";").for_each(|t| {
                let t_ = t.trim();
                if !t_.is_empty() { vtags.push(t_.to_string()) }
            });
            
            // let timestamp = (|ts: String| {
            //     if ts.is_empty() {
            //       return None;
            //     } else {
            //       return Some(crate::parse_str_utc(ts)
            //         .expect("addVariable not able to parse timestamp string"));
            //     }
            // })(cstr_to_str(_timestamp).to_string());

            // level 3 factor object with data
            let f = crate::FactorDFG::new(
                vvlbls, 
                fnc.unwrap().clone(),
                Vec::new(),
                None,
                None
            );

            // Do the add factor call here
            let idr = crate::services::addFactor(
                nvafg.unwrap(), 
                f
            );


            // return Some(Box::new( f ));
            if let Ok(id) = idr {
                return convert_str(&id.to_string());
            } else {
                return convert_str("ERROR");
            }
        }
    };
}
// ) -> Option<Box<crate::FactorDFG<crate::$T<FullNormal<'a>>>>> {


// REMEMBER TO DUPLICATE IN SDKSupplemental -- 
//  TODO find another way to avoid implicit function definition warning
GenFactorDFG_Type!(PriorPoint2, add_FactorDFG_PriorPoint2_FullNormal);
GenFactorDFG_Type!(PriorPoint3, add_FactorDFG_PriorPoint3_FullNormal);
GenFactorDFG_Type!(PriorPose2,  add_FactorDFG_PriorPose2_FullNormal);
GenFactorDFG_Type!(PriorPose3,  add_FactorDFG_PriorPose3_FullNormal);
GenFactorDFG_Type!(Point2Point2,add_FactorDFG_Point2Point2_FullNormal);
GenFactorDFG_Type!(Point3Point3,add_FactorDFG_Point3Point3_FullNormal);
GenFactorDFG_Type!(Pose2Pose2,  add_FactorDFG_Pose2Pose2_FullNormal);
GenFactorDFG_Type!(Pose3Pose3,  add_FactorDFG_Pose3Pose3_FullNormal);


// // Take ownership via passing by value, i.e. runs drop on fn exit. Option for null case.
// #[allow(non_snake_case)]
// #[no_mangle] pub extern "C" 
// fn free_FactorDFG_Pose3Pose3_FullNormal(
//     _: Option<Box<crate::FactorDFG<crate::Pose3Pose3<crate::FullNormal<'_>>>>>
// ) {}




#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_PriorPoint2_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::PriorPoint2<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_PriorPoint3_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::PriorPoint3<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_PriorPose2_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::PriorPose2<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_PriorPose3_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::PriorPose3<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_Point2Point2_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::Point2Point2<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_Point3Point3_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::Point3Point3<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_Pose2Pose2_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::Pose2Pose2<crate::FullNormal<'_>>>>>
) {}

#[allow(non_snake_case)]
#[no_mangle] pub extern "C" 
fn free_FactorDFG_Pose3Pose3_FullNormal(
    _: Option<Box<crate::FactorDFG<crate::Pose3Pose3<crate::FullNormal<'_>>>>>
) {}




//
