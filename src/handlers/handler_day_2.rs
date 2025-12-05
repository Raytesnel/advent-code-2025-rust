use axum::Json;
use std::time::Instant;
use crate::assignments::{assignment_2_a, assignment_2_b};
use crate::utils::read_file;
use serde::Serialize;

#[derive(Serialize)]
pub struct AssignmentResult {
    pub result: i64,
    pub time_elapsed: u128,
}


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


pub async fn assignment_2b_handler() -> String {
    let input = read_file("src/inputs/ass_02.txt");
    let now = Instant::now();
    let result = assignment_2_b(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}