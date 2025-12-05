use crate::assignments::{assignment_1_a, assignment_1_b};
use crate::utils::read_file;
use axum::Json;
use serde::Serialize;
use std::time::Instant;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct AssignmentResult {
    result: i64,
    time_elapsed: u128,
}

#[utoipa::path(
    post,
    path = "/assignment_1a",
    request_body = String,
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    )
)]
pub async fn assignment_1a_handler(body: String) -> Json<AssignmentResult> {
    let now = Instant::now();
    let result = assignment_1_a(&body).await;
    let elapsed = now.elapsed();
    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros() as u128,
    })
}

pub async fn assignment_1b_handler() -> String {
    let input = read_file("src/inputs/ass_01.txt");
    let now = Instant::now();
    let result = assignment_1_b(&input).await;
    let elapsed = now.elapsed();

    format!("Result: {} ({}µs)", result, elapsed.as_micros())
}
