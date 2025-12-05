use std::time::Instant;
use crate::assignments::{assignment_5_a, assignment_5_b};
use crate::utils::read_file;
use axum::Json;

use super::returns::AssignmentResult;

#[utoipa::path(
    get,
    path = "/assignment_5a",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 5"
)]
pub async fn assignment_5a_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_05.txt");
    let now = Instant::now();
    let result = assignment_5_a(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}

#[utoipa::path(
    get,
    path = "/assignment_5b",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 5"
)]
pub async fn assignment_5b_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_05.txt");
    let now = Instant::now();
    let result = assignment_5_b(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}