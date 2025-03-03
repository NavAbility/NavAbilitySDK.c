

#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
    // Uuid,
    Error,
    // Response,
    GraphQLQuery,
    // QueryBody,
    GetOrg,
    get_org,
    post_to_nvaapi,
    // to_console_debug,
    to_console_error,
    // check_deser,
    NavAbilityClient,
};


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub async fn post_org_id(
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
            // if s.orgs.is_empty() {
            //     to_console_error("Error, no orgs found");
            //     return Uuid::nil();
            // }
            // return Uuid::parse_str(&s.orgs[0].id.clone()).expect("Error, unable to parse OrgId Uuid from GQL GetOrg response string");
        // },
        Some(3)
    ).await;
}

