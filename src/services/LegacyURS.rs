
#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::{
    // Utc,
    // Uuid,
    // Sender,
    GraphQLQuery,
    Response,
    Error,
    // SDK_VERSION,
    GetURS,
    get_urs,
    to_console_error,
};

#[cfg(feature = "wasm")]
use crate::to_console_debug;

#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
use crate::NavAbilityClient;


#[cfg(feature = "blocking")]
pub fn get_robots_blocking(client: &NavAbilityClient) -> get_robots::ResponseData {
    let variables = get_robots::Variables {
        user_label: client.user_label.clone(),
    };

    let response_body =
        post_graphql_blocking::<GetRobots, _>(&client.client, &client.apiurl, variables)
            .expect("Failure to post graphql");
    
    //debug print raw response body
    dbg!(&response_body);

    let response_data: get_robots::ResponseData =
        response_body.data.expect("missing response data");

    return response_data;
}



#[cfg(feature = "blocking")]
pub fn fetch_ur_list_blocking(
    send_into: &mut crate::Sender<Vec<get_robots::GetRobotsUsers>>, 
    nvacl: &NavAbilityClient
) -> Result<(),Box<dyn Error>> {

    // THIS IS THE LEGACY VERSION IN SDK FIXME TO NEW VERSION FOR WEB/WASM
    let ur_list = get_robots_blocking(&nvacl).users;
    // dbg!(&ur_list);

    if let Err(e) = send_into.send(ur_list) {
        tracing::error!("Error sending user robot list data: {}", e);
    };

    Ok(())
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "blocking"))]
pub async fn fetch_urs_async(
    nvacl: &NavAbilityClient,
    // robot_label: String,
    // session_label: String,
) -> Result<Response<get_urs::ResponseData>, Box<dyn Error>> {

    let variables = get_urs::Variables {
        org_id: nvacl.user_label.to_string(),
    };

    let request_body = GetURS::build_query(variables);

    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;

    if let Err(ref re) = req_res {
        to_console_error(&format!("API request error: {:?}", re));
    }

    return crate::check_deser::<get_urs::ResponseData>(
        req_res?.json().await
    )
}




#[cfg(feature = "wasm")]
pub async fn fetch_context_web(
    send_into: crate::Sender<Vec<get_urs::GetUrsOrgs>>, 
    client: &NavAbilityClient,
    _robot_label: String,
    _session_label: String,
) { // -> Vec<get_robots::GetRobotsUsers> {
    let result = fetch_urs_async(&client).await;
    // FIXME use new common query functions refactor .orgs part
    if let Ok(response_body) = result {
        let res_errs = response_body.errors;
        match res_errs {
            Some(ref err) => {
                to_console_error(&format!("fetch_context_web has response errors {:?}",&err));
            },
            None => {
                let urs_data = response_body.data;
                match urs_data {
                    None => to_console_debug(&"GQL response_body.data is empty"),
                    Some(resdata) => {
                        let urs_data = resdata.orgs;
                        let res_len = urs_data.len();
                        to_console_debug(&format!("length of context send_into.send {}", res_len));  
        
                        let resp = send_into.send(urs_data);
                        if let Err(e) = resp {
                            to_console_error(&format!("Error sending user robot list data: {}", e));
                        }
                    }
                }
            }
        }
    } else {
        to_console_error(&"Unable to fetch list from client connection");
    }

}

