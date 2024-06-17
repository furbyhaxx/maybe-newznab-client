use serde::{Deserialize, Deserializer};
use thiserror::Error;

/// A raw Error that is returned from a Newznab API calls
/// This is just a step between to simplify the Deserialization
#[derive(Debug, Error, Deserialize)]
#[error("{code}: {description}")]
pub struct NewznabRawError {
    search_type: Option<String>,
    #[serde(deserialize_with = "u16_from_string")]
    code: u16,
    description: String,
}

/// An Error that is returned from a Newznab API call
/// 
#[derive(Debug, Error)]
pub enum NewznabError {

    /// 100
    #[error("{code}: {message}")]
    IncorrectUserCredentials { code: u16, message: String, inner: NewznabRawError } ,

    /// 101
    #[error("{code}: {message}")]
    AccountSuspended { code: u16, message: String, inner: NewznabRawError },

    /// 102
    #[error("{code}: {message}")]
    InsufficientPrivileges { code: u16, message: String, inner: NewznabRawError },

    /// 103
    #[error("{code}: {message}")]
    RegistrationDenied { code: u16, message: String, inner: NewznabRawError },

    /// 104
    #[error("{code}: {message}")]
    RegistrationsClosed { code: u16, message: String, inner: NewznabRawError },

    /// 105
    #[error("{code}: {message}")]
    RegistrationFailedEmailTaken { code: u16, message: String, inner: NewznabRawError },

    /// 106
    #[error("{code}: {message}")]
    RegistrationFailedEmailBadFormat { code: u16, message: String, inner: NewznabRawError },

    /// 107
    #[error("{code}: {message}")]
    RegistrationFailedDataError { code: u16, message: String, inner: NewznabRawError },

    /// 108..=199
    #[error("{code}: {message}")]
    UnknownAccountError { code: u16, message: String, inner: NewznabRawError }, // 100-199

    /// 200
    #[error("{code}: {message}")]
    MissingParameter { code: u16, message: String, inner: NewznabRawError },

    /// 201
    #[error("{code}: {message}")]
    IncorrectParameter { code: u16, message: String, inner: NewznabRawError },

    /// 202
    #[error("{code}: {message}")]
    NoSuchFunction { code: u16, message: String, inner: NewznabRawError },

    /// 203
    #[error("{code}: {message}")]
    FunctionNotAvailable { code: u16, message: String, inner: NewznabRawError },

    /// 204..=299
    #[error("{code}: {message}")]
    UnknownApiCallError { code: u16, message: String, inner: NewznabRawError }, //200-299

    /// 300
    #[error("{code}: {message}")]
    NoSuchItem { code: u16, message: String, inner: NewznabRawError },

    /// 301
    #[error("{code}: {message}")]
    ItemAlreadyExists { code: u16, message: String, inner: NewznabRawError },

    /// 302..=399
    #[error("{code}: {message}")]
    UnknownContentError { code: u16, message: String, inner: NewznabRawError }, //300-399

    /// 900
    #[error("{code}: {message}")]
    UnknownError { code: u16, message: String, inner: NewznabRawError },

    /// 901
    #[error("{code}: {message}")]
    ApiDisabled { code: u16, message: String, inner: NewznabRawError },

    /// 902..=999
    #[error("{code}: {message}")]
    UnknownOtherError { code: u16, message: String, inner: NewznabRawError }, // 900-999
}

impl From<NewznabRawError> for NewznabError {
    fn from(raw: NewznabRawError) -> Self {
        match raw.code {
            100 => NewznabError::IncorrectUserCredentials {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            101 => NewznabError::AccountSuspended {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            102 => NewznabError::InsufficientPrivileges {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            103 => NewznabError::RegistrationDenied {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            104 => NewznabError::RegistrationsClosed {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            105 => NewznabError::RegistrationFailedEmailTaken {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            106 => NewznabError::RegistrationFailedEmailBadFormat {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            107 => NewznabError::RegistrationFailedDataError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            108..=199 => NewznabError::UnknownAccountError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},

            200 => NewznabError::MissingParameter {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            201 => NewznabError::IncorrectParameter {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            202 => NewznabError::NoSuchFunction {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            203 => NewznabError::FunctionNotAvailable {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            204..=299 => NewznabError::UnknownApiCallError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},

            300 => NewznabError::NoSuchItem {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            301 => NewznabError::ItemAlreadyExists {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            302..=399 => NewznabError::UnknownContentError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},

            900 => NewznabError::UnknownError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            901 => NewznabError::ApiDisabled {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
            902..=999 | _ => NewznabError::UnknownOtherError {code: raw.code.clone(), message: raw.description.clone(), inner: raw},
        }
    }
}

/// Parser function for serde::Deserialize
/// Parses an u16 integer from a String
fn u16_from_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    String::deserialize(deserializer)
        .and_then(|string| string.parse::<u16>().map_err(|err| Error::custom(err.to_string())))
}