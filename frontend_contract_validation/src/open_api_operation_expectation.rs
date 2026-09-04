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
#[getters(bare)]
pub struct OpenApiOperationExpectation {
    #[constructor(order = 2)]
    #[getters(copy)]
    content_type: frontend_contract::contract_str::ContractStr,
    #[constructor(order = 0)]
    #[getters(copy)]
    metadata: frontend_contract::route_metadata::RouteMetadata,
    #[constructor(order = 3)]
    #[getters(copy)]
    security: crate::open_api_security_expectation::OpenApiSecurityExpectation,
    #[constructor(order = 1)]
    #[getters(copy)]
    status: crate::open_api_response_status::OpenApiResponseStatus,
}
