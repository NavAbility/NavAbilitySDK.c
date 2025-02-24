



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


use crate::{
  FullNormal,
};


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn PriorPoint2_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPoint2<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPoint2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn PriorPoint3_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPoint3<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPoint3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn PriorPose2_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPose2<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPose2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn PriorPose3_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::PriorPose3<crate::FullNormal<'a>>> {
    return Box::new(crate::PriorPose3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn Point2Point2_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Point2Point2<crate::FullNormal<'a>>> {
    return Box::new(crate::Point2Point2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn Point3Point3_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Point3Point3<crate::FullNormal<'a>>> {
    return Box::new(crate::Point3Point3::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn Pose2Pose2_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Pose2Pose2<crate::FullNormal<'a>>> {
    return Box::new(crate::Pose2Pose2::new(Z.clone()))
}


#[allow(non_snake_case)]
#[no_mangle] pub unsafe extern "C" 
fn Pose3Pose3_new<'a>(
    Z: &crate::FullNormal<'a>,
) -> Box<crate::Pose3Pose3<crate::FullNormal<'a>>> {
    return Box::new(crate::Pose3Pose3::new(Z.clone()))
}


