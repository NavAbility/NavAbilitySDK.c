

use crate::{
    get_blob_entry, Utc, Uuid, parse_str_utc,
};
use chrono::{
    ParseError,
};


/// A `BlobEntry` is a small amount of structured data that holds contextual/reference information to find an actual blob. 
/// A `BlobEntry` does not have to point to a particular blobId, e.g. storing metadata or providing topological context.
/// Many `BlobEntry`s can exist on different graph nodes spanning Robots, and Sessions which can all reference the same `Blob`.  
/// A `BlobEntry` is also a equivalent to a bridging entry between local `.originId` and a remotely assigned `.blobIds`.
///
/// Notes:
/// - `blobId`s should be unique within a blobstore and are immutable; or 
///   - if blobless, should have UUID("00000000-0000-0000-000000000000").
#[derive(Debug, Default, Clone)]
#[allow(non_snake_case)]
pub struct BlobEntry {
    /// Remotely assigned and globally unique identifier for the `BlobEntry` itself (not the `.blobId`).
    pub id: Option<Uuid>,
    /// Machine friendly and globally unique identifier of the 'Blob', usually assigned from a common point in the system.  This can be used to guarantee unique retrieval of the large data blob.
    pub blobId: Uuid,
    /// Machine friendly and locally assigned identifier of the 'Blob'.  `.originId`s are mandatory upon first creation at the origin regardless of network access.  Separate from `.blobId` since some architectures do not allow edge processes to assign a uuid4 to data store elements.
    pub originId: Option<Uuid>,
    /// Human friendly label of the `Blob` and also used as unique identifier per node on which a `BlobEntry` is added.  E.g. do "LEFTCAM_1", "LEFTCAM_2", ... of you need to repeat a label on the same variable.
    pub label: String,
    /// A hint about where the `Blob` itself might be stored.  Remember that a Blob may be duplicated over multiple blobstores.
    pub blobstore: String,
    /// A hash value to ensure data consistency which must correspond to the stored hash upon retrieval.  Use `bytes2hex(sha256(blob))`. [Legacy: some usage functions allow the check to be skipped if needed.]
    pub hash: String,
    /// Context from which a BlobEntry=>Blob was first created. E.g. user|robot|session|varlabel.
    pub origin: String,
    /// number of bytes in blob
    pub size: Option<i64>,
    /// Additional information that can help a different user of the Blob.
    pub description: String,
    /// MIME description describing the format of binary data in the `Blob`, e.g. 'image/png' or 'application/json; _type=CameraModel'.
    pub mimeType: String,
    /// Additional storage for functional metadata used in some scenarios, e.g. to support advanced features such as `parsejson(base64decode(entry.metadata))['time_sync']`.
    pub metadata: String,
    /// When the Blob itself was first created.
    pub timestamp: chrono::DateTime<Utc>,
    /// When the BlobEntry was created.
    pub createdTimestamp: Option<chrono::DateTime<Utc>>,
    /// Use carefully, but necessary to support advanced usage such as time synchronization over Blob data.
    pub lastUpdatedTimestamp: Option<chrono::DateTime<Utc>>,
    /// Self type declaration for when duck-typing happens.
    pub _type: String,
    /// Type version of this BlobEntry. Consider upgrading to `::VersionNumber`.
    pub _version: String,
}


pub trait BlobEntrySummaryImporters {
    fn id(&self) -> Option<Uuid>;
    fn label(&self) -> String;
    fn size(&self) -> Option<i64>;
    fn mimeType(&self) -> String;
    fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>>;
}


// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! BlobEntry_importers_summary { 
    ($T:ident) => {
        impl BlobEntrySummaryImporters for $T {
            fn id(&self) -> Option<Uuid> { Some(Uuid::parse_str(&self.id).expect("failed to parse blobentry id to uuid")) }

            fn label(&self) -> String { self.label.to_string() }

            fn size(&self) -> Option<i64> {
                if let Some(sz) = &self.size {
                    return Some(sz.parse::<i64>().unwrap());
                } else {
                    return None;
                }
            }

            fn mimeType(&self) -> String { 
                return self.mime_type.clone().unwrap_or("".to_owned()).to_string();
            }

            fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>> {
                let timestamp = &self.last_updated_timestamp;
                // 2024-09-16T16:51:20.555Z
                if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                    return Some(tms);
                } else {
                    let errm = format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp);
                    to_console_error(&errm);
                }
                return None;
            }
        }
    }
}





pub trait BlobEntryFieldsImporters {
    fn id(&self) -> Option<Uuid>;
    fn blobId(&self) -> Uuid;
    fn originId(&self) -> Option<Uuid>;
    fn label(&self) -> String;
    fn blobstore(&self) -> String;
    fn hash(&self) -> String;
    fn origin(&self) -> String;
    fn size(&self) -> Option<i64>;
    fn description(&self) -> String;
    fn mimeType(&self) -> String;
    fn metadata(&self) -> String;
    fn timestamp(&self) -> Result<chrono::DateTime<Utc>, ParseError>;
    fn createdTimestamp(&self) -> Option<chrono::DateTime<Utc>>;
    fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>>;
    fn _type(&self) -> String;
    fn _version(&self) -> String;
}

// helper macro to avoid repetition of "basic" impl Coordinates
#[macro_export]
macro_rules! BlobEntry_importers { 
    ($T:ident) => {
        impl BlobEntryFieldsImporters for $T {
            fn id(&self) -> Option<Uuid> { Some(Uuid::parse_str(&self.id).expect("failed to parse blobentry id to uuid")) }
            
            fn blobId(&self) -> Uuid { Uuid::parse_str(&self.blob_id).expect("failed to parse blobentry blob_id to uuid") }
            
            fn originId(&self) -> Option<Uuid> {
                if self.origin_id.is_some() {
                    return Some(Uuid::parse_str(&self.origin_id.clone().unwrap()).expect("failed to parse blobentry origin_id to uuid"));
                } else {
                    return None;
                }
            }
            
            fn label(&self) -> String { self.label.to_string() }
            
            fn blobstore(&self) -> String {
                if let Some(blobstore) = &self.blobstore {
                    return blobstore.to_string();
                } else {
                    return "default".to_string();
                }
            }
            
            fn hash(&self) -> String { 
                self.hash.clone().unwrap_or("".to_owned()).to_string() 
            }

            fn origin(&self) -> String { 
                self.origin.clone().unwrap_or("".to_owned()).to_string() 
            }

            fn size(&self) -> Option<i64> {
                if let Some(sz) = &self.size {
                    return Some(sz.parse::<i64>().unwrap());
                } else {
                    return None;
                }
            }

            fn description(&self) -> String { 
                return self.description.clone().unwrap_or("".to_owned()).to_string();
            }

            fn mimeType(&self) -> String { 
                return self.mime_type.clone().unwrap_or("".to_owned()).to_string();
            }

            fn metadata(&self) -> String { 
                return self.metadata.clone().unwrap_or("".to_owned()).to_string();
            }

            fn timestamp(&self) -> Result<chrono::DateTime<Utc>, ParseError> {
                return parse_str_utc(self.timestamp.clone().unwrap_or("".to_owned()));
            }

            fn createdTimestamp(&self) -> Option<chrono::DateTime<Utc>> {
                let timestamp = &self.created_timestamp;
                // 2024-09-16T16:51:20.555Z
                if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                    return Some(tms);
                } else {
                    let errm = format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp);
                    to_console_error(&errm);
                }
                return None;
            }

            fn lastUpdatedTimestamp(&self) -> Option<chrono::DateTime<Utc>> {
                let timestamp = &self.last_updated_timestamp;
                // 2024-09-16T16:51:20.555Z
                if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                    return Some(tms);
                } else {
                    let errm = format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp);
                    to_console_error(&errm);
                }
                return None;
            }

            fn _type(&self) -> String { 
                return self.type_.clone().unwrap_or("".to_owned()).to_string();
            }

            fn _version(&self) -> String { 
                return self.version.to_string();
            }
        }
    }
}



