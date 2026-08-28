pub fn validate_typed_route_contract<Route>(
    observed: frontend_contract::RouteMetadata,
) -> Result<(), crate::route_contract_validation::RouteContractMismatches>
where
    Route: frontend_contract::TypedRoute,
{
    crate::route_contract_validation::validate_route_contract_metadata(Route::metadata(), observed)
}
