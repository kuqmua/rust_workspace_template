pub fn apply_openapi_path_parameter_contract<Route>(
    operation: &mut utoipa::openapi::path::Operation,
) where
    Route: crate::typed_route::TypedRoute,
{
    if let Some(parameter) = Route::openapi_path_parameter() {
        operation
            .parameters
            .get_or_insert_default()
            .push(parameter.into());
    }
}
