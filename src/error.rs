
extern crate reqwest;
extern crate serde_json;
extern crate url;

use std;

pub type BufferFillResult<T> = Result<T, BufferFillError>;

#[derive(Debug)]
pub enum BufferFillError {
    InsufficientBuffer,
    NullPointerError,
    ZeroByteInString,
}

impl From<std::ffi::NulError> for BufferFillError {
    fn from(_: std::ffi::NulError) -> BufferFillError {
        BufferFillError::ZeroByteInString
    }
}

pub type GraphInfoResult<T> = Result<T, GraphInfoRetrievalError>;

#[derive(Debug)]
pub enum GraphInfoRetrievalError {
    NoAccessToken { response: String },
    BadHTTPResponse {
        status: reqwest::StatusCode,
        data: String,
    },
    BadJSONResponse,
    HTTPError(reqwest::Error),
    IOError,
    UnusableImmutableID,
    TooManyResults,
    NotFound,
}

impl From<serde_json::Error> for GraphInfoRetrievalError {
    fn from(_: serde_json::Error) -> GraphInfoRetrievalError {
        GraphInfoRetrievalError::BadJSONResponse
    }
}

impl From<reqwest::Error> for GraphInfoRetrievalError {
    fn from(err: reqwest::Error) -> GraphInfoRetrievalError {
        GraphInfoRetrievalError::HTTPError(err)
    }
}

impl From<std::io::Error> for GraphInfoRetrievalError {
    fn from(_: std::io::Error) -> GraphInfoRetrievalError {
        GraphInfoRetrievalError::IOError
    }
}

impl From<std::num::ParseIntError> for GraphInfoRetrievalError {
    fn from(_: std::num::ParseIntError) -> GraphInfoRetrievalError {
        GraphInfoRetrievalError::UnusableImmutableID
    }
}
