pub fn apply_openapi_success_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: crate::typed_route::TypedRoute,
{
    let metadata = Route::metadata();
    operation
        .responses
        .responses
        .retain(|status, _response| !status.starts_with('2'));
    let status = metadata.success_status().transport_status().to_string();
    let mut response = utoipa::openapi::response::Response::new(status.clone());
    if metadata.success_status() != crate::success_status::SuccessStatus::Code204
        && let Some(schema) = Route::openapi_response_schema()
    {
        let _previous_content = response.content.insert(
            constants_str::APPLICATION_JSON.to_owned(),
            utoipa::openapi::Content::new(Some(
                utoipa::openapi::RefOr::<utoipa::openapi::Schema>::from(schema),
            )),
        );
    }
    let _previous_response = operation
        .responses
        .responses
        .insert(status, utoipa::openapi::RefOr::T(response));
}
