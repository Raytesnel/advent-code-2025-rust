use utoipa::OpenApi;

use crate::handlers::AssignmentResult;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::handler_day_1::assignment_1a_handler,
        crate::handlers::handler_day_1::assignment_1b_handler,
        crate::handlers::handler_day_2::assignment_2a_handler,
        crate::handlers::handler_day_2::assignment_2b_handler,
        crate::handlers::handler_day_3::assignment_3a_handler,
        crate::handlers::handler_day_3::assignment_3b_handler,
        crate::handlers::handler_day_4::assignment_4a_handler,
        crate::handlers::handler_day_4::assignment_4b_handler,
        crate::handlers::handler_day_5::assignment_5a_handler,
        crate::handlers::handler_day_5::assignment_5b_handler,
    ),
    components(schemas(AssignmentResult))
)]
pub struct ApiDoc;
