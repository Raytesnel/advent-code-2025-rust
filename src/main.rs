mod api;
mod assignments;
mod handlers;
mod utils;
use crate::api::ApiDoc;
use crate::handlers::{
    assignment_1a_handler, assignment_1b_handler, assignment_2a_handler, assignment_2b_handler,
    assignment_3a_handler, assignment_3b_handler, assignment_4a_handler, assignment_4b_handler,
};
use axum::{routing::get, routing::post, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info)
        .try_init();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/assignment_1a", post(assignment_1a_handler))
        .route("/assignment_1b", get(assignment_1b_handler))
        .route("/assignment_2a", get(assignment_2a_handler))
        .route("/assignment_2b", get(assignment_2b_handler))
        .route("/assignment_3a", get(assignment_3a_handler))
        .route("/assignment_3b", get(assignment_3b_handler))
        .route("/assignment_4a", get(assignment_4a_handler))
        .route("/assignment_4b", get(assignment_4b_handler))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
