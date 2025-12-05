use std::time::Instant;
use crate::assignments::{assignment_3_a, assignment_3_b};
use crate::utils::read_file;
use super::returns::AssignmentResult;
use axum::Json;


#[utoipa::path(
    get,
    path = "/assignment_3a",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 3"
)]
pub async fn assignment_3a_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_03.txt");
    let now = Instant::now();
    let result = assignment_3_a(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}
#[utoipa::path(
    get,
    path = "/assignment_3b",
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    ),
    tag = "Day 3"
)]
pub async fn assignment_3b_handler() -> Json<AssignmentResult> {
    let input = read_file("src/inputs/ass_03.txt");
    let now = Instant::now();
    let result = assignment_3_b(&input).await;
    let elapsed = now.elapsed();

    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros(),
    })
}