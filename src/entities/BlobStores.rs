
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking", feature = "thread"))]
use crate::NavAbilityClient;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking", feature = "thread"))]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum NvaStoreLabel {
    Cloud(String),
    Onprem(String),
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking", feature = "thread"))]
#[derive(Debug, Clone)]
pub struct NavAbilityBlobStore {
    pub client: NavAbilityClient,
    pub label: NvaStoreLabel,
}

