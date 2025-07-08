

use std::fmt;
use std::str::FromStr;


use crate::{
    to_console_error, 
    MeanMaxPPE, 
    PackedVariableNodeData, 
    Utc, 
    VariableDFG, 
    VariableType,
    SDK_VERSION,
};


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::{
    parse_str_utc, 
    Uuid,
    BlobEntry, 
    Error,     
    Sender, 
    NavAbilityDFG,
    send_api_result,
    post_to_nvaapi,
    GetVariable, 
    GraphQLQuery,
    ListVariables,
    AddVariable,
    DeleteVariable,
    delete_variable,
    GetId,
    get_variable::{
        self, 
        ppe_fields
    }, 
    list_variables, 
    // to_console_debug, 
    // to_console_error, 
};


impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RoME.{:?}", self)
    }
}


impl FromStr for VariableType {

    type Err = ();

    fn from_str(
        var_type: &str
    ) -> Result<VariableType, Self::Err> {
        let s = var_type.split('.').collect::<Vec<&str>>();
        if s.len() != 2 {
            to_console_error(&format!("Missing variable type module definition: {}", var_type));
        } else {
            if !s[0].eq("RoME") {
                to_console_error(&format!("Unknown variable type module (currently only RoME): {}", s[0]));
            }
        }
        return match s.last().unwrap().as_ref() {
            "Point2" => Ok(VariableType::Point2),
            "Point3" => Ok(VariableType::Point3),
            "Pose2" => Ok(VariableType::Pose2),
            "Pose3" => Ok(VariableType::Pose3),
            _ => Err(()),
        };
    }
}


#[allow(non_snake_case)]
impl MeanMaxPPE {
    pub fn new(
        solveKey: &str,
        suggested: Vec<f64>,
        max: Vec<f64>,
        mean: Vec<f64>,
        _type: &str,
        _version: &str,
        createdTimestamp: Option<chrono::DateTime<Utc>>,
        lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
    ) -> Self {
        Self {
            id: None,
            solveKey: solveKey.to_owned(),
            suggested,
            max,
            mean,
            _type: _type.to_owned(),
            _version: _version.to_owned(),
            createdTimestamp,
            lastUpdatedTimestamp,
        }
    }

    #[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
    pub fn from_gql(
        ppe: &ppe_fields // FIXME refac to trait similar to BlobEntry_fields
    ) -> Self {
        let mut ppesugg = Vec::new();
        if let Some(ps) = &ppe.suggested {
            for p in ps.iter() {
                if p.is_some() { ppesugg.push(p.unwrap().clone()); }
            }
        }
        let mut ppemax = Vec::new();
        if let Some(ps) = &ppe.max {
            for p in ps.iter() {
                if p.is_some() { ppemax.push(p.unwrap().clone()); }
            }
        }
        let mut ppemean = Vec::new();
        if let Some(ps) = &ppe.mean {
            for p in ps.iter() {
                if p.is_some() { ppemean.push(p.unwrap().clone()); }
            }
        }
        let mut ppe_struct = MeanMaxPPE {
            id: None,
            solveKey: ppe.solve_key.clone(),
            suggested: ppesugg,
            max: ppemax,
            mean: ppemean,
            _type: ppe.type_.clone(),
            _version: ppe.version.clone(),
            createdTimestamp: None,
            lastUpdatedTimestamp: None,
        };
        if let Ok(id) = Uuid::parse_str(
            &ppe.id
        ) {
            ppe_struct.id = Some(id);
        }
        if let Ok(dt) = parse_str_utc(
            ppe.created_timestamp.clone()
        ) {
            ppe_struct.createdTimestamp = Some(dt);
        }
        if let Ok(dt) = parse_str_utc(
            ppe.last_updated_timestamp.clone()
        ) {
            ppe_struct.lastUpdatedTimestamp = Some(dt);
        }
        return ppe_struct;
    }
}


#[allow(non_snake_case)]
impl VariableDFG {
    pub fn new(
        label: &str,
        variableType: &VariableType,
        timestamp: Option<chrono::DateTime<Utc>>,
        nstime: Option<usize>,
    ) -> Self {
        let _ts = timestamp.unwrap_or(Utc::now());
        Self {
            id: None,
            label: label.to_owned(),
            tags: Vec::new(),
            timestamp: _ts,
            nstime: nstime.unwrap_or(0),
            ppes: Vec::new(),
            blobEntries: Vec::new(),
            variableType: variableType.clone(),
            _version: SDK_VERSION.to_owned(),
            metadata: "".to_owned(),
            solvable: 1,
            solverData: Vec::new(),
        }
    }

    #[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
    pub fn from_gql(
        vgql: &get_variable::GetVariableVariables
    ) -> Self {
        
        let timestamp = if let Ok(dt) = parse_str_utc(
            vgql.variable_summary_fields.timestamp.clone()
        ) {
            Some(dt)
        } else {
            None
        };
        let nstime = if let Ok(ns) = vgql.variable_summary_fields.nstime.parse::<usize>() {
            Some(ns)
        } else {
            None
        };

        let mut variable = Self::new(
            &vgql.variable_skeleton_fields.label.clone(),
            &VariableType::from_str(
                &vgql.variable_summary_fields.variable_type.to_string()
            ).unwrap(),
            timestamp,
            nstime,
        );

        // mutate additional information that is available
        if let Ok(id) = Uuid::parse_str(
            &vgql.variable_skeleton_fields.id
        ) {
            variable.id = Some(id);
        }
        // label
        let mut tags = Vec::new();
        for tag in vgql.variable_skeleton_fields.tags.iter() {
            // if (&tag).is_some() {
                tags.push(tag.clone());
            // }
        }
        variable.tags = tags;

        variable._version = vgql.variable_summary_fields.version.clone();
        variable.solvable = vgql.variable_full_fields.solvable as i32;
        if let Some(md) = vgql.variable_full_fields.metadata.clone() {
            variable.metadata = md;
        }

        let mut ppes = Vec::new();
        for ppe in &vgql.variable_summary_fields.ppes {
            ppes.push(MeanMaxPPE::from_gql(ppe));
        }
        variable.ppes = ppes;

        let mut bes = Vec::new();
        for be in &vgql.variable_summary_fields.blob_entries {
            bes.push(BlobEntry::from_gql(be));
        }
        variable.blobEntries = bes;

        let mut vnds = Vec::new();
        for vnd in &vgql.variable_full_fields.solver_data {
            vnds.push(PackedVariableNodeData::from_gql(vnd));
        }
        variable.solverData = vnds;

        return variable
    }
}


impl PackedVariableNodeData {
    pub fn new(
    ) -> Self {
        Self {
            id: None,
            vecval: Vec::new(),
            dimval: 0 as i32,
            vecbw: Vec::new(),
            dimbw: 0 as i32,
            BayesNetOutVertIDs: Vec::new(),
            dimIDs: Vec::new(),
            dims: 0 as i32,
            eliminated: false,
            BayesNetVertID: "".to_owned(),
            separator: Vec::new(),
            variableType: "".to_owned(),
            initialized: false,
            infoPerCoord: Vec::new(),
            ismargin: false,
            dontmargin: false,
            solveInProgress: 0 as i32,
            solvedCount: 0 as i32,
            solveKey: "".to_owned(),
            covar: Vec::new(),
            _version: SDK_VERSION.to_string(),
        }
    }

    #[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
    pub fn from_gql(
        vndgql: &get_variable::solverdata_fields
    ) -> Self {
        return Self {
            id: Some(Uuid::parse_str(&vndgql.id).expect("failed to parse variable solver data id to uuid")),
            dimIDs: vndgql.dim_i_ds.clone().into_iter().map(|x| x as i32).collect(),
            infoPerCoord: vndgql.info_per_coord.clone(),
            BayesNetOutVertIDs: vndgql.bayes_net_out_vert_i_ds.clone().expect("PackedVariableNodeData to struct failed on BayesNetOutVertIDs"),
            separator: vndgql.separator.clone().unwrap_or(Vec::new()),
            vecval: vndgql.vecval.clone().expect("PackedVariableNodeData to struct failed on vecval"),
            vecbw: vndgql.vecbw.clone().expect("PackedVariableNodeData to struct failed on vecbw"),
            covar: vndgql.covar.clone().expect("PackedVariableNodeData to struct failed on covar"),
            dimval: vndgql.dimval as i32,
            dimbw: vndgql.dimbw as i32,
            dims: vndgql.dims as i32,
            solvedCount: vndgql.solved_count as i32,
            solveInProgress: vndgql.solve_in_progress as i32,
            initialized: vndgql.initialized,
            ismargin: vndgql.ismargin,
            dontmargin: vndgql.dontmargin,
            eliminated: vndgql.eliminated,
            BayesNetVertID: vndgql.bayes_net_vert_id.as_ref().unwrap_or(&"".to_string()).to_string(),
            variableType: vndgql.variable_type.to_string(),
            solveKey: vndgql.solve_key.to_string(), 
            _version: vndgql.version.to_string(),
        }
    }
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub fn getPPEMean(
    vari: &VariableDFG,
    solveKey: &str
) -> Vec<f64> {

    for ppe in vari.ppes.iter() {
        if ppe.solveKey.eq(solveKey) {
            return ppe.mean.clone();
        }
    }
    return Vec::new();
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub fn getPPECov(
    _vari: &VariableDFG,
    _solveKey: &str
) -> Vec<f64> {

    todo!("getPPECov, TODO extract from solverData.val -- see JuliaRobotics/DistributedFactorGraphs.jl#535");
    // for ppe in vari.ppes.iter() {
    //     if ppe.solveKey.eq(solveKey) {
    //         return ppe.cov.clone();
    //     }
    // }
    // return Vec::new();
}


// ===================== Queries =========================

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_get_variable(
    nvafg: &NavAbilityDFG,
    label: &str,
    fields_full: bool,
) -> Result<Option<VariableDFG>, Box<dyn Error>> {

    let id = nvafg.fg.getId(label); 
    let request_body = GetVariable::build_query(
        get_variable::Variables {
            var_id: id.to_string(),
            fields_summary: true, // TODO simplify, since this must always be true
            fields_full,
        }
    );

    return post_to_nvaapi::<
        get_variable::Variables,
        get_variable::ResponseData,
        Option<VariableDFG>
    >(
        &nvafg.client,
        request_body, 
        |s| {
            if 0 < s.variables.len() {
                // FIXME return the entire list of variables
                return Some(VariableDFG::from_gql(&s.variables[0]));
            }
            return None;
        },
        Some(3)
    ).await;
}

#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn getVariable(
    nvafg: &NavAbilityDFG,
    label: &str,
    fields_full: bool,
) -> Option<VariableDFG> {
    // TODO why not returning Result like others, did something force this to Option?
    let result = crate::execute(post_get_variable(
        nvafg,
        label,
        fields_full,
    ));

    if let Ok(variable) = result {
        return variable;
    }

    return None
}


// TODO get better function signature
// #[allow(non_snake_case)]
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_list_variables(
    nvafg: &NavAbilityDFG,
) -> Result<Vec<String>, Box<dyn Error>> {
    let id = nvafg.fg.getId(""); 
    let request_body = ListVariables::build_query(
        list_variables::Variables {
            fg_id: id.to_string(),
            solvable_gt: None,
            solvable_gte: None,
            solvable_in: None,
            solvable_lt: None,
            solvable_lte: None,
            tags_includes: None,
        }
    );

    return post_to_nvaapi::<
        list_variables::Variables,
        list_variables::ResponseData,
        Vec<String>
    >(
        &nvafg.client,
        request_body, 
        |s| {
            return s.list_variables;
        },
        Some(3)
    ).await;
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn send_list_variables(
    send_into: Sender<Vec<String>>,
    nvafg: &NavAbilityDFG,
) -> Result<(), Box<dyn Error>> {
    let result = post_list_variables(nvafg).await;
    send_api_result(send_into, result)
}


#[cfg(feature = "tokio")]
#[allow(non_snake_case)]
pub fn listVariables(
    nvafg: &NavAbilityDFG,
) -> Result<Vec<String>, Box<dyn Error>> {
    return tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(post_list_variables(
            nvafg,
        ));
}





// FIXME use standardized function instead, perhaps rfc3339
// pub fn to_ISO8601(
//     dt: chrono::DateTime<Utc>
// ) -> String {
//     Working example: "2025-01-01 00:00:00.000"
//     return dt.to_rfc3339_opts(chrono::SecondsFormat::Millis??, false);
// }
pub fn to_string_ISO8601(
    dt: chrono::DateTime<Utc>
) -> String{
    let timestamp = dt.to_string()
    .replace(" UTC","")
    .replace("UTC","")
    .replace("Z","")
    .replace("T"," ");
    let parts = timestamp.split(".").collect::<Vec<&str>>();
    // have millis
    let mut millis = "000".to_owned();
    if 1 < parts.len() {
        millis = format!("{:0<3}", parts[1].get(..3).unwrap_or(parts[1]));
    }
    return format!("{}.{}", parts[0], millis);
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_add_variable(
    nvafg: &NavAbilityDFG,
    label: &String,
    variableType: &VariableType,
    _tags: Option<Vec<String>>,
    _timestamp: Option<chrono::DateTime<Utc>>,
    _nstime: Option<usize>,
    _solvable: Option<i64>,
    _metadata: Option<String>,
) -> Result<String,Box<dyn Error>> {

    let metadata = Some(if _metadata.is_some() {
        _metadata.unwrap().clone()
    } else {
        "e30=".to_string()
    });
    let tags = if _tags.is_some() {
        _tags.clone().unwrap()
    } else {
        let mut v = Vec::new();
        v.push("VARIABLE".to_owned());
        v
    };
    let solvable = _solvable.unwrap_or(1);
    let timestamp = to_string_ISO8601(_timestamp.unwrap_or(Utc::now()));
    let nstime = _nstime.unwrap_or(0).to_string();
    
    let variables = crate::add_variable::Variables {
        id: nvafg.getId(label).to_string(),
        label: label.to_string(),
        variable_type: variableType.to_string(),
        tags,
        timestamp,
        nstime: nstime.to_string(),
        solvable,
        metadata,
        fg_id: nvafg.getId("").to_string(),
    };

    let request_body = AddVariable::build_query(variables);

    return post_to_nvaapi::<
        crate::add_variable::Variables,
        crate::add_variable::ResponseData,
        String
    >(
        &nvafg.client,
        request_body, 
        |s| {
            // return Uuid::parse_str(&s.add_variables.variables[0].id).expect("post_add_variable not able to parse uuid from API response");
            return s.add_variables.variables[0].label.to_string();
        },
        Some(1)
    ).await;
}


#[cfg(any(feature = "tokio", feature = "thread"))]
pub async fn add_variable_send(
    send_into: std::sync::mpsc::Sender<String>,
    nvafg: &NavAbilityDFG,
    label: &String,
    variableType: &VariableType,
    _tags: Option<Vec<String>>,
    _timestamp: Option<chrono::DateTime<Utc>>,
    _nstime: Option<usize>,
    _solvable: Option<i64>,
    _metadata: Option<String>,
) -> Result<(),Box<dyn Error>> {
    
    return send_api_result(
        send_into, 
        post_add_variable(
            nvafg,
            label,
            variableType,
            _tags,
            _timestamp,
            _nstime,
            _solvable,
            _metadata,
        ).await,
    );
}


#[cfg(feature = "tokio")]
pub fn addVariable(
    nvafg: &NavAbilityDFG,
    label: &String,
    variableType: &VariableType,
    _tags: Option<Vec<String>>,
    _timestamp: Option<chrono::DateTime<Utc>>,
    _nstime: Option<usize>,
    _solvable: Option<i64>,
    _metadata: Option<String>,
) -> Result<String, Box<dyn Error>> {

    return crate::execute(post_add_variable(
        nvafg,
        label,
        variableType,
        _tags,
        _timestamp,
        _nstime,
        _solvable,
        _metadata,
    ));
}




#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
pub async fn post_delete_variable(
  nvafg: &NavAbilityDFG,
  label: &str,
) -> Result<i64, Box<dyn Error>> {
  
  let variables = delete_variable::Variables {
    variable_id: nvafg.getId(label).to_string(),
  };
  let request_body = DeleteVariable::build_query(variables);
  
  return crate::post_to_nvaapi::<
    delete_variable::Variables,
    delete_variable::ResponseData,
    i64
  >(
    &nvafg.client,
    request_body, 
    |s| {
      s.delete_variables.nodes_deleted
    },
    Some(3)
  ).await;
}


#[allow(non_snake_case)]
#[cfg(any(feature = "tokio"))] // , feature = "thread"
pub fn q_deleteVariable(
  send_into: crate::Sender<i64>, 
  nvafg: &NavAbilityDFG,
  label: &str,
) -> Result<(), Box<dyn Error>> {
  crate::execute(async {
    return send_api_result(
      send_into, 
      post_delete_variable(nvafg, label).await,
    );
  })
}


#[cfg(feature = "tokio")]
pub fn deleteVariable(
  nvafg: &NavAbilityDFG,
  label: &str,
) -> Result<i64, Box<dyn Error>> {
  return crate::execute(post_delete_variable(nvafg, label));
}
