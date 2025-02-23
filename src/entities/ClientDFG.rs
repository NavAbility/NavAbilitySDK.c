
use std::collections::HashMap;

use crate::{
    Agent,
    NvaNode,
    Factorgraph,
};

#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
    Client,
    NavAbilityBlobStore,
};


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct NavAbilityClient {
    pub client: Client,
    pub apiurl: String,
    pub user_label: String,
    pub nva_api_token: String,
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct NavAbilityDFG {
    pub client: NavAbilityClient,
    pub fg: NvaNode<Factorgraph>,
    pub agent: NvaNode<Agent>,
    pub blobStores: HashMap<String, NavAbilityBlobStore>,
}