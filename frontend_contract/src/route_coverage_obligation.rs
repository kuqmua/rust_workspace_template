#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteCoverageObligation {
    IntegrationFixture,
    OpenApiOperation,
    PayloadValidation,
    ReplayValidation,
    SecurityValidation,
}
pub const PUBLIC_READ_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
];
pub const PUBLIC_MUTATING_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::ReplayValidation,
];
pub const AUTHENTICATED_READ_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::SecurityValidation,
];
pub const AUTHENTICATED_MUTATING_ROUTE_COVERAGE_OBLIGATIONS: &[RouteCoverageObligation] = &[
    RouteCoverageObligation::IntegrationFixture,
    RouteCoverageObligation::OpenApiOperation,
    RouteCoverageObligation::PayloadValidation,
    RouteCoverageObligation::ReplayValidation,
    RouteCoverageObligation::SecurityValidation,
];
