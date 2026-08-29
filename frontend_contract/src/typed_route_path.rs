#[must_use]
pub fn typed_route_path<Route>() -> crate::contract_str::ContractStr
where
    Route: crate::typed_route::TypedRoute,
{
    Route::metadata().path()
}
