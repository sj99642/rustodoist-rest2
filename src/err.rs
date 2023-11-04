use reqwest;
use serde_json;

/// Holds the different kinds of things that can go wrong when making API requests.
/// Functions in this crate will typically return a Result with this as the error type,
/// so that it's possible for the caller to know what stage a problem happened at.
#[derive(Debug)]
pub enum TodoistAPIError {
    /// Indicates that the reqwest module either failed to send an HTTP request, or
    /// no HTTP response was returned.
    ///
    /// This could be caused by various things:
    /// - No internet connection
    /// - Todoist servers are down
    /// - Todoist servers are up and responding, but the response was lost in transit
    /// - An internal error in the crate causing a malformed request
    ///
    /// Holds a copy of the Error returned by reqwest.
    ReqwestRequestError(reqwest::Error),

    /// A valid HTTP response was received, but the status code indicates an unsuccessful API
    /// request.
    ///
    /// - If the status code is 4xx, then the request was invalid in some way (e.g. missing/bad
    ///   authentication). It could also indicate an internal crate error, e.g. putting a bad
    ///   header or value into the request.
    /// - If the status code is 5xx, it's on Todoist's side and indicates a server issue.
    ///
    /// Holds the problematic status code, as well as the text of the response from the API (which
    /// typically will explain the issue).
    UnsuccessfulHTTPStatus(reqwest::StatusCode, String),

    /// An API response was received which should have contained a valid JSON response, but
    /// this could not be deserialised to the type of object expected. This could either be because
    /// Todoist is sending back malformed JSON, or (more likely) because the struct type the crate
    /// is trying to deserialise the JSON into is defined wrong (e.g. fails to account for a field
    /// being able to be `null`).
    ReqwestDeserialisationError(reqwest::Error),

    /// Failed to serialise a structure into JSON form. This most likely is caused by an error in
    /// the crate.
    SerdeSerialisationError(serde_json::Error),
}
