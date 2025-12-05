use axum::Json;
use std::time::Instant;
use crate::assignments::{assignment_2_a, assignment_2_b};
use crate::utils::read_file;

use super::returns::AssignmentResult;

#[utoipa::path(
    get,
    path = "/assignment_2a",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 2"
)]
pub async fn assignment_2a_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_02.txt");
    let now = Instant::now();
    let result = assignment_2_a(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}

#[utoipa::path(
    get,
    path = "/assignment_2b",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 2"
)]
pub async fn assignment_2b_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_02.txt");
    let now = Instant::now();
    let result = assignment_2_b(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}