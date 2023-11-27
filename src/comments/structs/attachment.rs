use serde::{Deserialize};

/// Defines an attachment to a comment. This can be returned from the API,
/// and can also be used in the creation of a comment, although you can't
/// actually upload new attachments to Todoist's servers using the REST API.
#[derive(Debug, Deserialize)]
pub struct Attachment {
    file_name: String,
    file_size: u64,
    file_type: String,
    file_url: String,
    upload_state: Option<UploadState>,

    #[serde(flatten)]
    #[serde(default)]
    attachment_type: AttachmentType,
}


#[derive(Debug, Deserialize)]
pub enum UploadState {
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "completed")]
    Completed
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AttachmentType {
    Basic(),
    Image {
        tn_l: Thumbnail,
        tn_m: Thumbnail,
        tn_s: Thumbnail,
    },
    Audio {
        file_duration: i32
    }
}

// If no special attachment information is specified, then default to Basic
// This means the attachment type isn't an Option
impl Default for AttachmentType {
    fn default() -> Self {
        AttachmentType::Basic()
    }
}

pub type Thumbnail = (String, u32, u32);
