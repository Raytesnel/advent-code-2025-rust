use serde::Serialize;
use utoipa::ToSchema;


#[derive(Serialize, ToSchema)]
pub struct AssignmentResult {
    pub result: i64,
    pub time_elapsed: u128,
}