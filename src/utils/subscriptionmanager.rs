

use reqwest_eventsource::{
  EventSource,
  Event,
};
use url::form_urlencoded;

use futures::stream::StreamExt;

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


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
  Uuid,
  GraphQLQuery,
  QueryBody,
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
/// SPECIAL NOTE, can use standaline Self::subscription_listener(_)
#[cfg(any(feature = "tokio", feature = "wasm"))]
pub struct SubscriptionManager {
  /// Keep track of work requests / events by their UUID
  events: BTreeMap<Uuid, Option<crate::default_subscription::ResponseData>>,
  /// Keep history of last `size` for n-many received events (SSEs)
  sse_history: Vec<Uuid>,
  /// Maximum size of sse_history
  size: usize,
  /// Receive channel for subscription events
  subs_recv: Receiver<crate::default_subscription::ResponseData>, 
}


#[cfg(any(feature = "tokio", feature = "wasm"))]
impl SubscriptionManager {
  /// Create a new SubscriptionManager, starts a subscription listener that sends events into an internal channel
  pub fn new(
    nvacl: &NavAbilityClient,
    size: usize,
  ) -> Self {
    let (send_into, recv_from) = channel();
    Self::subscription_listener(
      send_into,
      nvacl,
    );
    return Self {
      events: BTreeMap::new(),
      sse_history: Vec::new(),
      size,
      subs_recv: recv_from,
    };
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
    id: Uuid
  ) {
    self.events.insert(id, None);
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
    return match self.get_status(id) {
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
    if let Some(Status) = self.has_status(id) {
      if !Status {
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
    while let Ok(subscr) = self.subs_recv.try_recv() {
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


  /// Start a subscription listener that sends received events into the provided channel
  /// DOES NOT REQUIRE a SubscriptionManager instance, can be called standalone
  #[cfg(any(feature = "tokio", feature = "wasm"))]
  pub fn subscription_listener(
      send_into: Sender<crate::default_subscription::ResponseData>,
      nvacl: &NavAbilityClient,
  ) {
    // FIXME upgrade to eventsource_reqwest for sse
    let nvacl_e = NavAbilityClient::similar(
      nvacl,
      true
    );
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
      while let Some(event) = nvaes.next().await {
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
                send_into.send(
                  subwe
                ).expect("Failed to send Event");
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
