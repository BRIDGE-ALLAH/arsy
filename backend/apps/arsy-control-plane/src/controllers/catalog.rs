use std::collections::HashMap;

use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::RegionId,
    models::{
        _entities::{compute_offerings, regions},
        catalog::{ComputeOfferingResponse, RegionResponse, RegionStatus},
    },
};

#[derive(Debug, Deserialize)]
struct OfferingsQuery {
    region_id: Option<String>,
}

/// Planned and disabled regions are returned for discovery but are never selectable.
#[debug_handler]
async fn list_regions(State(ctx): State<AppContext>) -> Result<Response> {
    let models = regions::Entity::find()
        .order_by_asc(regions::Column::Code)
        .all(&ctx.db)
        .await?;
    let items = models
        .into_iter()
        .map(RegionResponse::try_from)
        .collect::<Result<Vec<_>>>()?;
    format::json(items)
}

/// An offering is selectable only if both it and its region are enabled.
#[debug_handler]
async fn list_compute_offerings(
    Query(query): Query<OfferingsQuery>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let region_filter = match query.region_id {
        Some(value) => match RegionId::parse(&value) {
            Ok(id) => Some(id.into_uuid()),
            Err(_) => return bad_request("invalid region_id"),
        },
        None => None,
    };

    let mut offerings_query = compute_offerings::Entity::find();
    if let Some(region_id) = region_filter {
        offerings_query = offerings_query.filter(compute_offerings::Column::RegionId.eq(region_id));
    }
    let offerings = offerings_query
        .order_by_asc(compute_offerings::Column::Code)
        .all(&ctx.db)
        .await?;

    let region_models = regions::Entity::find().all(&ctx.db).await?;
    let region_statuses = region_models
        .into_iter()
        .map(|region| Ok((region.id, RegionStatus::parse(&region.status)?)))
        .collect::<Result<HashMap<Uuid, RegionStatus>>>()?;

    let mut items = Vec::with_capacity(offerings.len());
    for offering in offerings {
        let region_status = region_statuses
            .get(&offering.region_id)
            .copied()
            .unwrap_or(RegionStatus::Disabled);
        items.push(ComputeOfferingResponse::from_model(offering, region_status)?);
    }
    format::json(items)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1/catalog")
        .add("/regions", get(list_regions))
        .add("/compute-offerings", get(list_compute_offerings))
}
