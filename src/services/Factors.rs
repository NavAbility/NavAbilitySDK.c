
use crate::{
    FullNormal,
    PriorPoint2,
    PriorPoint3,
    PriorPose2,
    PriorPose3,
    Point2Point2,
    Point3Point3,
    Pose2Pose2,
    Pose3Pose3,
};



#[allow(non_snake_case)]
impl<'a> PriorPoint2<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}



#[allow(non_snake_case)]
impl<'a> PriorPoint3<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> PriorPose2<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> PriorPose3<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> Point2Point2<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> Point3Point3<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> Pose2Pose2<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}


#[allow(non_snake_case)]
impl<'a> Pose3Pose3<FullNormal<'a>> {
    pub fn new(
        Z: FullNormal<'a>
    ) -> Self {
        Self {
            Z: Z
        }
    }
}
