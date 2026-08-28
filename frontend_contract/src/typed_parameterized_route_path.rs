use super::{ParameterizedRoute, ParameterizedRoutePath};

#[must_use]
pub fn typed_parameterized_route_path<Route>(parameter: &Route::Parameter) -> ParameterizedRoutePath
where
    Route: ParameterizedRoute,
{
    Route::path(parameter)
}
