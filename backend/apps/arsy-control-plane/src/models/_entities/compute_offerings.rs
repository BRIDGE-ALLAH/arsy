//! SeaORM entity for compute_offerings; kept in sync with its migration.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "compute_offerings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub region_id: Uuid,
    pub code: String,
    pub name: String,
    pub capability_key: String,
    pub vcpu_count: i32,
    pub memory_mib: i32,
    pub storage_gib: i32,
    pub image_id: String,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
