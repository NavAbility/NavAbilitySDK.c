
use crate::{
    Uuid,
    Utc,
    BlobEntry,
};


/// Data container to store Parameteric Point Estimate (PPE) for mean and max.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct MeanMaxPPE {
    pub id: Option<Uuid>,
    pub solveKey: String,
    pub suggested: Vec<f64>,
    pub max: Vec<f64>,
    pub mean: Vec<f64>,
    pub _type: String,
    pub _version: String, // TODO getVersion
    pub createdTimestamp: Option<chrono::DateTime<Utc>>,
    pub lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
}

/// Packed VariableNodeData structure for serializing DFGVariables.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct PackedVariableNodeData {
    pub id: Option<Uuid>,
    pub vecval: Vec<f64>,
    pub dimval: i32,
    pub vecbw: Vec<f64>,
    pub dimbw: i32,
    pub BayesNetOutVertIDs: Vec<String>,
    pub dimIDs: Vec<i32>,
    pub dims: i32,
    pub eliminated: bool,
    pub BayesNetVertID: String,
    pub separator: Vec<String>,
    pub variableType: String,
    pub initialized: bool,
    pub infoPerCoord: Vec<f64>,
    pub ismargin: bool,
    pub dontmargin: bool,
    pub solveInProgress: i32,
    pub solvedCount: i32,
    pub solveKey: String,
    pub covar: Vec<f64>,
    pub _version: String,
}


/// The Variable information packed in a way that accomdates multi-lang using json.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct VariableDFG {
    pub id: Option<Uuid>,
    pub label: String,
    pub tags: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub nstime: usize,
    pub ppes: Vec<MeanMaxPPE>,
    pub blobEntries: Vec<BlobEntry>,
    pub variableType: VariableType,
    pub _version: String,
    pub metadata: String,
    pub solvable: i32,
    pub solverData: Vec<PackedVariableNodeData>,
}


#[derive(Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub enum VariableType {
    Point2,
    Point3,
    Pose2,
    Pose3,
}


