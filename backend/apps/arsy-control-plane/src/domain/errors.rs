//! Domain-level failures shared by use cases and provider adapters.
//!
//! HTTP handlers and queue workers translate these errors at their boundaries;
//! domain code does not depend on Axum or a provider SDK.

use std::{error::Error, fmt};

use crate::domain::ids::ResourceId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainError {
    NotFound {
        resource: &'static str,
        id: String,
    },
    InvalidInput {
        field: &'static str,
        reason: &'static str,
    },
    Unauthorized,
    Forbidden,
    Conflict {
        resource: &'static str,
        reason: &'static str,
    },
    IdempotencyConflict,
    UnsupportedCapability {
        provider: String,
        capability: &'static str,
    },
    ProviderUnavailable {
        provider: String,
        retryable: bool,
    },
    OperationNotRecoverable {
        operation_id: String,
    },
}

impl DomainError {
    #[must_use]
    pub fn resource_not_found(resource: &'static str, id: ResourceId) -> Self {
        Self::NotFound {
            resource,
            id: id.to_string(),
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { resource, id } => write!(formatter, "{resource} '{id}' was not found"),
            Self::InvalidInput { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }
            Self::Unauthorized => formatter.write_str("authentication is required"),
            Self::Forbidden => formatter.write_str("the action is not permitted"),
            Self::Conflict { resource, reason } => {
                write!(formatter, "{resource} conflict: {reason}")
            }
            Self::IdempotencyConflict => {
                formatter.write_str("idempotency key was already used for a different request")
            }
            Self::UnsupportedCapability {
                provider,
                capability,
            } => write!(
                formatter,
                "provider '{provider}' does not support capability '{capability}'"
            ),
            Self::ProviderUnavailable { provider, retryable } => {
                write!(formatter, "provider '{provider}' is unavailable (retryable: {retryable})")
            }
            Self::OperationNotRecoverable { operation_id } => {
                write!(formatter, "operation '{operation_id}' cannot be recovered automatically")
            }
        }
    }
}

impl Error for DomainError {}

pub type DomainResult<T> = Result<T, DomainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_is_safe_to_display() {
        let error = DomainError::Forbidden;
        assert_eq!(error.to_string(), "the action is not permitted");
    }

    #[test]
    fn provider_capability_error_is_explicit() {
        let error = DomainError::UnsupportedCapability {
            provider: "provider-x".to_owned(),
            capability: "snapshot",
        };
        assert_eq!(
            error.to_string(),
            "provider 'provider-x' does not support capability 'snapshot'"
        );
    }
}
