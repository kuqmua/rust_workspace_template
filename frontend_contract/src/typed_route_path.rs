use super::TypedRoute;

#[must_use]
pub fn typed_route_path<Route>() -> crate::ContractStr
where
    Route: TypedRoute,
{
    Route::metadata().path()
}
