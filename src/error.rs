use std::string::FromUtf8Error;

use crate::Element;
use quick_xml::Error as XMLError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RootCannotMove,
    NotFound,
    IsAnElement,
    ElementNotExist(Element),
    MalformedXML(String),
    NotEmpty,
    HasAParent,
    LazyError(quick_xml::Error),
}

impl From<XMLError> for Error {
    fn from(err: XMLError) -> Error {
        match err {
            XMLError::EndEventMismatch { expected, found } => Error::MalformedXML(format!(
                "Closing tag mismatch. Expected {}, found {}",
                expected, found,
            )),
            _ => Error::LazyError(err),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::MalformedXML("Not a valid utf-8".to_string())
    }
}
