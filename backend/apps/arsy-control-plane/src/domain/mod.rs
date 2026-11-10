//! Framework-independent domain primitives used by the ARSY Control Plane.

pub mod errors;
pub mod ids;

pub use errors::{ConflictCode, DomainError, DomainErrorKind, EntityKind, ProviderErrorCode, RetryableCode, ValidationCode};
pub use ids::{OperationId, ProjectId, ProviderId, RegionId, ResourceId, TenantId};
