use super::domain_types::{CreateNotificationReq, CreateNotificationRes};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::AuthenticationRequirement::Public,
    error_policy = frontend_contract::RouteErrorPolicy::Default,
    method = frontend_contract::RouteMethod::Post,
    mutation = frontend_contract::RouteMutation::Mutating,
    obligations = frontend_contract::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "create_notification",
    path = "/notifications",
    request = CreateNotificationReq,
    request_body = frontend_contract::RouteRequestBody::Json,
    response = CreateNotificationRes,
    success_status = frontend_contract::SuccessStatus::Code201,
    transport = frontend_contract::PublicTransport
)]
pub struct CreateNotificationRoute;
