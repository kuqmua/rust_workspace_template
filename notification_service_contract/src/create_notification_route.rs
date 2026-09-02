#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default,
    method = frontend_contract::route_method::RouteMethod::Post,
    mutation = frontend_contract::route_mutation::RouteMutation::Mutating,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "create_notification",
    path = "/notifications",
    request = crate::create_notification_request::CreateNotificationRequest,
    request_body = frontend_contract::route_request_body::RouteRequestBody::Json,
    response = crate::create_notification_response::CreateNotificationResponse,
    success_status = frontend_contract::success_status::SuccessStatus::Code201,
    transport = frontend_contract::public_transport::PublicTransport
)]
pub struct CreateNotificationRoute;
