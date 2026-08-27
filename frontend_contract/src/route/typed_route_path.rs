use super::TypedRoute;

#[must_use]
pub fn typed_route_path<Route>() -> crate::domain_types::ContractStr
where
    Route: TypedRoute,
{
    Route::metadata().path()
}
