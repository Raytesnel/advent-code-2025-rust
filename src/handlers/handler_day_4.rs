use std::time::Instant;
use crate::assignments::{assignment_4_a, assignment_4_b};
use crate::utils::read_file;
use axum::Json;

use super::returns::AssignmentResult;

#[utoipa::path(
    get,
    path = "/assignment_4a",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 4"
)]
pub async fn assignment_4a_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_04.txt");
    let now = Instant::now();
    let result = assignment_4_a(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}

#[utoipa::path(
    get,
    path = "/assignment_4b",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 4"
)]
pub async fn assignment_4b_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_04.txt");
    let now = Instant::now();
    let result = assignment_4_b(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}