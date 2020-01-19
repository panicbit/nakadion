use std::fmt;

use serde::{Deserialize, Serialize};

/// Indicator of the application owning this EventType.
///
/// See also [Nakadi Manual](https://nakadi.io/manual.html#definition_EventType*owning_application)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwningApplication(String);

impl OwningApplication {
    pub fn new(v: impl Into<String>) -> Self {
        OwningApplication(v.into())
    }
}

/// An attribute for authorization.
///
/// This object includes a data type, which represents the type of the
/// attribute attribute (which data types are allowed depends on which authorization
/// plugin is deployed, and how it is configured), and a value.
/// A wildcard can be represented with data type and value. It means that
/// all authenticated users are allowed to perform an operation.
///
/// See also [Nakadi Manual](https://nakadi.io/manual.html#definition_AuthorizationAttribute)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationAttribute {
    /// The type of attribute (e.g., ‘team’, or ‘permission’, depending on the Nakadi configuration)
    pub data_type: AuthAttDataType,
    /// The value of the attribute
    pub value: AuthAttValue,
}

/// Data type of `AuthorizationAttribute`
///
/// See also [Nakadi Manual](https://nakadi.io/manual.html#definition_AuthorizationAttribute)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthAttDataType(pub String);

/// Value of `AuthorizationAttribute`
///
/// See also [Nakadi Manual](https://nakadi.io/manual.html#definition_AuthorizationAttribute)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthAttValue(pub String);