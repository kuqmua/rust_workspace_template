pub fn validate_typed_route_contract<Route>(
    observed: frontend_contract::route_metadata::RouteMetadata,
) -> Result<(), crate::route_contract_mismatches::RouteContractMismatches>
where
    Route: frontend_contract::typed_route::TypedRoute,
{
    crate::validate_route_contract_metadata::validate_route_contract_metadata(
        Route::metadata(),
        observed,
    )
}
