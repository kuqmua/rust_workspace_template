use super::{CreateNotificationReq, CreateNotificationRes};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(
    authentication = frontend_contract::domain_types::AuthenticationRequirement::Public,
    error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default,
    method = frontend_contract::domain_types::RouteMethod::Post,
    mutation = frontend_contract::domain_types::RouteMutation::Mutating,
    obligations = frontend_contract::domain_types::PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS,
    openapi_operation_id = "create_notification",
    path = "/notifications",
    request = CreateNotificationReq,
    request_body = frontend_contract::domain_types::RouteRequestBody::Json,
    response = CreateNotificationRes,
    success_status = frontend_contract::domain_types::SuccessStatus::Code201,
    transport = frontend_contract::domain_types::PublicTransport
)]
pub struct CreateNotificationRoute;
