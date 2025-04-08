

#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
    // Uuid,
    Sender,
    Error,
    GraphQLQuery,
    // QueryBody,
    GetOrg,
    post_to_nvaapi,
    send_api_result,
    to_console_debug,
    to_console_error,
    NavAbilityClient,
};


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub async fn post_org_id(
    nvacl: &NavAbilityClient,
) -> Result<crate::get_org::ResponseData, Box<dyn Error>> {
    
    let request_body = GetOrg::build_query(crate::get_org::Variables {});

    return post_to_nvaapi::<
        crate::get_org::Variables,
        crate::get_org::ResponseData,
        crate::get_org::ResponseData
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


#[cfg(any(feature = "tokio", feature = "blocking"))]
pub fn q_getOrgId(
  send_into: Sender<crate::get_org::ResponseData>, 
  nvacl: &NavAbilityClient,
) {
  // wasmbindgen limitation?  overcome +'static requirement
  crate::execute(async move {
    let _ = send_api_result(
      send_into, 
      post_org_id(&nvacl).await,
    );
  });
}


#[cfg(feature = "wasm")]
pub fn q_getOrgId(
  send_into: Sender<crate::get_org::ResponseData>, 
  nvacl: &NavAbilityClient,
) {
  // wasmbindgen limitation?  overcome +'static requirement
  let nvacl_ = nvacl.clone();
  let send_into_ = send_into.clone();
  crate::execute(async move {
    let _ = send_api_result(
      send_into_, 
      post_org_id(&nvacl_).await,
    );
  });
}

