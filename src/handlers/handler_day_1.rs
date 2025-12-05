use crate::assignments::{assignment_1_a, assignment_1_b};
use crate::utils::read_file;
use axum::Json;
use std::time::Instant;

use super::returns::AssignmentResult;

#[utoipa::path(
    post,
    path = "/assignment_1a",
    request_body = String,
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 1"
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


#[utoipa::path(
    get,
    path = "/assignment_1b",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 1"
)]
pub async fn assignment_1b_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_01.txt");
    let now = Instant::now();
    let result = assignment_1_b(&input).await;
    let elapsed = now.elapsed();
    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros() as u128,
    })
}
