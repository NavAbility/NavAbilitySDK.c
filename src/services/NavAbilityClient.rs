
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    Uuid,
    Client,
    entities::NavAbilityClient,
    GetId,
};


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
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


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl NavAbilityClient {
    pub fn new(
        apiurl: &String, 
        user_label: &String, 
        nva_api_token: &String
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
                        reqwest::header::HeaderValue::from_str(&apiurl)
                            .unwrap(),
                    ))
                ).chain(
                    std::iter::once((
                        reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        reqwest::header::HeaderValue::from_str(&apiurl.replace("api.","app."))
                            .unwrap(),
                    ))
                )
                .collect(),
            )
            .build()
            .expect("Failure to create client");

        NavAbilityClient {
            client,
            apiurl: apiurl.to_string(),
            user_label: user_label.to_string(),
            nva_api_token: nva_api_token.to_string(),
        }
    }
}

