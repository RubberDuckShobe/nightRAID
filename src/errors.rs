/*
    Credit to:
    - https://github.com/m1guelpf/dyson/blob/main/api/src/errors.rs
    - https://github.com/JosephLenton/axum-route-error
    - https://www.reddit.com/r/rust/comments/ozc0m8/an_actixanyhow_compatible_error_helper_i_found/
*/

use std::fmt::Debug;

use anyhow::Error as AnyhowError;
use serde::{Deserialize, Serialize};

pub struct TextError<S = (), const EXPOSE_INTERNAL_ERROR: bool = false>
where
    S: Serialize + for<'a> Deserialize<'a> + Debug,
{
    error: Option<AnyhowError>,
    extra_data: Option<Box<S>>,
    public_error_message: Option<String>,
}

impl<S, const EXPOSE_INTERNAL_ERROR: bool> TextError<S, EXPOSE_INTERNAL_ERROR>
where
    S: Serialize + for<'a> Deserialize<'a> + Debug,
{
    /// Set an internal error.
    ///
    /// This is used for tracking the source of the error internally.
    pub fn set_error(self, error: AnyhowError) -> Self {
        Self {
            error: Some(error),
            ..self
        }
    }

    pub fn set_error_data<NewS>(self, extra_data: NewS) -> TextError<NewS>
    where
        NewS: Serialize + for<'a> Deserialize<'a> + Debug,
    {
        TextError {
            extra_data: Some(Box::new(extra_data)),
            error: self.error,
            public_error_message: self.public_error_message,
        }
    }

    /// Set the error message to display within the error.
    pub fn set_public_error_message(self, public_error_message: &str) -> Self {
        Self {
            public_error_message: Some(public_error_message.to_string()),
            ..self
        }
    }

    /// Returns the error message that will be shown to the end user.
    pub fn public_error_message(&self) -> &str {
        if let Some(public_error_message) = self.public_error_message.as_ref() {
            return public_error_message;
        }

        "AN INTERNAL ERROR OCCURRED! PLEASE CONTACT m1nt_"
    }
}

pub trait IntoTextError<T> {
    fn text_error(self, message: &str) -> core::result::Result<T, TextError>;
}

impl<S, const EXPOSE_INTERNAL_ERROR: bool> Default for TextError<S, EXPOSE_INTERNAL_ERROR>
where
    S: Serialize + for<'a> Deserialize<'a> + Debug,
{
    fn default() -> Self {
        Self {
            error: None,
            extra_data: None,
            public_error_message: None,
        }
    }
}

/// This essentially means if you can turn it into an Anyhow,
/// then you can turn it into a `RouteError`.
impl<S, const EXPOSE_INTERNAL_ERROR: bool, FE> From<FE> for TextError<S, EXPOSE_INTERNAL_ERROR>
where
    S: Serialize + for<'a> Deserialize<'a> + Debug,
    FE: Into<AnyhowError>,
{
    fn from(error: FE) -> Self {
        let anyhow_error: AnyhowError = error.into();
        Self {
            error: Some(anyhow_error),
            ..Self::default()
        }
    }
}

impl<T: std::fmt::Debug, E: Into<AnyhowError> + std::fmt::Debug> IntoTextError<T> for Result<T, E> {
    fn text_error(self, message: &str) -> core::result::Result<T, TextError> {
        self.map_err(|err| TextError::from(err).set_public_error_message(message))
    }
}

impl ToString for TextError {
    fn to_string(&self) -> String {
        "error: ".to_owned() + self.public_error_message() + "\n\n"
    }
}
