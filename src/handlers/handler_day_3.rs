use std::time::Instant;
use crate::assignments::{assignment_3_a, assignment_3_b};
use crate::utils::read_file;


pub async fn assignment_3a_handler() -> String {
    let input = read_file("src/inputs/ass_03.txt");
    let now = Instant::now();
    let result = assignment_3_a(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}

pub async fn assignment_3b_handler() -> String {
    let input = read_file("src/inputs/ass_03.txt");
    let now = Instant::now();
    let result = assignment_3_b(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}