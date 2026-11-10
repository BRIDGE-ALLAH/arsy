use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// Stable category for domain failures. HTTP status mapping belongs to the API layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainErrorKind {
    Validation,
    NotFound,
    Forbidden,
    Conflict,
    Retryable,
    Provider,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Tenant,
    Project,
    Provider,
    Region,
    Resource,
    Operation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCode {
    InvalidIdentifier,
    InvalidInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictCode {
    AlreadyExists,
    IdempotencyConflict,
    InvalidStateTransition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryableCode {
    TemporarilyUnavailable,
    RateLimited,
    DependencyTimeout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderErrorCode {
    Unavailable,
    Rejected,
    InvalidResponse,
    Unknown,
}

/// Framework-independent domain failure.
///
/// This type contains only stable classifications, never raw provider messages,
/// credentials, SQL errors, or other untrusted error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "details", rename_all = "snake_case")]
pub enum DomainError {
    Validation { code: ValidationCode },
    NotFound { entity: EntityKind },
    Forbidden,
    Conflict { code: ConflictCode },
    Retryable { code: RetryableCode },
    Provider {
        code: ProviderErrorCode,
        retryable: bool,
    },
}

impl DomainError {
    #[must_use]
    pub const fn kind(self) -> DomainErrorKind {
        match self {
            Self::Validation { .. } => DomainErrorKind::Validation,
            Self::NotFound { .. } => DomainErrorKind::NotFound,
            Self::Forbidden => DomainErrorKind::Forbidden,
            Self::Conflict { .. } => DomainErrorKind::Conflict,
            Self::Retryable { .. } => DomainErrorKind::Retryable,
            Self::Provider { .. } => DomainErrorKind::Provider,
        }
    }

    #[must_use]
    pub const fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::Retryable { .. } | Self::Provider { retryable: true, .. }
        )
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "domain error: {:?}", self.kind())
    }
}

impl Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_categories_remain_distinct() {
        let cases = [
            (DomainError::Validation { code: ValidationCode::InvalidInput }, DomainErrorKind::Validation),
            (DomainError::NotFound { entity: EntityKind::Resource }, DomainErrorKind::NotFound),
            (DomainError::Forbidden, DomainErrorKind::Forbidden),
            (DomainError::Conflict { code: ConflictCode::AlreadyExists }, DomainErrorKind::Conflict),
            (DomainError::Retryable { code: RetryableCode::RateLimited }, DomainErrorKind::Retryable),
            (DomainError::Provider { code: ProviderErrorCode::Unavailable, retryable: false }, DomainErrorKind::Provider),
        ];

        for (error, expected) in cases {
            assert_eq!(error.kind(), expected);
        }
    }

    #[test]
    fn retryability_is_explicit_and_bounded_by_the_classification() {
        assert!(DomainError::Retryable {
            code: RetryableCode::TemporarilyUnavailable,
        }
        .is_retryable());
        assert!(DomainError::Provider {
            code: ProviderErrorCode::Unavailable,
            retryable: true,
        }
        .is_retryable());
        assert!(!DomainError::Provider {
            code: ProviderErrorCode::Rejected,
            retryable: false,
        }
        .is_retryable());
        assert!(!DomainError::Conflict {
            code: ConflictCode::AlreadyExists,
        }
        .is_retryable());
    }

    #[test]
    fn serialized_errors_expose_only_stable_codes() {
        let error = DomainError::Provider {
            code: ProviderErrorCode::Unavailable,
            retryable: true,
        };
        let json = serde_json::to_string(&error).expect("serialize domain error");

        assert_eq!(
            json,
            r#"{"kind":"provider","details":{"code":"unavailable","retryable":true}}"#
        );
        assert!(!json.contains("password"));
    }
}
