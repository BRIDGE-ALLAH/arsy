//! Domain types and invariants. This layer must remain transport- and provider-neutral.

pub mod errors;
pub mod ids;

pub use errors::{DomainError, DomainResult};
pub use ids::{
    EnvironmentId, MembershipId, NodeId, OperationId, OrganizationId, ProjectId, ProviderAccountId,
    ResourceId, UserId,
};
