use std::time::Instant;
use crate::assignments::{assignment_5_a, assignment_5_b};
use crate::utils::read_file;

pub async fn assignment_5a_handler() -> String {
    let input = read_file("src/inputs/ass_05.txt");
    let now = Instant::now();
    let result = assignment_5_a(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}
pub async fn assignment_5b_handler() -> String {
    let input = read_file("src/inputs/ass_05.txt");
    let now = Instant::now();
    let result = assignment_5_b(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}