#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::ValidatedRead, authentication = crate::admin_rule_requirement::admin_rule_requirement(crate::admin_rule::AdminRule::PermissionResourceActionsRead), method = frontend_contract::route_method::RouteMethod::Post, mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "read_permission_resource_actions_table", path = "/permission_resource_actions/read", request = crate::admin_permission_resource_actions_read_request::AdminPermissionResourceActionsReadRequest, request_body = frontend_contract::route_request_body::RouteRequestBody::Json, response = crate::admin_data_table_view::AdminDataTableView, success_status = frontend_contract::success_status::SuccessStatus::Code200, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminPermissionResourceActionsTableRoute;
