
use uuid::Uuid;
use std::marker;
use crate::Utc;


#[derive(Debug, Clone)]
pub struct Org {
  id: Uuid,
  label: String,
  description: String,
}

#[derive(Debug, Clone)]
pub struct Model {}

#[derive(Debug, Clone)]
pub struct Factorgraph {}

#[derive(Clone,Debug)]
pub struct NvaNode<T> {
  pub namespace: Uuid,
  pub label: String,
  pub _marker: marker::PhantomData<T>
  // https://doc.rust-lang.org/nomicon/phantom-data.html
}


impl<T> NvaNode<T> {
  pub fn new(
    namespace: Uuid, label: String
  ) -> Self {
    return Self {
      namespace,
      label,
      _marker: marker::PhantomData
    };
  }
}


impl Default for NvaNode<Factorgraph> {
  fn default() -> Self {
    return Self {
      namespace: Uuid::nil(),
      label: String::new(),
      _marker: marker::PhantomData
    };
  }
}

pub trait GraphFieldImportersSkeleton {
  fn id(&self) -> Option<Uuid>;
  fn label(&self) -> String;
  fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>>;
  fn namespace(&self) -> Uuid;
}


// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! Graph_importers_skeleton { 
  ($T:ident) => {
    impl GraphFieldImportersSkeleton for $T {
      fn id(&self) -> Option<Uuid> { Some(Uuid::parse_str(&self.id).expect("failed to parse factorgraph id to uuid")) }
      
      fn label(&self) -> String { self.label.to_string() }
      
      fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>> {
        let timestamp = &self.last_updated_timestamp;
        match parse_str_utc(timestamp.clone()) {
          Ok(tms) => { 
            return Some(tms);
          },
          Err(e) => {
            let errm = format!("GraphImporterSkeleton, lastUpdatedTimestamp failed chrono parse_from_str timestamp {:?} with error {:?}",timestamp,e);
            to_console_error(&errm);
            return None;
          }
        }
      }

      fn namespace(&self) -> Uuid { 
        return Uuid::parse_str(
          &self.namespace
          .clone().unwrap()
        ).expect("failed to parse factorgraph namespace to uuid");
      }
    }
  }
}
