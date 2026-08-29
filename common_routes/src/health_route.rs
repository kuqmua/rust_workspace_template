#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract_macros::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::authentication_requirement::AuthenticationRequirement::Public,
    error_response = crate::health_report::HealthReport,
    error_statuses = &[frontend_contract::route_error_status::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::route_method::RouteMethod::Get,
    mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly,
    obligations = frontend_contract::route_coverage_obligation::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health",
    path = "/health",
    request = crate::common_no_body::CommonNoBody,
    response = crate::health_report::HealthReport,
    success_status = frontend_contract::success_status::SuccessStatus::Code200,
    transport = frontend_contract::public_transport::PublicTransport
)]
pub struct HealthRoute;
