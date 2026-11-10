//! Strongly typed identifiers for control-plane aggregates.
//!
//! Every identifier wraps a UUID but remains distinct at compile time. Use
//! separate ID types at domain and application boundaries instead of passing
//! unvalidated UUIDs or strings between tenants and resources.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates an identifier from an already validated UUID.
            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Generates a new random identifier.
            #[must_use]
            pub fn generate() -> Self {
                Self(Uuid::new_v4())
            }

            /// Borrows the underlying UUID.
            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Returns the underlying UUID by value.
            #[must_use]
            pub const fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl From<Uuid> for $name {
            fn from(value: Uuid) -> Self {
                Self::from_uuid(value)
            }
        }

        impl From<$name> for Uuid {
            fn from(value: $name) -> Self {
                value.into_uuid()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(value).map(Self::from_uuid)
            }
        }
    };
}

define_id!(UserId);
define_id!(OrganizationId);
define_id!(MembershipId);
define_id!(ProjectId);
define_id!(EnvironmentId);
define_id!(ProviderAccountId);
define_id!(ResourceId);
define_id!(OperationId);
define_id!(NodeId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_are_parseable_and_round_trip() {
        let id = OrganizationId::generate();
        let parsed: OrganizationId = id.to_string().parse().expect("valid UUID");
        assert_eq!(id, parsed);
    }

    #[test]
    fn serde_uses_the_uuid_string_representation() {
        let id = ProjectId::generate();
        let encoded = serde_json::to_string(&id).expect("serialize ID");
        let decoded: ProjectId = serde_json::from_str(&encoded).expect("deserialize ID");
        assert_eq!(id, decoded);
    }

    #[test]
    fn distinct_domain_ids_have_distinct_types() {
        fn accepts_project(_: ProjectId) {}
        accepts_project(ProjectId::generate());
        // A UserId cannot be passed to the ProjectId function: the compiler
        // enforces this invariant before runtime.
        let _user = UserId::generate();
    }
}
