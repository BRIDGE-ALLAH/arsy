use loco_rs::prelude::*;
use serde::Serialize;

use crate::domain::{RegionId, ResourceId};

pub use super::_entities::{compute_offerings, regions};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionStatus {
    Planned,
    Enabled,
    Disabled,
}

impl RegionStatus {
    pub fn parse(value: &str) -> loco_rs::Result<Self> {
        match value {
            "planned" => Ok(Self::Planned),
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Err(Error::string("catalog contains an invalid region status")),
        }
    }

    #[must_use]
    pub const fn is_selectable(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OfferingStatus {
    Unverified,
    Enabled,
    Disabled,
}

impl OfferingStatus {
    pub fn parse(value: &str) -> loco_rs::Result<Self> {
        match value {
            "unverified" => Ok(Self::Unverified),
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
            _ => Err(Error::string("catalog contains an invalid offering status")),
        }
    }

    #[must_use]
    pub const fn is_selectable(self, region_status: RegionStatus) -> bool {
        matches!(self, Self::Enabled) && region_status.is_selectable()
    }
}

#[derive(Debug, Serialize)]
pub struct RegionResponse {
    pub id: RegionId,
    pub code: String,
    pub name: String,
    pub location: String,
    pub status: RegionStatus,
    pub selectable: bool,
}

impl TryFrom<regions::Model> for RegionResponse {
    type Error = loco_rs::Error;

    fn try_from(model: regions::Model) -> loco_rs::Result<Self> {
        let status = RegionStatus::parse(&model.status)?;
        Ok(Self {
            id: RegionId::from_uuid(model.id),
            code: model.code,
            name: model.name,
            location: model.location,
            status,
            selectable: status.is_selectable(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ComputeOfferingResponse {
    pub id: ResourceId,
    pub region_id: RegionId,
    pub code: String,
    pub name: String,
    pub capability_key: String,
    pub vcpu_count: i32,
    pub memory_mib: i32,
    pub storage_gib: i32,
    pub image_id: String,
    pub status: OfferingStatus,
    pub selectable: bool,
}

impl ComputeOfferingResponse {
    pub fn from_model(
        model: compute_offerings::Model,
        region_status: RegionStatus,
    ) -> loco_rs::Result<Self> {
        let status = OfferingStatus::parse(&model.status)?;
        Ok(Self {
            id: ResourceId::from_uuid(model.id),
            region_id: RegionId::from_uuid(model.region_id),
            code: model.code,
            name: model.name,
            capability_key: model.capability_key,
            vcpu_count: model.vcpu_count,
            memory_mib: model.memory_mib,
            storage_gib: model.storage_gib,
            image_id: model.image_id,
            status,
            selectable: status.is_selectable(region_status),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_status_fails_closed() {
        assert!(RegionStatus::parse("available").is_err());
        assert!(OfferingStatus::parse("available").is_err());
    }

    #[test]
    fn only_enabled_regions_are_selectable() {
        assert!(RegionStatus::Enabled.is_selectable());
        assert!(!RegionStatus::Planned.is_selectable());
        assert!(!RegionStatus::Disabled.is_selectable());
    }

    #[test]
    fn offering_requires_enabled_offering_and_region() {
        assert!(OfferingStatus::Enabled.is_selectable(RegionStatus::Enabled));
        assert!(!OfferingStatus::Unverified.is_selectable(RegionStatus::Enabled));
        assert!(!OfferingStatus::Disabled.is_selectable(RegionStatus::Enabled));
        assert!(!OfferingStatus::Enabled.is_selectable(RegionStatus::Planned));
        assert!(!OfferingStatus::Enabled.is_selectable(RegionStatus::Disabled));
    }
}
