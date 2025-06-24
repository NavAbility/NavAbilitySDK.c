
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use std::{
  sync::mpsc::Sender,
  error::Error,
  // any::type_name
};

use serde::Serialize;
use uuid::Uuid;

use chrono::{
  DateTime, 
  Utc
};

use base64::{
  // alphabet,
  engine::general_purpose,
  Engine as _,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  to_console_error,
  // type_of,
  GraphQLQuery,
  GetId,
  AddFactors,
  add_factors,
  send_api_result,
  SDK_VERSION,
  common_traits::GetLabel,
};

use crate::{
  entities::Distributions::Distribution, 
  FullNormal, 
  entities::Factors::{FactorDFG, FunctionData},
  // FullNormal, 
  Point2Point2, 
  Point3Point3, 
  Pose2Pose2, 
  Pose3Pose3, 
  PriorPoint2, 
  PriorPoint3,
  PriorPose2, PriorPose3,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::entities::ClientDFG::NavAbilityDFG;


// helper macro to avoid repetition of "basic" impl Coordinates
// TODO factors can come from more than just RoME
#[macro_export]
macro_rules! GenDistrFactor { 
  ($T:ident, $fns:literal) => {
    impl<'a, D: Distribution<'a>> crate::FactorType<'a, D> for $T<D> {
      fn new(Z: D) -> Self {
        Self {
          Z
        }
      }

      fn type_str(&self) -> String {
        return format!("{}", $fns); //get_fnc_name(&std::any::type_name::<Self>()));
      }

      fn pack(&self) -> String {
        return self.Z.to_json();
      }
    }
  }
}


GenDistrFactor!(PriorPoint2, "PriorPoint2");
GenDistrFactor!(PriorPoint3, "PriorPoint3");
GenDistrFactor!(PriorPose2, "PriorPose2");
GenDistrFactor!(PriorPose3, "PriorPose3");
GenDistrFactor!(Point2Point2, "Point2Point2");
GenDistrFactor!(Point3Point3, "Point3Point3");
GenDistrFactor!(Pose2Pose2, "Pose2Pose2");
GenDistrFactor!(Pose3Pose3, "Pose3Pose3");



// =======================================================


fn assemble_factor_name(ovlb: Vec<String>) -> String {
  let mut flb = "".to_string();
  for o in ovlb {
    flb += &o;
  }
  flb += "_";
  flb += &(Uuid::new_v4().to_string()[0..4]);
  
  return flb;
}


impl FunctionData {
  pub fn new(
    fnc: &str,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>,
  ) -> Self {
    let mut fd = Self::default();
    fd.fnc = fnc.to_owned();
    fd.nullhypo = nullhypo.unwrap_or(0.0);
    fd.multihypo = multihypo.unwrap_or(Vec::new());
    fd.inflation = inflation.unwrap_or(3.0);
    return fd;
  }
  
  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap().to_string()
  }
  
  pub fn to_string_b64(&self) -> String {
    general_purpose::STANDARD.encode(self.to_json())
  }
}


// TODO support more D: Distributions<'a>
impl<'a, F> FactorDFG<F>
where 
  F: crate::FactorType<'a, FullNormal<'a>>
{
  /// Create a new factor
  /// # Arguments
  /// * `varlbls` - variable labels is a ordered vector of variable label strings, e.g. ["x1", "x2"]
  /// * `fnctype` - function type, e.g. "RoME.Pose3Pose3", "RoME.PriorPose2", see Caesar.jl.
  /// * `tags` - tags are additional strings to help identify the factor
  /// * `timestamp` - timestamp, in DateTime<Utc>, default is now
  /// * `nstime` - nstime, also known as "time since epoch" in nanoseconds
  /// * # Returns
  /// * `FactorDFG` - a new factor
  /// # Example
  /// ```
  /// use navabilitysdk::{services::Factors,FullNormal,Distribution,Pose2Pose2,FactorType,FactorDFG};
  /// use chrono::{DateTime, Utc};
  /// let f = FactorDFG::new(
  ///   vec!["x1".to_owned(), "x2".to_owned()], 
  ///   Pose2Pose2::new(FullNormal::new(vec![&[1.0, 2.0, 3.0], &[0.01, 0.01, 0.01]])),
  ///   vec!["ODOMETRY".to_owned(),"BODY_MOTION".to_owned()], 
  ///   Some(Utc::now()),
  ///   None
  /// );
  /// ```
  /// # Note
  /// * This is a simplified version of the ::new_more function, which has more options.
  pub fn new(
    varlbls: Vec<String>,
    fnctype: F,
    tags: Vec<String>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
  ) -> Self {
    return Self::new_more(
      varlbls, 
      fnctype, 
      tags, 
      timestamp, 
      nstime, 
      None, 
      None, 
      None, 
      None,
    );
  }
  
  /// Create a new factor with more expansive inputs.  See `new` for details.
  /// # Arguments
  /// solvable - solvable, default is 1 which means the solver will attempt compute numerical values
  /// multihypo - multihypo, default is empty vector.  Allows fractional hypotheses, see Caesar.jl Docs for details
  /// nullhypo - nullhypo, default is 0.0.  Allows fractional null hypotheses, see Caesar.jl Docs for details
  /// inflation - inflation, default is 3.0.  Allows inflation of the covariance, see Caesar.jl Docs for details
  /// # Returns
  /// * `FactorDFG` - a new factor
  pub fn new_more(
    varlbls: Vec<String>,
    fnctype: F,
    tags: Vec<String>,
    timestamp: Option<DateTime<Utc>>,
    nstime: Option<usize>,
    solvable: Option<i64>,
    multihypo: Option<Vec<f64>>,
    nullhypo: Option<f64>,
    inflation: Option<f64>,
  ) -> Self {

    let binding = fnctype.pack();
    let mut f = Self {
      id: None,
      label: assemble_factor_name(varlbls.clone()),
      tags: Vec::new(),
      variableOrderSymbols_: Vec::new(),
      timestamp: Some(timestamp.unwrap_or(Utc::now())),
      nstime: Some(format!("{}", nstime.unwrap_or(0))),
      fnctype,
      solvable: Some(solvable.unwrap_or(0)),
      data: None,
      metadata: Some("e30=".to_string()),
      _version: Some(crate::SDK_VERSION.to_string()),
    };
    
    for vl in varlbls {
      f.variableOrderSymbols_.push(vl.to_string());
    }
    f.tags.push("FACTOR".to_string());
    for t in tags {
      if !t.eq("FACTOR") {
        f.tags.push(t.to_string());
      }
    }
    // default on create, also deser is different use-case    
    let fdata = FunctionData::new(
      binding.as_str(),
      multihypo, 
      nullhypo, 
      inflation
    );
    // FIXME, should not be json'd so early: JuliaRobotics/DistributedFactorGraphs.jl#1118
    f.data = Some(fdata.to_json());
    
    return f;
  }
}

#[derive(Serialize)]
struct ManualVarWhere {
  id: String,
}

#[derive(Serialize)]
struct ManualVarConnectWhereInput {
  node: ManualVarWhere,
}

#[derive(Serialize)]
struct ManualFacVarConnFieldInput {
  r#where: ManualVarConnectWhereInput,
}

#[derive(Serialize)]
struct ManualFacVarFieldInput {
  connect: Vec<ManualFacVarConnFieldInput>,
}

impl ManualFacVarFieldInput {
  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap().to_string()
  }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_add_factor<'a, F: crate::FactorType<'a, FullNormal<'a>>>(
  nvafg: &NavAbilityDFG,
  factor: FactorDFG<F>,
) -> Result<Uuid, Box<dyn crate::Error>> {
  let label = factor.getLabel().to_string();
  let id = nvafg.getId(&label).to_string();

  let mut variable_order_symbols = Vec::new();
  for v in factor.variableOrderSymbols_ {
    variable_order_symbols.push(v);
  }

  let mut connect = ManualFacVarFieldInput {
    connect: Vec::new(),
  };
  for vl in &variable_order_symbols {
    connect.connect.push(
      ManualFacVarConnFieldInput {
        r#where: ManualVarConnectWhereInput {
          node: ManualVarWhere {
            id: nvafg.getId(vl).to_string(),
          }
        }
      }
    );
  }

  // DFG.FactorDFG + BlobEntry + ... ~= GQL.FactorCreateInput
  let variables = add_factors::Variables {
    id,
    label,
    tags: factor.tags,
    timestamp: factor.timestamp.expect("FactorDFG missing .timestamp field").to_string(),
    nstime: factor.nstime.expect("FactorDFG missing .nstime field"),
    fnctype: factor.fnctype.type_str(),
    solvable: factor.solvable.expect("FactorDFG missing .solvable field"),
    data: factor.data.expect("FactorDFG missing .data field"),
    // metadata: factor.metadata,
    variable_order_symbols: variable_order_symbols,
    version: SDK_VERSION.to_string(),
    fg_id: nvafg.getId("").to_string(),
    variables_connect: None
    // blob_entries: None
    // _type: "",
  };
  
  
  let request_body = AddFactors::build_query(variables);
  // reverse engineer request body to splice in variables_connect without full types
  let jstr = serde_json::to_string(&request_body).unwrap();
  // FIXME just use serde_json::to_value as simpler route
  let mut jval: serde_json::Value = serde_json::from_str(&jstr).unwrap();
  let jcon: serde_json::Value = serde_json::from_str(&serde_json::to_string(&connect).unwrap()).expect("problem with connect");
  jval["variables"]["variables_connect"] = jcon;

  let nvacl = nvafg.client.clone();
  let post_req = nvacl.client
    .post(&nvacl.apiurl)
    .json(&jval);

  return crate::post_to_nvaapi_cb::<
    add_factors::ResponseData,
    Uuid
  >(
    |s| {
      if &s.add_factors.factors.len() != &1 {
        to_console_error(&format!("post_add_factor: expected 1 factor in response, got {}", s.add_factors.factors.len()));
        return Uuid::nil();
      }
      return Uuid::parse_str(&s.add_factors.factors[0].factor_skeleton_fields.id).expect("post_add_variable not able to parse uuid from API response");
    },
    Some(1),
    post_req
  ).await;
}


#[cfg(any(feature = "tokio", feature = "thread"))]
pub fn addFactor<'a, F: crate::FactorType<'a, FullNormal<'a>>>(
  nvafg: &NavAbilityDFG,
  factor: FactorDFG<F>,
) -> Result<Uuid, Box<dyn Error>> {
  return crate::execute(post_add_factor(nvafg, factor));
}


#[cfg(any(feature = "tokio", feature = "thread"))] // feature = "thread", 
pub fn q_addFactor<'a, F: crate::FactorType<'a, FullNormal<'a>>>(
  send_into: Sender<Uuid>, 
  nvafg: NavAbilityDFG,
  factor: FactorDFG<F>,
) -> Result<(), Box<dyn Error>> {
  crate::execute(async {
    return send_api_result(
      send_into, 
      post_add_factor(&nvafg, factor).await,
    );
  })
}

