#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, frontend_contract::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::RouteErrorPolicy::ValidatedRead, authentication = frontend_contract::AuthenticationRequirement::Authenticated, method = frontend_contract::RouteMethod::Get, mutation = frontend_contract::RouteMutation::ReadOnly, obligations = frontend_contract::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_data_table", path = "/tables/{table}", path_parameter = crate::domain_types::AdminDataTable, request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminDataTableView, success_status = frontend_contract::SuccessStatus::Code200, transport = frontend_contract::AuthenticatedTransport)]
pub struct AdminDataTableRoute;
