#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_frontend_contract_derive_typed_route::TypedRoute,
)]
#[typed_route(error_policy = frontend_contract::route_error_policy::RouteErrorPolicy::Default, authentication = crate::admin_rule_requirement::admin_rule_requirement(crate::admin_rule::AdminRule::RolesDelete), method = frontend_contract::route_method::RouteMethod::Get, mutation = frontend_contract::route_mutation::RouteMutation::ReadOnly, obligations = frontend_contract::route_coverage_obligation::AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS, openapi_operation_id = "delete_roles_payload_example", path = "/roles/delete_payload_example/read", request = crate::admin_no_body::AdminNoBody, response = crate::admin_delete_roles_request::AdminDeleteRolesRequest, success_status = frontend_contract::success_status::SuccessStatus::Code200, transport = frontend_contract::authenticated_transport::AuthenticatedTransport)]
pub struct AdminDeleteRolesPayloadExampleRoute;
