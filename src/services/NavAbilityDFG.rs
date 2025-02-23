
use std::collections::HashMap;

use crate::{
    Uuid,
    GetId,
    NvaNode,
    Agent,
    Factorgraph,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    NavAbilityClient,
    NavAbilityDFG,
    NavAbilityBlobStore,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl GetId for NavAbilityDFG {
    fn getId(
        &self, 
        label: &str
    ) -> Uuid {
        return self.fg.getId(label)
    }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[allow(non_snake_case)]
impl NavAbilityDFG {
    pub fn new(
        client: &NavAbilityClient,
        fgLabel: &str,
        agentLabel: &str, // TODO make Option and try find only linked Agent to factorgraph
        storeLabel: Option<&str>,
        addAgentIfAbsent: Option<bool>,
        addGraphIfAbsent: Option<bool>,
    ) -> Self {
        let _client = client.clone();
        let namespace = Uuid::parse_str(&client.user_label).unwrap();
        let storelb = storeLabel.unwrap_or("default");
        
        

        let fg = NvaNode::<Factorgraph>{
            namespace: namespace.clone(),
            label: fgLabel.to_string(),
            _marker: Default::default()
        };
        let agent = NvaNode::<Agent>{
            namespace: namespace,
            label: agentLabel.to_string(),
            _marker: Default::default()
        };
        let store = NavAbilityBlobStore {
            client: _client.clone(),
            label: storelb.to_owned(),
        };
        let mut blobStores = HashMap::new();
        blobStores.insert(store.label.clone(), store);
        return Self {
            client: _client,
            fg,
            agent,
            blobStores,
        }
    }
}