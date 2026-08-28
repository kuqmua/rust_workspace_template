use super::TypedRoute;

pub fn apply_openapi_error_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: TypedRoute,
{
    operation
        .responses
        .responses
        .retain(|status, _response| !status.starts_with('4') && !status.starts_with('5'));
    Route::metadata()
        .error_statuses()
        .iter()
        .copied()
        .for_each(|error_status| {
            let status = error_status.transport_status().to_string();
            let mut response = utoipa::openapi::response::Response::new(status.clone());
            if let Some(schema) = Route::openapi_error_response_schema(error_status) {
                let _previous_content = response.content.insert(
                    constants_str::APPLICATION_JSON.to_owned(),
                    utoipa::openapi::Content::new(Some(utoipa::openapi::RefOr::<
                        utoipa::openapi::Schema,
                    >::from(schema))),
                );
            }
            if error_status == crate::domain_types::RouteErrorStatus::RateLimited {
                let _previous_header = response.headers.insert(
                    constants_str::RETRY_AFTER.to_owned(),
                    utoipa::openapi::header::Header::default(),
                );
            }
            let _previous_response = operation
                .responses
                .responses
                .insert(status, utoipa::openapi::RefOr::T(response));
        });
}
