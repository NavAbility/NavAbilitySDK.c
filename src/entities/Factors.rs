
use std::ptr::null;

use uuid::Uuid;
use chrono::{
    DateTime,
    Utc
};


use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FunctionData {
  pub eliminated:  Option<bool>,
  pub potialused: Option<bool>,
  pub edgeIDs: Option<Vec<i64>>,
  pub fnc: String,
  pub multihypo: Vec<f64>,
  pub certainhypo: Option<Vec<i64>>,
  pub nullhypo: f64,
  pub solveInProgress: Option<i64>,
  pub inflation: f64
}


#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct FactorDFG<F> {
  pub id: Option<Uuid>,
  pub label: String,
  pub tags: Vec<String>,
  pub variableOrderSymbols_: Vec<String>,
  pub timestamp: Option<DateTime<Utc>>,
  pub nstime: Option<String>,
  pub fnctype: F,
  pub solvable: Option<i64>,
  pub data: Option<String>,
  pub metadata: Option<String>,
  pub _version: Option<String>
}



#[allow(non_snake_case)]
pub trait FactorType<'a, D: crate::Distribution<'a>> {
  fn new(Z: D) -> Self;
  fn type_str(&self) -> String;
}




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
