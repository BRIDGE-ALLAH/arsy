use std::{fmt, str::FromStr};

use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use super::errors::{DomainError, ValidationCode};

fn parse_canonical_uuid(value: &str) -> Result<Uuid, DomainError> {
    let parsed = Uuid::parse_str(value).map_err(|_| DomainError::Validation {
        code: ValidationCode::InvalidIdentifier,
    })?;

    if parsed.hyphenated().to_string() != value {
        return Err(DomainError::Validation {
            code: ValidationCode::InvalidIdentifier,
        });
    }

    Ok(parsed)
}

macro_rules! define_domain_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates a new random version-4 identifier.
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Wraps an existing database UUID without changing its value.
            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Parses the canonical lowercase, hyphenated UUID representation.
            ///
            /// # Errors
            /// Returns a validation error when the value is not canonical.
            pub fn parse(value: &str) -> Result<Self, DomainError> {
                parse_canonical_uuid(value).map(Self)
            }

            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            #[must_use]
            pub const fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
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

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{}", self.0.hyphenated())
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(&value).map_err(D::Error::custom)
            }
        }
    };
}

define_domain_id!(TenantId);
define_domain_id!(ProjectId);
define_domain_id!(ProviderId);
define_domain_id!(RegionId);
define_domain_id!(ResourceId);
define_domain_id!(OperationId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_domain_id_uses_canonical_uuid_strings_for_json() {
        let uuid = Uuid::parse_str("8f14e45f-ea9a-4d59-8f14-e45fea9a4d59").expect("valid test UUID");
        let ids = [
            serde_json::to_string(&TenantId::from_uuid(uuid)).expect("serialize tenant"),
            serde_json::to_string(&ProjectId::from_uuid(uuid)).expect("serialize project"),
            serde_json::to_string(&ProviderId::from_uuid(uuid)).expect("serialize provider"),
            serde_json::to_string(&RegionId::from_uuid(uuid)).expect("serialize region"),
            serde_json::to_string(&ResourceId::from_uuid(uuid)).expect("serialize resource"),
            serde_json::to_string(&OperationId::from_uuid(uuid)).expect("serialize operation"),
        ];

        assert!(ids.iter().all(|id| id == "\"8f14e45f-ea9a-4d59-8f14-e45fea9a4d59\""));
    }

    #[test]
    fn identifiers_round_trip_without_changing_the_value() {
        let id = ResourceId::new();
        let serialized = serde_json::to_string(&id).expect("serialize resource ID");
        let parsed: ResourceId = serde_json::from_str(&serialized).expect("deserialize resource ID");

        assert_eq!(id, parsed);
        assert_eq!(id.to_string(), serialized.trim_matches('"'));
    }

    #[test]
    fn non_canonical_or_invalid_identifiers_are_rejected() {
        assert!(TenantId::parse("not-a-uuid").is_err());
        assert!(TenantId::parse("8F14E45F-EA9A-4D59-8F14-E45FEA9A4D59").is_err());
        assert!(serde_json::from_str::<ProjectId>("\"not-a-uuid\"").is_err());
        assert!(serde_json::from_str::<ProjectId>("\"8F14E45F-EA9A-4D59-8F14-E45FEA9A4D59\"").is_err());
    }
}
