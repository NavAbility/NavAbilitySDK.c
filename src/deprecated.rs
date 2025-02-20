
use crate::{
    Uuid,
    Utc,
    BlobEntry,
};

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
use crate::{
    Sender,
    Error,
    send_api_result,
    to_console_debug,
    to_console_error,
    parse_str_utc,
    get_blob_entry,
    get_variable,
};


// ================== DEPRECATED ===================

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
#[deprecated(since="0.1.0", note="please use send_api_result(send_into, `response_body=Ok(data)`) instead")]
pub fn send_api_response<T>(
    send_into: Sender<T>,
    data: T,
) -> Result<(),Box<dyn Error>> {
    return send_api_result(send_into,Ok(data));
}


pub trait SameBlobEntryFields {
    fn to_gql_blobentry(self) -> get_blob_entry::blobEntry_fields;
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl BlobEntry {
    pub fn same_gql(
        sgql: impl SameBlobEntryFields,
    ) -> get_blob_entry::blobEntry_fields {
        return sgql.to_gql_blobentry();
    }

    // DEPRECATING
    pub fn from_gql2(
        gety: &get_blob_entry::blobEntry_fields
    ) -> Self {
        let mut be = BlobEntry::default();
        be.id = Some(Uuid::parse_str(&gety.id).expect("failed to parse entry id to uuid"));
        be.blobId = Uuid::parse_str(&gety.blob_id).expect("failed to parse entry blob_id to uuid");
        be.label = gety.label.to_string();
        if let Some(blobstore) = &gety.blobstore {
            be.blobstore = blobstore.to_string();
        }
        if let Some(origin) = &gety.origin {
            be.origin = origin.to_string();
        }
        if let Some(mime) = &gety.mime_type {
            be.mimeType = mime.to_string();
        }
        if let Some(description) = &gety.description {
            be.description = description.to_string();
        }
        if let Some(hash) = &gety.hash {
            be.hash = hash.to_string();
        }
        if let Some(metadata) = &gety.metadata {
            be.metadata = metadata.to_string();
        }
        if let Some(size) = &gety.size {
            be.size = Some((*size).parse::<i64>().unwrap());
        }
        if let Some(timestamp) = &gety.timestamp {
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                be.timestamp = tms;
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        {
            let timestamp = &gety.created_timestamp;
            // to_console_debug(&format!("BlobEntry from rx timestamp string {}",&timestamp));
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(timestamp.clone()) {
                be.createdTimestamp = Some(tms);
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        {
            let timestamp = &gety.last_updated_timestamp;
            // to_console_debug(&format!("BlobEntry from rx timestamp string {}",&timestamp));
            // 2024-09-16T16:51:20.555Z
            if let Ok(tms) = parse_str_utc(
                timestamp.clone()
            ) {
                be.lastUpdatedTimestamp = Some(tms);
            } else {
                to_console_error(&format!("BlobEntry, failed chrono parse_from_str timestamp {:?}",timestamp));
            }
        }
        if let Some(_type) = &gety.type_ {
            be._type = _type.to_string();
        }
        be._version = gety.version.to_string();

        return be;
    }
}


#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl SameBlobEntryFields for get_blob_entry::blobEntry_fields {
    fn to_gql_blobentry(self) -> get_blob_entry::blobEntry_fields {
        return self;
    }
}

#[cfg(any(feature = "tokio", feature = "wasm", feature = "blocking"))]
impl SameBlobEntryFields for get_variable::blobEntry_fields {
    fn to_gql_blobentry(
        self
    ) -> get_blob_entry::blobEntry_fields {
        return get_blob_entry::blobEntry_fields {
            id: self.id.clone(),
            blob_id: self.blob_id.clone(),
            origin_id: self.origin_id.clone(),
            label: self.label.clone(),
            blobstore: self.blobstore.clone(),
            hash: self.hash.clone(),
            origin: self.origin.clone(),
            size: self.size.clone(),
            description: self.description.clone(),
            mime_type: self.mime_type.clone(),
            metadata: self.metadata.clone(),
            timestamp: self.timestamp.clone(),
            created_timestamp: self.created_timestamp.clone(),
            last_updated_timestamp: self.last_updated_timestamp.clone(),
            version: self.version.clone(),
            type_: self.type_.clone(),
        }
    }
}

// unclear if manual definition of user robot session is necessary
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub label: String,
    pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}



// TBD: TO BE DEPRECATED? with SDK.jl v0.8
#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub label: String,
    pub robot_label: String,
    pub user_label: String,
    pub _version: String,
    pub created_timestamp: chrono::DateTime::<Utc>,
    pub last_updated_timestamp: chrono::DateTime::<Utc>,
}
