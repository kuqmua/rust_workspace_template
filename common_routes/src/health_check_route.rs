use super::CommonNoBody;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_response = (),
    error_statuses = &[frontend_contract::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::RouteMethod::Get,
    mutation = frontend_contract::RouteMutation::ReadOnly,
    obligations = frontend_contract::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health_check",
    path = "/health_check",
    request = CommonNoBody,
    response = CommonNoBody,
    success_status = frontend_contract::SuccessStatus::Code200,
    transport = frontend_contract::PublicTransport
)]
pub struct HealthCheckRoute;
