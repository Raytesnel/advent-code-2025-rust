use std::fs;
use std::time::Instant;
mod assignments {
    pub mod ass_01;
}

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/assignment_1a", get(assignment_1a_handler))
        .route("/assignment_1b", get(assignment_1b_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_file(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("LogRocket: Should have been able to read the file{}")
}

async fn assignment_1a_handler() -> String {
    let input = read_file("src/inputs/ass_01.txt");
    let now = Instant::now();
    let result = assignments::ass_01::assignment_1_a(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}

async fn assignment_1b_handler() -> String {
    let input = read_file("src/inputs/ass_01.txt");
    let now = Instant::now();
    let result = assignments::ass_01::assignment_1_b(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_assignment_1_a() {
        let input = read_file("src/inputs/ass_01.txt");
        let expected = 1089;
        assert_eq!(assignments::ass_01::assignment_1_a(&input).await, expected);
    }

    #[tokio::test]
    async fn test_assignment_1_b() {
        let input = read_file("src/inputs/ass_01.txt");
        let expected = 6530;
        assert_eq!(assignments::ass_01::assignment_1_b(&input).await, expected);
    }
}
