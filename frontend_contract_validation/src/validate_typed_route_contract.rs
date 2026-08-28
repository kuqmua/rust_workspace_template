pub fn validate_typed_route_contract<Route>(
    observed: frontend_contract::domain_types::RouteMetadata,
) -> Result<(), super::RouteContractMismatches>
where
    Route: frontend_contract::domain_types::TypedRoute,
{
    super::validate_route_contract_metadata(Route::metadata(), observed)
}
