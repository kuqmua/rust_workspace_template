pub fn register_openapi_route_schemas<Route>(
    document: &mut crate::utoipa_open_api_ref_mut::UtoipaOpenApiRefMut<'_>,
) where
    Route: crate::typed_route::TypedRoute,
{
    let raw_components = document
        .0
        .components
        .get_or_insert_with(utoipa::openapi::schema::Components::new);
    let mut schema_components =
        crate::utoipa_open_api_components_ref_mut::UtoipaOpenApiComponentsRefMut::from(
            raw_components,
        );
    Route::register_openapi_schemas(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<crate::api_problem::ApiProblem>(
        &mut schema_components,
    );
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_detail::ApiProblemDetail,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_field::ApiProblemField,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_kind::ApiProblemKind,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_request_id::ApiProblemRequestId,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_status::ApiProblemStatus,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::api_problem_violation::ApiProblemViolation,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::filter_operation::FilterOperation,
    >(&mut schema_components);
    crate::register_openapi_schema::register_openapi_schema::<
        crate::filter_value_shape::FilterValueShape,
    >(&mut schema_components);
}
