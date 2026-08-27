use super::{CommonNoBody, HealthReport};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_response = HealthReport,
    error_statuses = &[frontend_contract::domain_types::RouteErrorStatus::ServiceUnavailable],
    method = frontend_contract::domain_types::RouteMethod::Get,
    mutation = frontend_contract::domain_types::RouteMutation::ReadOnly,
    obligations = frontend_contract::domain_types::PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "health",
    path = "/health",
    request = CommonNoBody,
    response = HealthReport,
    success_status = frontend_contract::domain_types::SuccessStatus::Code200,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct HealthRoute;
