use std::time::Instant;
use crate::assignments::{assignment_4_a, assignment_4_b};
use crate::utils::read_file;

pub async fn assignment_4a_handler() -> String {
    let input = read_file("src/inputs/ass_04.txt");
    let now = Instant::now();
    let result = assignment_4_a(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}

pub async fn assignment_4b_handler() -> String {
    let input = read_file("src/inputs/ass_04.txt");
    let now = Instant::now();
    let result = assignment_4_b(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}