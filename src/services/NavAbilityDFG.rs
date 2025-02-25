
use std::collections::HashMap;

use crate::{
    Uuid,
    GetId,
    NvaNode,
    Agent,
    Factorgraph,
};

#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
    NavAbilityClient,
    NavAbilityDFG,
    NavAbilityBlobStore,
};


#[cfg(any(feature = "tokio", feature = "blocking"))]
impl GetId for NavAbilityDFG {
    fn getId(
        &self, 
        label: &str
    ) -> Uuid {
        return self.fg.getId(label)
    }
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
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

        // check if fgraph exists
        let fgs = crate::services::listGraphs(client);
        if !fgs.is_ok() || !fgs.unwrap().contains(&fgLabel.to_string()) {
            let _ = crate::services::addFactorgraph(
                client, 
                fgLabel,
                "",
                "e30="
            );
        }

        // check if agent exists
        let agents = crate::services::listAgents(client);
        if !agents.is_ok() || !agents.unwrap().contains(&agentLabel.to_string()) {
            let _ = crate::services::addAgent(client, &(agentLabel.to_string()));
        }

        let _ = crate::services::connectAgentGraph(
            client, 
            agentLabel, 
            fgLabel
        );

        return Self {
            client: _client,
            fg,
            agent,
            blobStores,
        }
    }
}