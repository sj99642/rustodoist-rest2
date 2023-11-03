use serde::de::DeserializeOwned;
use crate::err::TodoistAPIError;

/// Takes a Result<Response, Error> from a Reqwest request and (hopefully) returns the deserialised JSON payload
///
/// This does three steps:
/// 1. Make sure that the Result was Ok; if it was Err then return a Todoist error with that payload
/// 2. Make sure that the HTTP response was successful; if it wasn't then return a Todoist error with the bad status code
/// 3. Try to deserialise the JSON payload; if it fails then return a Todoist error with the deserialisation error
///
/// If that was all successful, return the payload
pub fn get_from_reqwest_response<T: DeserializeOwned>(response: Result<reqwest::blocking::Response, reqwest::Error>)
                                -> Result<T, TodoistAPIError> {
    // Unpack the meaning of the response, making sure there was no error in making the HTTP request
    let response = match response {
        Ok(rsp) => rsp,
        Err(err) => return Err(TodoistAPIError::ReqwestRequestError(err)),
    };

    // We got an HTTP response; was it successful?
    if !response.status().is_success() {
        return Err(TodoistAPIError::UnsuccessfulHTTPStatus(response.status()));
    }

    // We got a successful HTTP response. That means it *should* be proper JSON we can deserialise
    let parsed = response.json::<T>();
    match parsed {
        Ok(data) => Ok(data),
        Err(err) => Err(TodoistAPIError::ReqwestDeserialisationError(err))
    }
}