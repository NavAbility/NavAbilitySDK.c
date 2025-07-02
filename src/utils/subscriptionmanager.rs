
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
use reqwest_eventsource::{
  EventSource,
  Event,
};

#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
use url::form_urlencoded;

#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
use futures::stream::StreamExt;

#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
use std::{
  sync::mpsc::{
    Sender, 
    Receiver, 
    channel
  },
  collections::{
    BTreeSet,
    BTreeMap,
  },
};


#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm", feature = "blocking"))]
use crate::{
  Uuid,
  GraphQLQuery,
  NavAbilityClient,
  to_console_debug, 
  to_console_error,
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkerStatusEnum {
  Pending,
  Status(String),
  Unknown(String),
}


/// Manages subscription events from NavAbilityClient subscriptions
/// SPECIAL NOTE1, can use standalone Self::subscription_listener(_)
/// SPECIAL_NOTE2, both non-blocking and blocking interfaces are provided (for wasm or tokio)
#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
pub struct SubscriptionManager {
  /// Keep track of work requests / events by their UUID
  events: BTreeMap<Uuid, Option<crate::default_subscription::ResponseData>>,
  /// Keep history of last `size` for n-many received events (SSEs)
  sse_history: Vec<Uuid>,
  /// Maximum size of sse_history
  size: usize,
  /// Receive channel for subscription events (polling interface)
  nonblocking_recv: Receiver<crate::default_subscription::ResponseData>, 
  /// Direct notifications channel for individual uuid blocking user requests
  blocking_into: Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
}


#[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
impl SubscriptionManager {
  /// Create a new SubscriptionManager, starts a subscription listener that sends events into an internal channel
  pub fn from_parts(
    nvacl: &NavAbilityClient,
    size: usize,
    nonblocking_recv: Receiver<crate::default_subscription::ResponseData>,
    blocking_into: Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
  ) -> Self {

    let nvasm = Self {
      events: BTreeMap::new(),
      sse_history: Vec::new(),
      size,
      nonblocking_recv,
      blocking_into,
    };

    return nvasm;
  }

  pub fn new_channels(
  ) -> (
    (
      Sender<crate::default_subscription::ResponseData>,
      Receiver<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
    ),(
      Receiver<crate::default_subscription::ResponseData>,
      Sender<(Uuid, Sender<crate::default_subscription::ResponseData>)>,
    )
  ) {
    // common channel for received subscription events (fullied by polling)
    let (nonblocking_into, nonblocking_recv) = channel();
    // specific user request channel for setting up direct (blocking) notifications per user Uuid
    let (blocking_into, blocking_recv) = channel();

    ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into))
  }

  pub fn new(
    nvacl: &NavAbilityClient,
    size: usize,
  ) -> Self {
    // create the channels for non-blocking and blocking interfaces
    let ((nonblocking_into, blocking_recv), (nonblocking_recv, blocking_into)) = Self::new_channels();

    let nvasm = Self::from_parts(nvacl, size, nonblocking_recv, blocking_into);

    Self::subscription_listener(
      nonblocking_into,
      nvacl,
      blocking_recv,
    );

    return nvasm;
  }


  /// List all tracked UUIDs, returning a tuple of:
  /// (set of pending UUIDs, map of Status UUIDs to their status, map of unknown UUIDs to their status)
  /// NOTE, user is responsible for calling try_recv() to update the internal state before calling this (requires mutable self)
  pub fn build_summary(
    &self
  ) -> (BTreeSet<Uuid>, BTreeMap<Uuid, WorkerStatusEnum>, BTreeMap<Uuid, WorkerStatusEnum>) {
    // self.try_recv();
    let mut pending_set: BTreeSet<Uuid> = BTreeSet::new();
    let mut status_map: BTreeMap<Uuid, WorkerStatusEnum> = BTreeMap::new();
    let mut unknown_map: BTreeMap<Uuid, WorkerStatusEnum> = BTreeMap::new();
    for (id, _) in &self.events {
      match self.get_status(id) {
        WorkerStatusEnum::Pending => { pending_set.insert(*id); },
        WorkerStatusEnum::Status(s) => { status_map.insert(*id, WorkerStatusEnum::Status(s)); },
        WorkerStatusEnum::Unknown(s) => { unknown_map.insert(*id, WorkerStatusEnum::Unknown(s)); },
      }
    }
    return (pending_set, status_map, unknown_map);
  }


  /// Get the maximum history size
  pub fn max_history(
    &self
  ) -> usize {
    return self.size;
  }

  /// Check if a given UUID is being tracked
  pub fn contains(
    &self, 
    id: &Uuid
  ) -> bool {
    return self.events.contains_key(id);
  }


  /// Add a given UUID to be tracked, initial status is None/Pending
  pub fn add_tracking(
    &mut self, 
    id: &Uuid
  ) {
    // tell the async process user wants to be notified via a channel

    // include this UUID in the tracking map (as WorkerStatusEnum::Pending)
    self.events.insert(id.clone(), None);

  }

  /// Check if a given event UUID is trackable then return Some(true/false),
  /// ELSE return None if not trackable/something else is wrong
  pub fn has_status(
    &self, 
    id: &Uuid
  ) -> Option<bool> {
    return if self.contains(id) { 
      Some(self.events.get(id).unwrap().is_some())
    } else {
      None
    }
  }


  /// Check if a given event UUID is done (status "DONE/done" or "SUCCESS/success"), return Some(true/false),
  /// ELSE return None if not trackable/something else is wrong
  pub fn is_status_success(
    &self,
    id: &Uuid
  ) -> Option<bool> {
     return Self::is_enum_success(&self.get_status(id));
  }

  pub fn is_enum_success(
    wse: &WorkerStatusEnum
  ) -> Option<bool>{
    return match wse {
      WorkerStatusEnum::Pending => Some(false),
      WorkerStatusEnum::Status(s) => {
        if s == "DONE" || s == "done" || s == "SUCCESS" || s == "success" {
          Some(true)
        } else {
          Some(false)
        }
      },
      WorkerStatusEnum::Unknown(_) => None,
    }
  }

  /// Get the status of a given UUID as WorkerStatusEnum
  pub fn get_status(
    &self, 
    id: &Uuid
  ) -> WorkerStatusEnum {
    // if Some(true/false)=>trackable ELSE None=>something else is wrong
    if let Some(status) = self.has_status(id) {
      if !status {
        return WorkerStatusEnum::Pending;
      }
      if let Some(req_data_) = self.events.get(id).as_ref() {
        if let Some(req_data) = req_data_ {
          if let Some(we) = &req_data.worker_event {
            return WorkerStatusEnum::Status(we.status.to_string());
          }
        }
        return WorkerStatusEnum::Unknown(format!("{:?}",req_data_));
      }
    }
    return WorkerStatusEnum::Unknown("Untracked or other problem".to_owned());
  }

  /// Try to receive any pending subscription events, store in events map and sse_history
  pub fn poll(&mut self) {
    while let Ok(subscr) = self.nonblocking_recv.try_recv() {
      // to_console_debug(&format!("Got subscription response {:?}",&subscr));
      if let Some(subs) = &subscr.worker_event {
        // to_console_debug(&format!("Got subscription for sync_subscription {:?}",&subs));
        let idstr = &subs.id;
        if let Ok(id) = Uuid::parse_str(idstr) {
          self.events.insert(id, Some(subscr));
          self.sse_history.push(id);
          if self.sse_history.len() > self.size {
            let old_id = self.sse_history.remove(0);
            self.events.remove(&old_id);
          }
        } else {
          to_console_error(&format!("Failed to parse UUID from subscription event id string '{}'",idstr));
        }
      }
    }
  }

  /// Block on a specific UUID subscription event, waiting for a response
  /// Returns Some(true/false) if the subscription event was successful, otherwise None for other errors
  pub fn block_on(
    &mut self,
    wid: &Uuid,
    tout_millis: std::time::Duration,
  ) -> Option<bool> {
    // if using both polling and blocking, add the id to the polling track map
    self.add_tracking(wid);
    // if using blocking, send the request to the channel via standalone function
    return Self::block_on_standalone(
      self.blocking_into.clone(),
      wid,
      tout_millis,
    );
  }

  /// block on a specific UUID subscription event, waiting for a response
  /// Returns Some(true/false) if the subscription event was successful, otherwise None for other errors
  pub fn block_on_standalone(
    blocking_into: Sender<(Uuid,Sender<crate::default_subscription::ResponseData>)>,
    wid: &Uuid,
    tout_millis: std::time::Duration,
  ) -> Option<bool> {

    let (etx, erx) = channel();
    let _ = blocking_into.send((wid.clone(), etx));
    match erx.recv_timeout(tout_millis) {
      Ok(subscr) => {
        if let Some(we) = subscr.worker_event.as_ref() {
          return Self::is_enum_success(&WorkerStatusEnum::Status(we.status.to_string()));
        }
      },
      Err(e) => {
        to_console_error(&format!("Failed to receive subscription response: {}", e));
      }
    }
    return None; // failure
  }


  /// Start a subscription listener that sends received events into the provided channel
  /// DOES NOT REQUIRE a SubscriptionManager instance, can be used as standalone function
  #[cfg(any(feature = "tokio", feature = "thread", feature = "wasm"))]
  pub fn subscription_listener(
      nonblocking_into: Sender<crate::default_subscription::ResponseData>,
      nvacl: &NavAbilityClient,
      blocking_recv: Receiver<(Uuid,Sender<crate::default_subscription::ResponseData>)>,
  ) {
    // NOTE couldn't use eventsource_reqwest for sse get requests -- missing header support
    let nvacl_e = NavAbilityClient::similar(
      nvacl,
      true
    );
    // use the SDK's existing subscription query definition
    let query = crate::DefaultSubscription::build_query(
      crate::default_subscription::Variables {}
    );
    // https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/
    let query_string = form_urlencoded::byte_serialize(query.query.as_bytes())
      .collect::<String>();
    // let query_string = "subscription%7BworkerEvent%7Bpayload+id+status%7D%7D";
    let uri = format!("{}?query={}", nvacl.apiurl, query_string);

    let mut nvaes = EventSource::new(
      nvacl_e.client.get(&uri)
    ).expect("Failed to create EventSource");

    // wasmbindgen limitation?  overcome +'static requirement

    // start a thread to run the async event loop monitoring the EventSource 
    // and send received eventes into the channel
    crate::execute(async move {
      let mut please_notify: BTreeMap<Uuid, Sender<crate::default_subscription::ResponseData>> = BTreeMap::new();
      while let Some(event) = nvaes.next().await {
        // pull any direct user request uuids
        match blocking_recv.try_recv() {
          Ok((uuid, sender)) => {
            please_notify.insert(uuid, sender);
          },
          Err(_) => {}
        }

        // process the event
        match event {
          Ok(Event::Open) => to_console_debug("SSE connection Open!"),
          Ok(Event::Message(message)) => {
            let msg_: Result<
              serde_json::Map<String, serde_json::Value>,
              serde_json::Error
            > = serde_json::from_str(&message.data);
            if let Err(e) = msg_ {
              to_console_error(&format!("Failed to parse message data: {}", e));
              continue;
            }
            let msg = msg_.unwrap();
            if let Some(jobj) = msg.get("data") { //msg.contains_key("data") {
              // to_console_debug(&format!("{:?}",&msg));
              let jstr = jobj.to_string();
              let jobj_: Result<
                crate::default_subscription::ResponseData, 
                serde_json::Error
              > = serde_json::from_str(&jstr);
              if let Ok(subwe) = jobj_ {
                use uuid::Uuid;

                nonblocking_into.send(
                  subwe.clone()
                ).expect("Failed to send Event");
                // notify any direct uuids requested by the user
                let ewid = Uuid::parse_str(&subwe.worker_event.as_ref().unwrap().id).expect("Failed to parse UUID from worker event id");
                if let Some(sender) = please_notify.remove(&ewid) {
                  sender.send(subwe).expect(&format!("Failed to send direct notification for UUID {}", &ewid));
                }
              } else {
                to_console_error("Failed to parse 'data' from message data");
              }
            }
          },
          Err(err) => {
            to_console_error(&format!("Error: {}", err));
            nvaes.close();
          }
        }
      }
    });
  }
}
