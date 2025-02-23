
/// Utility functions and common tools for the NavAbility SDK.
///
/// This module provides various utility functions and common tools used throughout the NavAbility SDK,
/// including functions for type introspection, string parsing, console logging, and handling GraphQL query responses.


use std::{
  fmt,
  any::type_name,
  convert::TryInto,
  future::Future
};

use serde::{Serialize,Deserialize};

// use graphql_client::GraphQLQuery;
use crate::{
  Error, 
  Sender, 
  Response
};

#[cfg(feature = "wasm")]
use wasm_bindgen_futures;
#[cfg(feature = "tokio")]
use tokio;


#[cfg(feature = "wasm")]
pub fn execute<F>(
  future: F
) where 
  // https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/fn.spawn_local.html
  F:  Future<Output = ()>  + 'static 
{
  wasm_bindgen_futures::spawn_local(future);
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
    write!(f, "API response empty error {:?}", self.details)
  }
}

impl Error for GQLResponseEmptyError {}

#[derive(Debug)]
pub struct GQLResponseErrors {
  details: Vec<graphql_client::Error>,
}

impl fmt::Display for GQLResponseErrors {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "API response has error {:?}", self.details)
  }
}

impl Error for GQLResponseErrors {}

#[derive(Debug)]
pub struct GQLRequestError {
  pub details: String,
}

impl fmt::Display for GQLRequestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NvaSDK, API request error {}", self.details)
  }
}

impl Error for GQLRequestError {}


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
        return Err(Box::new(
          GQLResponseErrors {
            details: resbody.errors.unwrap()
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

/// Sends the result of a GraphQL query to a given sender.
///
/// # Arguments
///
/// * `send_into` - A sender to which the query result will be sent.
/// * `response_body` - A `Result` containing the response body of the GraphQL query.
pub fn send_query_result<F,T>(
  send_into: Sender<T>,
  response_body: Result<Response<F>,Box<dyn Error>>,
  fn_modifier: fn(F) -> T,
) -> Result<(),Box<dyn Error>> {
  match check_query_response_data(response_body, fn_modifier) {
    Ok(data) => {
      // let _ = send_into.send(data);
      if let Err(e) = send_into.send(data) {
        to_console_error(&format!("Error sending data on channel: {:?}", e));
      };
      return Ok(())
    },
    Err(e) => {
      return Err(e)
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



/// Checks the deserialization result of a GraphQL query response.
///
/// # Arguments
///
/// * `serde_res` - A `Result` containing the deserialization result of the response.
///
/// # Returns
///
/// * `Result<Response<T>, Box<dyn Error>>` - A `Result` containing the deserialized response or an error.
#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
pub fn check_deser<T>(
  serde_res: Result<Response<T>,reqwest::Error>
) -> Result<Response<T>,Box<dyn Error>> {
  
  if let Err(ref e) = serde_res {
    to_console_error(&format!("JSON unpack failure from possibly good API response: {:?}", &e));
  }
  
  return Ok(serde_res?)
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
  
  // TBD
  // let query = MyQuery::build_query(my_query::Variables {});
  // match query {
  //     Ok(q) => () // println!("Query: {:?}", q),
  //     Err(e) => eprintln!("Failed to build query: {:?}", e),
  // }
  
  let mut trycount = retries.unwrap_or(3);
  while 0 < trycount {
    
    let req_res = nvacl.client
    .post(&nvacl.apiurl)
    .json(&request_body)
    .send().await;
    
    if let Err(ref re) = req_res {
      let erm = format!("API request error for {:?}: {:?}",  type_name::<V>(), &re);
      to_console_error(&erm);
    } else {
      // generic transport and serde error checks
      let response_body = check_deser::<R>(
        req_res?.json().await
      );
      
      // query response during error checks
      return check_query_response_data(response_body, fn_modifier);
    }
    trycount -= 1;
  }
  return Err(Box::new(crate::GQLRequestError { 
    details: format!("API request failed after {} retries {:?}", retries.unwrap_or(3), type_name::<V>()).to_owned()
  }));
}


// ====================== FUTURE IDEAS ======================





// missing traits for generic serde on query types
// async fn post_query_serde<V,R>(
//     nvacl: NavAbilityClient,
//     request_body: QueryBody<V>,
// ) -> Result<R,Box<dyn Error>> {
//     let req_res = nvacl.client
//     .post(&nvacl.apiurl)
//     .json(&request_body)
//     .send().await;

//     match req_res {
//         Err(re) => {
//             to_console_error(&format!("API request error: {:?}", re));
//             return Err(Box::new(re));
//         },
//         Ok(res) => {
//             let serde_res = res.json().await;
//             match serde_res {
//                 Ok(response_body) => {
//                     return Ok(response_body)
//                 },
//                 Err(e) => {
//                     to_console_error(&format!("JSON unpack of API response failed: {:?}", &e));
//                     return Err(Box::new(e));
//                 }
//             }
//         }
//     }
// }