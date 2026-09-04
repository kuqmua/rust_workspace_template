#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct RouteTestCapabilities {
    database: crate::route_database_usage::RouteDatabaseUsage,
    json_body: crate::route_json_body_usage::RouteJsonBodyUsage,
    response: crate::route_response_kind::RouteResponseKind,
}
