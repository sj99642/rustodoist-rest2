use reqwest;
use serde_json;

#[derive(Debug)]
pub enum TodoistAPIError {
    ReqwestRequestError(reqwest::Error),
    UnsuccessfulHTTPStatus(reqwest::StatusCode, String),
    ReqwestDeserialisationError(reqwest::Error),
    SerdeSerialisationError(serde_json::Error),
}
