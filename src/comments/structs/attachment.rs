use serde::{Deserialize, Serialize};

/// Defines an attachment to a comment. This can be returned from the API,
/// and can also be used in the creation of a comment, although you can't
/// actually upload new attachments to Todoist's servers using the REST API.
pub struct Attachment {
    file_name: String,
    file_size: u64,
    file_type: String,
    file_url: String,
    upload_state: UploadState,
    attachment_type: Option<AttachmentType>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum UploadState {
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "completed")]
    Completed
}


// TODO Implement this properly
#[derive(Debug, Serialize, Deserialize)]
pub enum AttachmentType {
    Basic(),
    Image {
        tn_l: Thumbnail,
        tn_m: Thumbnail,
        tn_s: Thumbnail,
    },
    Audio
}

pub type Thumbnail = (String, u32, u32);
