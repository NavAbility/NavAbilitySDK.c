
/// Utility functions and common tools for the NavAbility SDK.
///
/// This module provides various utility functions and common tools used throughout the NavAbility SDK,
/// including functions for type introspection, string parsing, console logging, and handling GraphQL query responses.

use std::fmt;

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use std::{
  any::type_name,
  convert::TryInto,
  future::Future
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use serde::{Serialize,Deserialize};

// use graphql_client::GraphQLQuery;
use crate::{
  Error, 
  Sender, 
  Response
};

#[cfg(feature = "tokio")]
use tokio;



// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! genGetLabel { 
    ($T:ident) => {
        impl GetLabel for $T {
            fn getLabel(&self) -> &String { &self.label }
        }
    }
}


#[cfg(feature = "thread")]
pub fn execute<R, F: Future<Output = R>>(  //  + Send + 'static
f: F
) -> R {
  // use any executor of your choice instead
  std::thread::spawn(move || futures::executor::block_on(f));
}

#[cfg(feature = "tokio")]
pub fn execute<R,F: Future<Output = R>>(  // Result<R,Box<dyn Error>> // < + Send + 'static>
f: F
) -> R {
  // TODO, use any executor of your choice instead
  return tokio::runtime::Builder::new_current_thread()
  .enable_all()
  .build()
  .unwrap()
  .block_on(f);
  // std::thread::spawn(move || futures::executor::block_on(f));
}



/// Returns the type name of a given value.
///
/// # Arguments
///
/// * `_: T` - A value of any type.
///
/// # Returns
///
/// * `&'static str` - The name of the type of the given value.
pub fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}

/// Parses a string into a `chrono::DateTime<Utc>` object.
///
/// # Arguments
///
/// * `text` - A string representing a date and time in UTC.
///
/// # Returns
///
/// * `Result<chrono::DateTime<chrono::Utc>, chrono::ParseError>` - A `Result` containing the parsed `DateTime` object or a `ParseError`.
pub fn parse_str_utc(
  text: String
) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
  let tmstr = text
  .replace(" UTC"," +00")
  .replace("Z", " +00")
  .replace("T", " ");
  match chrono::DateTime::parse_from_str(&tmstr, "%Y-%m-%d %H:%M:%S%.f %#z") {
    Ok(tmsp) => {
      return Ok(tmsp.to_utc());
    }
    Err(e) => {
      to_console_error(&format!("Unable parse UTC datetime {} with error {:?}",text.to_string(),e));
      return Err(e)
    }
  }
}

// fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
//     v.try_into()
//         .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

/// Logs a debug message to the console.
///
/// # Arguments
///
/// * `text` - A string slice containing the debug message.
pub fn to_console_debug(
  text: &str
) {
  #[cfg(not(target_arch = "wasm32"))]
  println!("{}",text);
  // tracing::debug!("{}",text);
  #[cfg(target_arch = "wasm32")]
  gloo_console::log!(text.to_string());
}

/// Logs an error message to the console.
///
/// # Arguments
///
/// * `text` - A string slice containing the error message.
pub fn to_console_error(
  text: &str
) {
  #[cfg(not(target_arch = "wasm32"))]
  println!("ERROR NvaSDK.rs {}",&text);
  // tracing::error!("ERROR NvaSDK.rs {}",&text);
  #[cfg(target_arch = "wasm32")]
  gloo_console::log!(&format!("ERROR NvaSDK.rs {}",&text));
  // web_sys::console::log_1 // alternative
}



// ===================== COMMON QUERY TOOLS ======================


#[derive(Debug)]
pub struct GQLResponseEmptyError {
  details: String,
}

impl fmt::Display for GQLResponseEmptyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, GQLResponseEmptyError {:?}", self.details)
  }
}

impl Error for GQLResponseEmptyError {}

#[derive(Debug)]
pub struct GQLResponseErrors {
  details: Vec<graphql_client::Error>,
}

impl fmt::Display for GQLResponseErrors {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, GQLResponseErrors {:?}", self.details)
  }
}

impl Error for GQLResponseErrors {}

#[derive(Debug)]
pub struct GQLRequestError {
  pub details: String,
}

impl fmt::Display for GQLRequestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, GQLRequestError {}", self.details)
  }
}

impl Error for GQLRequestError {}


#[derive(Debug)]
pub struct GQLResponseUnfamiliar {
  pub body: String,
  pub error: String,
}

impl fmt::Display for GQLResponseUnfamiliar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, GQLResponseUnfamiliar error: {:?}\nbody: {}", self.error, self.body)
  }
}

impl Error for GQLResponseUnfamiliar {}

#[derive(Debug)]
pub struct GQLUnauthenticated;

impl fmt::Display for GQLUnauthenticated {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, GQLUnauthenticated")
  }
}

impl Error for GQLUnauthenticated {}


/// Checks the ResponseData: F of a GraphQL query and applies a user specified modifier callback.
///
/// # Arguments
///
/// * `response_body` - A `Result` containing the response body of the GraphQL query.
///
/// # Returns
///
/// * `Result<T, Box<dyn Error>>` - A `Result` containing the data of the response or an error.
pub fn check_query_response_data<F,T>(
  response_body: Result<Response<F>,Box<dyn Error>>,
  fn_modifier: fn(F) -> T,
) -> Result<T,Box<dyn Error>> {
  match response_body {
    Ok(resbody) => {
      if resbody.errors.is_none() {
        match resbody.data {
          Some(data) => {
            return Ok(fn_modifier(data))
          }
          None => {
            to_console_error(&"API query response data is empty.");
            return Err(Box::new(GQLResponseEmptyError {
              details: "API post query error: response data is empty.".to_owned(),
            }))
          },
        }
      } else {
        to_console_error(&format!("API post query error: response errors exist: {:?}", &resbody.errors));
        let details = resbody.errors.as_ref().unwrap();
        let errs = details
          .iter()
          .map(|e| e.to_string())
          .collect::<Vec<String>>();
        if errs.len() == 1 {
          if errs[0].to_uppercase().eq("UNAUTHENTICATED") {
            // Special case for unauthenticated error
            to_console_error("API post query error: UNAUTHENTICATED");
            return Err(Box::new(GQLUnauthenticated {}));
          }
        }
        return Err(Box::new(
          GQLResponseErrors {
            details: details.to_vec()
          }
        ));
      }
    }
    Err(e) => {
      to_console_error(&format!("failure before check_query_response_data: {:?}",&e));
      return Err(e);
    }
  }
}


pub fn send_api_result<T>(
  send_into: Sender<T>,
  api_result: Result<T,Box<dyn Error>>,
) -> Result<(),Box<dyn Error>> {
  match api_result {
    Ok(data) => {
      match send_into.send(data) {
        Ok(_) => {
          return Ok(())
        },
        Err(e) => {
          // TODO upgrade to impl TryFrom: https://www.reddit.com/r/rust/comments/bu2fmn/how_print_a_generic_type_debugt/?rdt=63064
          let erm = format!(
            "Error Sender<{}> data on std::mspc::sync::channel: {:?}", 
            std::any::type_name::<T>(), 
            &e
          );
          to_console_error(&erm);
          return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, erm)));
        }
      }
    }
    Err(e) => {
      to_console_error(&format!("send_api_result cannot send error {:?}", &e));
      return Err(e);
    }
  }
}






#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_to_nvaapi_cb<
  R: for<'de> Deserialize<'de>,
  T
>(
  fn_modifier: fn(R) -> T,
  retries: Option<i32>,
  post_req: reqwest::RequestBuilder
) -> Result<T, Box<dyn Error>> {
  // Note, this function allows request body json splicing for incomplete GQL types

  let mut trycount = retries.unwrap_or(3);
  while 0 < trycount {

    let request_response = post_req.try_clone()
    .expect("Unable to clone request")
    .send().await;
    
    match request_response { 
      Err(re) => {
        let erm = format!("API request error: {:?}", &re);
        to_console_error(&erm);
      },
      Ok(request_response_) => {
        // generic transport and serde error checks
        // https://docs.rs/reqwest/latest/src/reqwest/async_impl/response.rs.html#267-271
        let response_bytes = request_response_.bytes().await?;
        let response_body = serde_json::from_slice(&response_bytes); //.map_err(reqwest::error::decode);
        // let response_body: Result<Response<R>, reqwest::Error> = request_response_.json().await;
        if let Err(ref e) = response_body {
          let err = format!("JSON unpack failure from possibly good API response, maybe check response type scalar definition: {:?}", &e);
          let body = String::from_utf8_lossy(&response_bytes);
          to_console_error(&err);
          return Err(Box::new(crate::GQLResponseUnfamiliar {
            body: format!("{}", body.to_string()),
            error: err.to_string()
          }));
        }

        // query response during error checks
        return check_query_response_data(
          response_body.map_err(|e| Box::new(e) as Box<dyn Error>), // changing reqwest error unnecessarily, consolidation artifact, TODO simplify
          fn_modifier
        );
      }
    }
    trycount -= 1;
  }
  return Err(Box::new(crate::GQLRequestError { 
    details: format!("API request failed after {} retries", retries.unwrap_or(3)).to_owned()
  }));
}



#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub async fn post_to_nvaapi<
  V: Serialize,
  R: for<'de> Deserialize<'de>,
  T
>(
  nvacl: &crate::NavAbilityClient,
  request_body: crate::QueryBody<V>,
  fn_modifier: fn(R) -> T,
  retries: Option<i32>
) -> Result<T, Box<dyn Error>> {

  let post_req = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body);

  return post_to_nvaapi_cb::<R,T>(
    fn_modifier,
    retries,
    post_req
  ).await;
}



// ====================== FUTURE IDEAS ======================


// fn get_fnc_name(fnc: &str) -> String {
//   let parts = fnc.split(".");
//   let mut t = "";
//   for part in parts {
//     t = part;
//   }
//   return t.to_owned();
// }

