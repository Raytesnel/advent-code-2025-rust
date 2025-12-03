use std::fs;
use std::time::Instant;
mod assignments {
    pub mod ass_01;
    pub mod ass_02;
    pub mod ass_03;
}
use axum::Json;
use axum::{routing::get, routing::post, Router};
use serde::Serialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(assignment_1a_handler,), components(schemas(AssignmentResult)))]
struct ApiDoc;

#[derive(Serialize, ToSchema)]
pub struct AssignmentResult {
    result: i64,
    time_elapsed: u128,
}

#[utoipa::path(
    post,
    path = "/assignment_1a",
    request_body = String,
    responses(
        (status = 200, description = "Result", body = AssignmentResult)
    )
)]
async fn assignment_1a_handler(body: String) -> Json<AssignmentResult> {
    let now = Instant::now();
    let result = assignments::ass_01::assignment_1_a(&body).await;
    let elapsed = now.elapsed();
    Json(AssignmentResult {
        result: result as i64,
        time_elapsed: elapsed.as_micros() as u128,
    })
}

#[tokio::main]
async fn main() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info)
        .try_init();
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/assignment_1a", post(assignment_1a_handler))
        .route("/assignment_1b", get(assignment_1b_handler))
        .route("/assignment_2a", get(assignment_2a_handler))
        .route("/assignment_2b", get(assignment_2b_handler))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_file(input_path: &str) -> String {
    fs::read_to_string(input_path).expect("LogRocket: Should have been able to read the file{}")
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

async fn assignment_2a_handler() -> String {
    let input = read_file("src/inputs/ass_02.txt");
    let now = Instant::now();
    let result = assignments::ass_02::assignment_2_a(&input).await;
    let elapsed = now.elapsed();

    format!(
        "Result: {} (Time elapsed: {}µs)",
        result,
        elapsed.as_micros()
    )
}
async fn assignment_2b_handler() -> String {
    let input = read_file("src/inputs/ass_02.txt");
    let now = Instant::now();
    let result = assignments::ass_02::assignment_2_b(&input).await;
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

    #[tokio::test]
    async fn test_assignment_2_a() {
        let input = read_file("src/inputs/ass_02.txt");
        let expected: i64 = 44487518055;
        let result: i64 = assignments::ass_02::assignment_2_a(&input).await;
        assert_eq!(result, expected);
    }
    #[tokio::test]
    async fn test_assignment_2_b() {
        let input = read_file("src/inputs/ass_02.txt");
        let expected: i64 = 53481866137;
        let result: i64 = assignments::ass_02::assignment_2_b(&input).await;
        assert_eq!(result, expected);
    }    
    #[tokio::test]
    async fn test_assignment_3_a() {
        let input = read_file("src/inputs/ass_03.txt");
        let expected:u64 = 17301;
        let result:u64 = assignments::ass_03::assignment_3_a(&input).await;
        assert_eq!(result, expected);
    }
    #[tokio::test]
    async fn test_assignment_3_b() {
        let input = read_file("src/inputs/ass_03.txt");
        let expected:u64 = 17301;
        let result:u64 = assignments::ass_03::assignment_3_b(&input).await;
        assert_eq!(result, expected);
    }
}
