#![macro_use]

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use serde::{de::Error as SerdeError, Deserialize, Deserializer};

macro_rules! try_env {
    ($ENV_VAR_NAME:expr) => {
        match std::env::var($ENV_VAR_NAME) {
            Ok(value) => Ok(value),
            Err(std::env::VarError::NotPresent) => None,
            Err(std::env::VarError::NotUnicode) => Err($crate::helpers::MessageError::new(
                format!("env var '{}' is not unicode", $ENV_VAR_NAME),
            )
            .boxed()),
        }
    };
}

macro_rules! must_env {
    ($ENV_VAR_NAME:expr) => {
        match std::env::var($ENV_VAR_NAME) {
            Ok(value) => Ok(value),
            Err(std::env::VarError::NotPresent) => Err($crate::helpers::MessageError::new(
                format!("env var '{}' not found", $ENV_VAR_NAME),
            )
            .boxed()),
            Err(std::env::VarError::NotUnicode(_)) => Err($crate::helpers::MessageError::new(
                format!("env var '{}' is not unicode", $ENV_VAR_NAME),
            )
            .boxed()),
        }
    };
}

macro_rules! must_env_parsed {
    ($ENV_VAR_NAME:expr) => {
        match std::env::var($ENV_VAR_NAME) {
            Ok(value) => value.parse().map_err(|err| {
                $crate::helpers::MessageError::new(format!(
                    "could not parse env var '{}': {}",
                    $ENV_VAR_NAME, err
                ))
                .boxed()
            }),
            Err(std::env::VarError::NotPresent) => Err($crate::helpers::MessageError::new(
                format!("env var '{}' not found", $ENV_VAR_NAME),
            )
            .boxed()),
            Err(std::env::VarError::NotUnicode(_)) => Err($crate::helpers::MessageError::new(
                format!("env var '{}' is not unicode", $ENV_VAR_NAME),
            )
            .boxed()),
        }
    };
}

#[derive(Debug)]
pub struct MessageError(pub String);

impl MessageError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }

    pub fn boxed(self) -> Box<dyn Error> {
        Box::new(self)
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;

        Ok(())
    }
}

impl Error for MessageError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl<T> From<T> for MessageError
where
    T: Into<String>,
{
    fn from(msg: T) -> Self {
        Self::new(msg)
    }
}

pub fn deserialize_empty_string_is_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let parsed = s
            .parse::<T>()
            .map_err(|err| SerdeError::custom(err.to_string()))?;
        Ok(Some(parsed))
    }
}