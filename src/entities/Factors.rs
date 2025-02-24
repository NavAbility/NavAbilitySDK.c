

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PriorPoint2<T> {
    pub Z: T
}

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PriorPoint3<T> {
    pub Z: T
}


/// Create a PriorPose2 factor with a distribution Z representing the (x,y,th) relationship
/// between the variables, e.g. `FullNormal([1;0;0], diagm(0.01*ones(3)))`.
///
/// Example value: Z = `FullNormal(zeros(3), diagm(0.01*ones(3)))`.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PriorPose2<T> {
    pub Z: T
}


/// Create a PriorPose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
/// between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
///
/// Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PriorPose3<T> {
    pub Z: T
}


#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Point2Point2<T> {
    pub Z: T
}


#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Point3Point3<T> {
    pub Z: T
}


/// Create a Pose2->Pose2 factor with a distribution Z representing the (x,y,th) relationship
/// between the variables, e.g. `FullNormal([1;0;0], diagm(0.01*ones(3)))`.
///
/// Example value: Z = `FullNormal(zeros(3), diagm(0.01*ones(3)))`.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Pose2Pose2<T> {
    pub Z: T
}


/// Create a Pose3->Pose3 factor with a distribution Z representing the (x,y,z,a,b,c) relationship
/// between the variables, e.g. `FullNormal([1;zeros(5)], diagm(0.01*ones(6)))`.
///
/// Example value: Z = `FullNormal(zeros(6), diagm(0.01*ones(6)))`.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct Pose3Pose3<T> {
    pub Z: T
}
