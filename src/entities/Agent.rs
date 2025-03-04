
use std::collections::HashMap;

use crate::{
  BlobEntry, 
  NvaNode,
  Model,
  Factorgraph,
  Utc, 
  Uuid, 
  SDK_VERSION 
};

#[derive(Debug, Clone, Default)]
#[allow(non_snake_case)]
pub struct Agent {
    pub id: Option<Uuid>,
    pub label: String,
    pub description: String,
    pub tags: Vec<String>,
    pub _version: String,
    pub createdTimestamp: chrono::DateTime::<Utc>,
    pub lastUpdatedTimestamp: Option<chrono::DateTime::<Utc>>,
    pub metadata: Option<String>,
    pub blobEntries: Option<HashMap<String, BlobEntry>>,
    pub models: Option<Vec<(String,chrono::DateTime<Utc>)>>,
    pub fgs: Option<Vec<(String,chrono::DateTime<Utc>)>>,
}

impl Agent {
  pub fn new(
    org_id: &Uuid,
    label: String,
    description: String,
    tags: Vec<String>,
    createdTimestamp: chrono::DateTime<Utc>,
  ) -> Self {
    let mut ag = Self::default();
    // mutate the non-default fields per new inputs
    ag.id = Some(Uuid::new_v5(org_id, label.as_bytes()));
    ag.label = label;
    ag.description = description;
    ag.tags = tags;
    ag._version = SDK_VERSION.to_string();
    ag.lastUpdatedTimestamp = Some(createdTimestamp.clone());
    ag.createdTimestamp = createdTimestamp;

    return ag;
  }
}

pub trait AgentFieldImportersSummary {
  fn id(&self) -> Option<Uuid>;
  fn label(&self) -> String;
  fn description(&self) -> String;
  fn tags(&self) -> Vec<String>;
  fn _version(&self) -> String;
  fn createdTimestamp(&self) -> chrono::DateTime<Utc>;
  fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>>;
}

// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! Agent_importers_summary { 
  ($T:ident) => {
    impl AgentFieldImportersSummary for $T {
      fn id(&self) -> Option<Uuid> { Some(Uuid::parse_str(&self.id).expect("failed to parse blobentry id to uuid")) }
      
      fn label(&self) -> String { self.label.to_string() }

      fn description(&self) -> String { 
        return self.description.clone().unwrap_or("".to_owned()).to_string();
      }

      fn tags(&self) -> Vec<String> { 
        return self.tags.clone();
      }

      fn _version(&self) -> String { 
        return self.version.to_string();
      }

      fn createdTimestamp(&self) -> chrono::DateTime<chrono::Utc> {
        let timestamp = &self.created_timestamp;
        match parse_str_utc(timestamp.clone()) {
          Ok(tms) => { 
            return tms;
          },
          Err(e) => {
            let errm = format!("AgentImporter, createdTimestamp using default.now(utc) since chrono parse_from_str failed at timestamp {:?} with error {:?}",timestamp,e);
            to_console_error(&errm);
            return chrono::Utc::now();
          }
        }
      }

      fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>> {
        let timestamp = &self.last_updated_timestamp;
        match parse_str_utc(timestamp.clone()) {
          Ok(tms) => { 
            return Some(tms);
          },
          Err(e) => {
            let errm = format!("AgentImporter, lastUpdatedTimestamp failed chrono parse_from_str timestamp {:?} with error {:?}",timestamp,e);
            to_console_error(&errm);
            return None;
          }
        }
      }
    }
  }
}



pub trait AgentFieldImportersFull {
  fn metadata(&self) -> Option<String>;
  fn blobEntries(&self) -> Option<HashMap<String, BlobEntry>>;
  fn models(&self) -> Option<Vec<(String,chrono::DateTime<Utc>)>>;
  fn fgs(&self) -> Option<Vec<(String,chrono::DateTime<Utc>)>>;
}

// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! Agent_importers_full { 
  ($T:ident) => {
    impl AgentFieldImportersFull for $T {
      fn metadata(&self) -> Option<String> { self.metadata.clone() }

      fn blobEntries(&self) -> Option<HashMap<String, BlobEntry>> {
        let mut blob_entries = HashMap::new();
        for ge in &self.blob_entries {
          let ne = BlobEntry::from_gql_summary(ge);
          blob_entries.insert(ne.label.clone(), ne);
        }
        return Some(blob_entries);
      }

      fn models(&self) -> Option<Vec<(String,chrono::DateTime<Utc>)>> {
        let mut mdls = Vec::new();
        for ms in &self.models {
          let ts = parse_str_utc(ms.last_updated_timestamp.clone()).unwrap();
          mdls.push((ms.label.clone(),ts));
        }
        return Some(mdls);
      }

      fn fgs(&self) -> Option<Vec<(String,chrono::DateTime<Utc>)>> {
        let mut grs = Vec::new();
        for g in &self.fgs {
          let ts = parse_str_utc(g.last_updated_timestamp.clone()).unwrap();
          grs.push((g.label.clone(),ts));
        }
        return Some(grs);
      }
    }
  }
}