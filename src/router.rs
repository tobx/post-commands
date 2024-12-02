use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::method_routing as m, Json, Router};

use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{app, command};

pub fn build(state: Arc<app::State>, cors_is_permitted: bool) -> Router {
    let mut router = Router::new()
        .route("/execute", m::post(execute))
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    if cors_is_permitted {
        router = router.layer(CorsLayer::permissive());
    }
    router
}

async fn execute(
    State(state): State<Arc<app::State>>,
    Json(command_spec): Json<command::Spec>,
) -> crate::Result<impl IntoResponse> {
    state.commands.execute(command_spec)?;
    Ok(())
}
