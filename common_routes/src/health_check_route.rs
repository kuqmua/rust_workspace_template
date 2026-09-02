#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    error_response = (),
    error_statuses = &[frontend_contract::route_error_status::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::route_method::RouteMethod::Get,
    mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_check",
    path = "/health_check",
    request = crate::common_no_body::CommonNoBody,
    response = crate::common_no_body::CommonNoBody,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::public_transport::PublicTransport
)]
pub struct HealthCheckRoute;
