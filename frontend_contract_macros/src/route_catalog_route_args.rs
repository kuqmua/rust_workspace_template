#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogRouteArgs {
    contract: Option<crate::syn_expr::SynExpr>,
    path: Option<crate::syn_expr::SynExpr>,
    route: Option<crate::syn_type::SynType>,
    exclude_from_family: crate::std_bool::StdBool,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteCatalogRouteArgs {
    #[allow(
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        contract: Option<crate::syn_expr::SynExpr>,
        path: Option<crate::syn_expr::SynExpr>,
        route: Option<crate::syn_type::SynType>,
        exclude_from_family: crate::std_bool::StdBool,
    ) -> Self {
        Self {
            contract,
            path,
            route,
            exclude_from_family,
        }
    }
}
