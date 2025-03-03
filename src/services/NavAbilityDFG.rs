
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use std::collections::HashMap;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
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
            label: crate::NvaStoreLabel::Cloud(storelb.to_owned()),
        };
        let mut blobStores = HashMap::new();

        let mut mkey = "".to_owned();
        match &store.label {
            crate::NvaStoreLabel::Cloud(lb) => { mkey = lb.clone();},
            crate::NvaStoreLabel::Onprem(lb) => { mkey = lb.clone();},
        }
        blobStores.insert(mkey, store);

        // check if fgraph exists
        if addGraphIfAbsent.is_some() && addGraphIfAbsent.unwrap() {
            let fgs = crate::services::listGraphs(client);
            if !fgs.is_ok() || !fgs.unwrap().contains(&fgLabel.to_string()) {
                let _ = crate::services::addFactorgraph(
                    client, 
                    fgLabel,
                    "",
                    "e30="
                );
            }
        }

        // check if agent exists
        if addAgentIfAbsent.is_some() && addAgentIfAbsent.unwrap() {
            let agents = crate::services::listAgents(client);
            if !agents.is_ok() || !agents.unwrap().contains(&agentLabel.to_string()) {
                let _ = crate::services::addAgent(client, &(agentLabel.to_string()));
            }
        }


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