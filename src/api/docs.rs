use utoipa::OpenApi;

use crate::handlers::AssignmentResult;

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::handler_day_1::assignment_1a_handler),
    components(schemas(AssignmentResult))
)]
pub struct ApiDoc;
