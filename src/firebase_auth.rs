//! Structs and functions essential and initializing Firebase Auth

#[cfg(feature = "env")]
use crate::errors::{AuthError, Env};

#[cfg(feature = "env")]
use dotenvy;
#[cfg(feature = "env")]
use serde_json;

use serde::Deserialize;
#[cfg(feature = "env")]
use std::{env, fs::read_to_string};

/// Endpoint to fetch JWKs when verifying firebase tokens
pub static JWKS_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

/// A partial representation of firebase admin object provided by firebase.
///
/// The fields in the firebase admin object is necessary when encoding and
/// decoding tokens. All fields should be kept secret.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub(crate) project_id:     String,
    pub(crate) private_key_id: String,
    pub(crate) private_key:    String,
    pub(crate) client_email:   String,
    pub(crate) client_id:      String,
}

/// Firebase Auth instance
///
///
#[derive(Debug, Deserialize)]
pub struct FirebaseAuth {
    pub(crate) credentials: Credentials,
}

impl Default for FirebaseAuth {
    fn default() -> Self {
        Self {
            credentials: Credentials {
                project_id:     String::default(),
                private_key_id: String::default(),
                private_key:    String::default(),
                client_email:   String::default(),
                client_id:      String::default(),
            },
        }
    }
}

#[cfg(feature = "env")]
impl TryFrom<String> for FirebaseAuth {
    type Error = AuthError;

    fn try_from(credentials: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<Credentials>(&credentials)
            .map(|deserialized_credentials| {
                FirebaseAuth::new(deserialized_credentials)
            })
            .map_err(|e| {
                AuthError::Env(Env::InvalidFirebaseCredentials(e.to_string()))
            })
    }
}

impl FirebaseAuth {
    /// Create a new FirebaseAuth struct by providing Credentials
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }

    /// Create a new FirebaseAuth struct from a dotenv file
    #[cfg(feature = "env")]
    pub fn try_from_env(variable_name: &str) -> Result<Self, AuthError> {
        Self::try_from_filename(".env", variable_name)
    }

    /// Create a new FirebaseAuth struct by providing a dotenv filepath
    ///
    /// This function is will most likely find its way in the codebase when
    /// supplying the `FirebaseAuth` dummy values in tests.
    #[cfg(feature = "env")]
    pub fn try_from_filename(
        filepath: &str,
        variable_name: &str,
    ) -> Result<Self, AuthError> {
        dotenvy::from_filename(filepath).ok();

        env::var(variable_name)
            .map_err(|e| {
                AuthError::Env(Env::InvalidFirebaseCredentials(e.to_string()))
            })
            .and_then(|credentials| credentials.try_into())
    }

    /// Create a new FirebaseAuth struct from a file with the credentials given
    /// by Firebase, but not in a `.env` file.
    #[cfg(feature = "env")]
    pub fn try_from_json_file(filepath: &str) -> Result<Self, AuthError> {
        // given file must be a `.json` file
        if !filepath.ends_with(".json") {
            Err(AuthError::Env(Env::InvalidFileFormat(format!(
                "Expected .json file. Received {filepath}"
            ))))
        } else {
            read_to_string(filepath)
                .map_err(|e| {
                    AuthError::Env(Env::InvalidFirebaseCredentials(
                        e.to_string(),
                    ))
                })
                .and_then(|credentials| credentials.try_into())
        }
    }
}
