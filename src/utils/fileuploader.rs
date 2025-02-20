
use crate::Uuid;
use crate::to_console_error;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::NavAbilityClient;
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::Client;
// #[cfg(any(feature = "tokio", feature = "wasm"))]
// use reqwest::Client;
// #[cfg(feature="blocking")]
// use ::reqwest::blocking::Client;
// #[cfg(feature="blocking")]
// use graphql_client::reqwest::post_graphql_blocking;


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[derive(Debug,Clone)]
#[allow(non_snake_case)]
pub struct FileUploader<T> {
    nvacl: NavAbilityClient,
    pub file: T, // assume read and seek are available
    blobId: Uuid,
    chunk_size: u64,
    nbytes_uploaded: u64,
}

// TODO , feature = "blocking"
#[cfg(any(feature = "tokio", feature = "wasm"))]
#[allow(non_snake_case)]
impl<T> FileUploader<T> {
    pub fn new(
        nvacl: NavAbilityClient,
        file: T,
        label: String,
        blobId: Uuid,
        chunk_size: Option<u64>,
    ) -> Self {
        // create the actual uploader object
        Self {
            nvacl,
            file,
            blobId,
            chunk_size: chunk_size.expect("FileUpload expects chunk_size as u64"),
            nbytes_uploaded: 0 as u64,
        }
    }

    pub async fn upload_file(
        &mut self,
        content: Vec<u8>,
        url_endpoint: String
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_LENGTH, 
            reqwest::header::HeaderValue::from(content.len())
        );
        // headers.insert(
        //     reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        //     reqwest::header::HeaderValue::from("https://_mySendIP???_")
        // );
        
        // PUT POST OPTIONS CORS: https://aws.amazon.com/blogs/media/deep-dive-into-cors-configs-on-aws-s3-how-to/
        let response = Client::new()
            .put(url_endpoint)
            .headers(headers)
            .body(content)
            .send()
            .await?;
            
            // // .multipart(file)
            // to_console_debug(&format!("inner header {:?}", &postclient));
            
            let status_code = response.status();
            if reqwest::StatusCode::OK == status_code {
                let res_head = response.headers();
                let etag = res_head["etag"].to_str().unwrap().replace("\"","");
                // to_console_debug(&format!("Headers:\n{:#?}", response.headers()));
                // to_console_debug(&format!("Body:\n{}", response.text().await?));
                return Ok(etag)
            } else {
                to_console_error(&format!("Status: {:?}", &status_code));
                return Err(format!("Upload file put returned Status: {:?}", status_code).into())
            }
    }
}



// type CompletedUploadPartInput {
//   partNumber: Int!
//   eTag: String!
// }
// type CompletedUploadInput {
//   uploadId: String!
//   parts: [CompletedUploadPartInput!]!
// }
