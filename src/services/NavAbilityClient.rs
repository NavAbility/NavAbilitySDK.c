
#[cfg(any(feature = "tokio", feature = "blocking"))]
use crate::{
    Uuid,
    Client,
    entities::NavAbilityClient,
    GetId,
};


#[cfg(any(feature = "tokio", feature = "blocking"))]
impl GetId for NavAbilityClient {
    fn getId(
        &self, 
        labels: &str
    ) -> Uuid {
        Uuid::new_v5(
            &Uuid::parse_str(&self.user_label).expect("Uuid string parse error"), 
            labels.as_bytes()
        )
    }
}


#[cfg(any(feature = "tokio", feature = "blocking"))]
impl NavAbilityClient {
    pub fn getOrgId(
        &self
    ) -> Uuid   {
        if self.user_label.is_empty() {
            #[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
            crate::execute(crate::services::post_org_id(&self))
            .expect(&format!(
                "Error, unable to get OrgId with NavAbilityClient\napi_url:{}\ntoken:{}\n",
                self.apiurl,
                if self.nva_api_token.is_empty() {"__null__"} else {"***"}
            ));
        }
        return Uuid::parse_str(&self.user_label)
        .expect("Error, unable to parse OrgId Uuid from NavAbilityClient string");
    }

    pub fn new(
        nva_api_url: &String, 
        nva_api_token: &String,
        org_id: Option<&String>
    ) -> Self {
        // FIXME good header.insert example: https://medium.com/@itsuki.enjoy/post-file-using-multipart-form-data-in-rust-5171ae57aeed
        //   or https://users.rust-lang.org/t/how-to-upload-a-file-using-rust-or-some-library/45423/4
        let client = Client::builder()
        .user_agent("graphql-rust/0.12.0")
        .default_headers(
                // TODO use HeaderMap: https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.headers
                // TODO use bearer auth: https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.bearer_auth
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", nva_api_token))
                        .unwrap(),
                )).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str("https://navability.io")
                            .unwrap(),
                    ))
                ).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str(&nva_api_url)
                            .unwrap(),
                    ))
                ).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str(&nva_api_url.replace("api.","app."))
                            .unwrap(),
                    ))
                )
                .collect(),
            )
            .build()
            .expect("Failure to create client");

        let mut temp = NavAbilityClient {
            client,
            apiurl: nva_api_url.to_string(),
            user_label: "".to_string(), //user_label.to_string(),
            nva_api_token: nva_api_token.to_string(),
        };

        // also cover wasm case
        let mut oid = org_id.unwrap_or(&"".to_string()).to_string();
        #[cfg(any(feature = "tokio", feature = "blocking", feature = "thread"))]
        if org_id.is_none() {
            oid = crate::execute(crate::services::post_org_id(
                &temp
            )).expect("Error, unable to get OrgId from NavAbilityClient")
            .to_string();
        }
    
        temp.user_label = oid;

        return temp;
    }
}

