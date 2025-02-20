
use crate::{
    Error,
    Response,
    GraphQLQuery,
    QueryBody,
    GetOrg,
    get_org,
    post_to_nvaapi,
    to_console_debug,
    to_console_error,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    check_deser,
    NavAbilityClient,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn fetch_org_id(
    nvacl: &NavAbilityClient,
) -> Result<get_org::ResponseData, Box<dyn Error>> {
    
    let request_body = GetOrg::build_query(get_org::Variables {});

    return post_to_nvaapi::<
        get_org::Variables,
        get_org::ResponseData,
        get_org::ResponseData
    >(
        nvacl,
        request_body, 
        |s| s,
        Some(3)
    ).await;
}