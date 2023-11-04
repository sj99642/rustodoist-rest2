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
    /// Holds a copy of the Error returned by reqwest.
    ReqwestRequestError(reqwest::Error),

    /// A valid HTTP response was received, but the status code indicates an unsuccessful API
    /// request.
    ///
    /// Holds the problematic status code, as well as the text of the response from the API (which
    /// typically will explain the issue).
    UnsuccessfulHTTPStatus(reqwest::StatusCode, String),

    /// An API response was received which should have contained a valid JSON response, but
    /// this could not be deserialised to the type of object expected.
    ReqwestDeserialisationError(reqwest::Error),

    /// Failed to serialise a structure into JSON form.
    SerdeSerialisationError(serde_json::Error),
}
