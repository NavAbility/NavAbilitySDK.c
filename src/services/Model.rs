
// use std::collections::HashMap;

use chrono::DateTime;
// use log::Metadata;

use crate::{
    Agent, 
    BlobEntry, 
    Utc, 
    Uuid,
};

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::Error;
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use base64::{Engine as _, engine::general_purpose};

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::{
    GraphQLQuery,
    QueryBody,
    NavAbilityClient,
    // NvaNode,
    // Factorgraph,
    ListModels,
    GetModel,
    AddModel,
    ListModelsGraphs,
    // GetId,
    parse_str_utc,
    // to_console_debug,
    to_console_error,
    post_to_nvaapi,
};


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub fn list_models_query(
    model_label_contains: Option<&str>,
) -> QueryBody<crate::list_models::Variables> {
    let mut model_lbl_contains = Some("".to_string());
    if let Some(mt) = model_label_contains {
        model_lbl_contains = Some(mt.to_string());
    }

    let variables = crate::list_models::Variables {
        label_contains: model_lbl_contains,
    };
    ListModels::build_query(variables)
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_list_models(
    nvacl: &NavAbilityClient,
    model_label_contains: Option<&str>,
) -> Result<crate::list_models::ResponseData, Box<dyn Error>> {
    
    let request_body = list_models_query(model_label_contains);

    return post_to_nvaapi::<
        crate::list_models::Variables,
        crate::list_models::ResponseData,
        crate::list_models::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}



// #[macro_use]
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::{
    Graph_importers_skeleton,
    GraphFieldImportersSkeleton
};

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::get_model::graph_fields_skeleton as GM_GraphFieldsSkeleton;
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
Graph_importers_skeleton!(GM_GraphFieldsSkeleton);


pub struct GetFactorgraph {
    pub id: Uuid,
    pub label: String,
    pub lastUpdatedTimestamp: DateTime<Utc>,
    pub namespace: Uuid,
    pub numVariables: i64,
    pub numFactors: i64,
    pub agents: Vec<Agent>,
}

#[allow(non_snake_case)]
pub struct GetModelResponse {
    pub id: Uuid,
    pub label: String,
    pub lastUpdatedTimestamp: DateTime<Utc>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
    pub tags: Vec<String>,
    pub blobEntries: Vec<BlobEntry>,
    pub fgs: Vec<GetFactorgraph>
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
impl GetModelResponse {
    pub fn from_gql_summary(
        gmr: &crate::get_model::ResponseData
    ) -> Self {
        if gmr.models.is_empty() {
            to_console_error("get_model: no models found");
        }
        let mut fgs = Vec::new();
        for fg in &gmr.models[0].fgs {
            let mut agents = Vec::new();
            for ag in &fg.agents {
                agents.push(Agent::from_gql_summary(ag));
            }
            let fgsk = &fg.graph_fields_skeleton;
            fgs.push(GetFactorgraph {
                id: Uuid::parse_str(&fgsk.id).expect("failed to parse factorgraph id to uuid"),
                label: fgsk.label.to_string(),
                lastUpdatedTimestamp: parse_str_utc(fgsk.last_updated_timestamp.clone()).expect("failed to parse factorgraph last_updated_timestamp to datetime"),
                namespace: Uuid::parse_str(&fgsk.namespace.clone().unwrap()).expect("failed to parse factorgraph namespace to uuid"),
                numVariables: fg.num_variables.unwrap(),
                numFactors: fg.num_factors.unwrap(),
                agents,
            });
        }
        let mut bes = Vec::new();
        for be in &gmr.models[0].blob_entries {
            bes.push(BlobEntry::from_gql_summary(be));
        }


        let mut metadata: serde_json::Map<String,serde_json::Value> = serde_json::Map::new();
        let jstr_b64 = &gmr.models[0].metadata.clone().unwrap();
        match &general_purpose::STANDARD.decode(jstr_b64) {
            Err(e) => {
                to_console_error(&format!("Failed to decode agent metadata base64 string: {:?}", e));
            },
            Ok(jstr_) => {
                let jstr = std::str::from_utf8(jstr_).expect("Invalid agent metadata json UTF8 string decoding "); // FIXME make soft error only
                if let Ok(jmap) = serde_json::from_str(&jstr) {
                    metadata = jmap;
                }
            },
        }
        return Self {
            id: Uuid::parse_str(&gmr.models[0].id).expect("failed to parse model id to uuid"),
            label: gmr.models[0].label.to_string(),
            lastUpdatedTimestamp: parse_str_utc(gmr.models[0].last_updated_timestamp.clone()).expect("failed to parse model last_updated_timestamp to datetime"),
            metadata,
            tags: gmr.models[0].tags.clone(),
            blobEntries: bes,
            fgs,
        };
    }
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_get_model(
    nvacl: &NavAbilityClient,
    model_label: &str,
) -> Result<GetModelResponse, Box<dyn Error>> {

    let request_body = GetModel::build_query(crate::get_model::Variables {
        label: model_label.to_string(),
    });

    return post_to_nvaapi::<
        crate::get_model::Variables,
        crate::get_model::ResponseData,
        GetModelResponse
    >(
        nvacl,
        request_body,
        |s| GetModelResponse::from_gql_summary(&s),
        Some(3)
    ).await;
}



#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn add_model_async(
    nvacl: &NavAbilityClient,
    model_label: &String,
) -> Result<crate::add_model::ResponseData,Box<dyn Error>> {
    let org_id = Uuid::parse_str(&nvacl.user_label).expect("Unable to parse org_id as uuid.");
    let name = format!("{}",&model_label).to_string();
    let agent_id = Uuid::new_v5(&org_id, name.as_bytes());

    let variables = crate::add_model::Variables {
        org_id: org_id.to_string(),
        model_id: agent_id.to_string(),
        label: model_label.to_string(),
        tags: Some(Vec::new())
    };

    let request_body = AddModel::build_query(variables);

    return post_to_nvaapi::<
        crate::add_model::Variables,
        crate::add_model::ResponseData,
        crate::add_model::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(1)
    ).await;
}



#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub fn list_model_graphs_query(
    model_label: &str,
) -> QueryBody<crate::list_models_graphs::Variables> {

    let variables = crate::list_models_graphs::Variables {
        label: model_label.to_string(), 
    };
    return  ListModelsGraphs::build_query(variables);
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn post_list_model_graphs(
    nvacl: NavAbilityClient,
    model_label: &str,
) -> Result<crate::list_models_graphs::ResponseData, Box<dyn Error>> {
    
    let request_body = list_model_graphs_query(model_label);

    return post_to_nvaapi::<
        crate::list_models_graphs::Variables,
        crate::list_models_graphs::ResponseData,
        crate::list_models_graphs::ResponseData
    >(
        &nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}


