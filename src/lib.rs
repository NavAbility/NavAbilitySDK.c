

use std::{
    error::Error, 
    sync::mpsc::Sender,
    // collections::HashMap,
};
use serde::Serialize;
use uuid::Uuid;
use chrono::{
    self, 
    Utc
};

use graphql_client::{
    GraphQLQuery, 
    QueryBody, 
    Response
};


#[cfg(any(feature = "tokio"))]
use reqwest::Client;
#[cfg(feature="blocking")]
use ::reqwest::blocking::Client;
#[cfg(feature="blocking")]
use graphql_client::reqwest::post_graphql_blocking;



pub mod entities;
pub use crate::entities::*;

pub mod common_traits;
pub use crate::common_traits::*;

// type and file name are the same and requires precision import
pub use crate::Agent::Agent;
pub use crate::BlobEntry::BlobEntry;

pub mod utils;
pub use crate::utils::*;


pub mod services;
#[cfg(any(feature = "tokio", feature = "blocking"))]
pub use crate::services::{
    post_get_agents,
    // post_get_blob_entry,
    post_delete_blob,
    get_blob_entry_send,
    post_create_upload,
    post_create_download,
    // create_download_send,
    post_complete_upload,
    // post_org_id,
    post_delete_blobentry,
    post_add_agent,
    post_update_blobentry_metadata,
    post_get_agent_entries_metadata,
    post_add_agent_entry,
    list_models_query,
    post_list_models,
    add_model_async,
    post_add_model_blobentry,
    post_list_model_graphs,
    post_list_graphs,
};



pub mod deprecated;
pub use crate::deprecated::*;

// FFI for publishing C compliant library 
#[cfg(feature = "tokio")]
mod capi;
#[cfg(feature = "tokio")]
use crate::capi::*;


const SDK_VERSION: &str = "0.25";


type UUID = String;
type BigInt = String;
type DateTime = String;
type EmailAddress = String;
type Metadata = String;
type JSON = String;
type B64JSON = String;
type Latitude = f64;
type Longitude = f64;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct GetBlobEntry;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ListAgents.gql",
    response_derives = "Debug"
)]
pub struct ListAgents;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/UpdateAgent.gql",
    response_derives = "Debug"
)]
pub struct UpdateAgentMetadata;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetAgents.gql",
    response_derives = "Debug"
)]
pub struct GetAgents;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetAgent.gql",
    response_derives = "Debug"
)]
pub struct GetAgent;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetFactorgraphs.gql",
    response_derives = "Debug"
)]
pub struct GetFactorgraphs;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetURS.gql",
    response_derives = "Debug"
)]
pub struct GetURS;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ListModels.gql",
    response_derives = "Debug"
)]
pub struct ListModels;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetModel.gql",
    response_derives = "Debug"
)]
pub struct GetModel;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ListModelsGraphs.gql",
    response_derives = "Debug"
)]
pub struct ListModelsGraphs;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/FindModelBlobEntries.gql",
    response_derives = "Debug"
)]
pub struct FindModelBlobEntries;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddFactorgraph.gql",
    response_derives = "Debug"
)]
pub struct AddFactorgraph;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetAgentEntriesMetadata.gql",
    response_derives = "Debug"
)]
pub struct GetAgentEntriesMetadata;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/CreateDownload.gql",
    response_derives = "Debug"
)]
pub struct CreateDownload;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/CreateUpload.gql",
    response_derives = "Debug"
)]
pub struct CreateUpload;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/CompleteUpload.gql",
    response_derives = "Debug"
)]
pub struct CompleteUpload;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/StartWorker.gql",
    response_derives = "Debug"
)]
pub struct StartWorker;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetVariable.gql",
    response_derives = "Debug"
)]
pub struct GetVariable;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ListVariables.gql",
    response_derives = "Debug"
)]
pub struct ListVariables;
// Implicit ListWhere due to graphql-client limitation: https://github.com/graphql-rust/graphql-client/issues/508


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddVariable.gql",
    response_derives = "Debug"
)]
pub struct AddVariable;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddFactors.gql",
    response_derives = "Debug"
)]
pub struct AddFactors;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddAgentBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct AddAgentBlobEntry;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddVariableBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct AddVariableBlobEntry;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddFactorgraphBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct AddFactorgraphBlobEntry;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ConnectGraphToAgent.gql",
    response_derives = "Debug"
)]
pub struct ConnectGraphAgent;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ConnectGraphToModel.gql",
    response_derives = "Debug"
)]
pub struct ConnectGraphModel;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddModelBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct AddModelBlobEntry;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/DeleteBlobEntry.gql",
    response_derives = "Debug"
)]
pub struct DeleteBlobEntry;

#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/DeleteBlob.gql",
    response_derives = "Debug"
)]
pub struct DeleteBlob;

#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/GetOrg.gql",
    response_derives = "Debug"
)]
pub struct GetOrg;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/ListGraphs.gql",
    response_derives = "Debug"
)]
pub struct ListGraphs;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/FindOrgModelGraphs.gql",
    response_derives = "Debug"
)]
pub struct FindOrgModelGraphs;


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/FindFactorgraphBlobEntries.gql",
    response_derives = "Debug"
)]
pub struct FindFactorgraphBlobEntries;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddAgent.gql",
    response_derives = "Debug"
)]
pub struct AddAgent;

#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/AddModel.gql",
    response_derives = "Debug"
)]
pub struct AddModel;

#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.json",
    query_path = "src/gql/UpdateBlobentryMetadata.gql",
    response_derives = "Debug"
)]
pub struct UpdateBlobentryMetadata;


// ===================== traits =========================

pub trait QueryDetails<Q: Serialize> {
    fn operation_name(&self) -> &str;
    fn query(&self) -> String;
    fn variables_jstr(&self) -> Result<String,serde_json::Error>;
    fn to_jstr(&self) -> String;
}

impl<Q: Serialize> QueryDetails<Q> for graphql_client::QueryBody<Q> {
    fn operation_name(&self) -> &str { 
        self.operation_name 
    }

    fn query(&self) -> String { 
        self.query.replace("\n","").replace("\"","\\\"") 
    }
    
    fn variables_jstr(&self) -> Result<String,serde_json::Error> { 
        serde_json::to_string(&self.variables)
    }

    fn to_jstr(&self) -> String {
        format!(
            r#"{{"extensions": {}, "operationName": "{}", "query": "{}", "variables": {}}}"#, 
            "{}",
            self.operation_name(),
            self.query(),
            self.variables_jstr().unwrap_or("".to_owned()),
        )
    }
}




// move some impl GetLabel to services
genGetLabel!(User);
genGetLabel!(Agent);
genGetLabel!(BlobEntry);
genGetLabel!(VariableDFG);

#[cfg(any(feature = "tokio", feature = "blocking", feature = "thread"))]
impl GetLabel for NavAbilityBlobStore {
    fn getLabel(&self) -> &String { 
        match &self.label {
            NvaStoreLabel::Cloud(l) =>  {return l},
            NvaStoreLabel::Onprem(l) => {return l},
        };
    }
}


// move to services
impl<T> GetLabel for NvaNode<T> {
    fn getLabel(&self) -> &String { &self.label }
}

#[cfg(any(feature = "tokio", feature = "blocking"))]
impl GetLabel for crate::entities::ClientDFG::NavAbilityDFG {
    fn getLabel(&self) -> &String { &self.fg.getLabel() }
}

impl<F> GetLabel for FactorDFG<F> {
    fn getLabel(&self) -> &String { &self.label }
}

// ---------------- GetId trait ----------------

pub trait GetId {
    /// Get the deterministic identifier (uuid v5) for a node.
    #[allow(non_snake_case)]
    fn getId(
        &self,
        label: &str,
    ) -> Uuid;
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
impl<T> GetId for NvaNode<T> {
    fn getId(
        &self, 
        label: &str
    ) -> Uuid {
        let ostr = self.label.clone();
        return Uuid::new_v5(
            &self.namespace, 
            (&(ostr + label)).as_bytes()
        )
    }
}





// ================================= TESTS ===============================



#[cfg(any(feature = "tokio", feature = "blocking"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_robots() {
        // parse datetime example 1
        let text = "2024-09-03 02:31:39.367 UTC";
        let _res = parse_str_utc(text.to_owned());
        println!("parse_str_utc {:?}",_res);
        let _ = _res.unwrap(); // make sure the conversion worked
        // parse datetime example 2
        let text = "2024-10-28T20:56:35.270Z";
        let _res = parse_str_utc(text.to_owned());
        println!("parse_str_utc {:?}",_res);
        let _ = _res.unwrap(); // make sure the conversion worked


        let nva_userlabel: String = "test@wherewhen.ai".to_owned();
            // std::env::var("NAVABILITY_USERLABEL").expect("Missing NAVABILITY_USERLABEL env var");

        let nva_api_token: String = "".to_owned();
            // std::env::var("NAVABILITY_API_TOKEN").expect("Missing NAVABILITY_API_TOKEN env var");

        let api_url: &str = "https://api.navability.io/graphql";
        let client = NavAbilityClient::new(&api_url.to_string(), &nva_api_token, Some(&nva_userlabel));
        println!("client: {:?}", client);

        #[cfg(feature = "blocking")]
        let robotrs = get_robots_blocking(&client);
        // println!("robots: {:?}", robotrs);

        #[cfg(feature = "blocking")]
        let robotlist = get_robots_blocking(&client);
        // println!("robot list: {:?}", robotlist);
    }
}
