use reqwest;

#[derive(Debug)]
pub enum TodoistAPIError {
    ReqwestRequestError(reqwest::Error),
    UnsuccessfulHTTPStatus(reqwest::StatusCode),
    ReqwestDeserialisationError(reqwest::Error),
}
