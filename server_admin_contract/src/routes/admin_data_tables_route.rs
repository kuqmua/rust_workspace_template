use super::admin_permission_requirement;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::domain_types::RouteErrorPolicy::Default, authentication = admin_permission_requirement(crate::domain_types::AdminPermission::TablesRead), method = frontend_contract::domain_types::RouteMethod::Get, mutation = frontend_contract::domain_types::RouteMutation::ReadOnly, obligations = frontend_contract::domain_types::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "list_data_tables", path = "/tables", request = crate::domain_types::AdminNoBody, response = crate::domain_types::AdminDataTableCatalog, success_status = frontend_contract::domain_types::SuccessStatus::Code200, transport = frontend_contract::domain_types::AuthenticatedTransport)]
pub struct AdminDataTablesRoute;
