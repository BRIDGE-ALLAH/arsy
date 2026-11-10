use loco_rs::prelude::*;

async fn document() -> Result<Response> {
    let document: serde_json::Value = serde_json::from_str(include_str!("../../openapi.json"))
        .expect("the checked-in OpenAPI document must be valid JSON");
    format::json(document)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/v1")
        .add("/openapi.json", get(document))
}
