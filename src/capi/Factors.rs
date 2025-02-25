



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


macro_rules! GenFactorDFG_Type {
  ($T:ident, $fnm:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle] pub unsafe extern "C" 
        fn $fnm<'a>(
            varlbls: *const *const c_char,
            varlbls_len: usize,
            fnc: Option<&crate::$T<FullNormal<'a>>>,
        ) -> Option<Box<crate::FactorDFG<crate::$T<FullNormal<'a>>>>> {
            
            let mut ovlb = Vec::new();
            for i in 0..varlbls_len {
                let v = *varlbls.offset(i as isize);
                let s = CStr::from_ptr(v).to_string_lossy().into_owned();
                ovlb.push(s);
            }
            
            let f = crate::FactorDFG::new(
                ovlb, 
                fnc.unwrap().clone(),
                Vec::new(),
                None,
                None
            );

            return return Some(Box::new( f ));
        }
    };
}

// REMEMBER TO DUPLICATE IN SDKSupplemental -- 
//  TODO find another way to avoid implicit function definition warning
GenFactorDFG_Type!(PriorPoint2, FactorDFG_PriorPoint2_FullNormal_new);
GenFactorDFG_Type!(PriorPoint3, FactorDFG_PriorPoint3_FullNormal_new);
GenFactorDFG_Type!(PriorPose2, FactorDFG_PriorPose2_FullNormal_new);
GenFactorDFG_Type!(PriorPose3, FactorDFG_PriorPose3_FullNormal_new);
GenFactorDFG_Type!(Point2Point2, FactorDFG_Point2Point2_FullNormal_new);
GenFactorDFG_Type!(Point3Point3, FactorDFG_Point3Point3_FullNormal_new);
GenFactorDFG_Type!(Pose2Pose2, FactorDFG_Pose2Pose2_FullNormal_new);
GenFactorDFG_Type!(Pose3Pose3, FactorDFG_Pose3Pose3_FullNormal_new);


// #[allow(non_snake_case)]
// #[no_mangle] pub unsafe extern "C" 
// fn FactorDFG_Pose2Pose2_FullNormal_new<'a>(
//     varlbls: *const *const c_char,
//     varlbls_len: usize,
//     fnc: Option<&crate::Pose2Pose2<FullNormal<'a>>>,
// ) -> Option<Box<crate::FactorDFG<crate::Pose2Pose2<FullNormal<'a>>>>> {
//     todo!()
// }


//