use super::{RouteRequestBody, TypedRoute};

pub fn apply_openapi_request_contract<Route>(operation: &mut utoipa::openapi::path::Operation)
where
    Route: TypedRoute,
{
    operation.request_body = match Route::request_body() {
        RouteRequestBody::Absent => None,
        RouteRequestBody::Json => Route::openapi_request_body_schema().map(|schema| {
            utoipa::openapi::request_body::RequestBodyBuilder::new()
                .required(Some(utoipa::openapi::Required::True))
                .content(
                    constants_str::APPLICATION_JSON,
                    utoipa::openapi::Content::new(Some(utoipa::openapi::RefOr::<
                        utoipa::openapi::Schema,
                    >::from(schema))),
                )
                .build()
        }),
    };
}
