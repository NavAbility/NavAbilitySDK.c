
use crate::{
    Utc,
    Uuid,
    Sender,
    GraphQLQuery,
    Response,
    Error,
    NavAbilityClient,
    post_to_nvaapi,
    ListGraphs,
    list_graphs,
    check_deser,
    to_console_debug,
    to_console_error,
};




#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_list_graphs(
    nvacl: &NavAbilityClient,
    id: &Uuid,
) -> Result<list_graphs::ResponseData, Box<dyn Error>> {
    
    let request_body = ListGraphs::build_query(list_graphs::Variables {
        id: id.to_string()
    });

    return post_to_nvaapi::<
        list_graphs::Variables,
        list_graphs::ResponseData,
        list_graphs::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}