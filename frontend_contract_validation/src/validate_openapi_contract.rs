pub fn validate_openapi_contract<Document>(
    document: &Document,
    runtime_routes: crate::runtime_routes_ref::RuntimeRoutesRef<'_>,
) -> Result<(), crate::open_api_validation_error::OpenApiValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        crate::open_api_validation_error::OpenApiValidationError::DocumentSerialization(
            crate::serde_json_open_api_serialization_error::SerdeJsonOpenApiSerializationError::from(error),
        )
    })?;
    let references = crate::openapi_schema_references::openapi_schema_references(&document_value)?;
    references.validate(&document_value)?;
    let schemas = document_value
        .pointer(constants_str::catalog::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(crate::open_api_validation_error::OpenApiValidationError::MissingSchemas)?;
    schemas.keys().try_for_each(|name| {
        let contract_name =
            crate::open_api_contract_text::OpenApiContractText::try_from(name.clone())
                .map_err(crate::open_api_validation_error::OpenApiValidationError::TextTooLong)?;
        if references.contains(&contract_name) {
            Ok(())
        } else {
            Err(
                crate::open_api_validation_error::OpenApiValidationError::UnusedSchema(
                    contract_name,
                ),
            )
        }
    })?;

    let paths = document_value
        .get(constants_str::catalog::PATHS)
        .and_then(serde_json::Value::as_object)
        .ok_or(crate::open_api_validation_error::OpenApiValidationError::MissingPaths)?;
    let mut documented = std::collections::BTreeMap::new();
    paths.iter().try_for_each(|(path, path_item)| {
        path_item
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(method, _)| {
                [
                    constants_str::integration_fixtures::DELETE,
                    constants_str::catalog::GET,
                    constants_str::test_fixtures::HEAD,
                    constants_str::test_fixtures::OPTIONS,
                    constants_str::catalog::PATCH,
                    constants_str::catalog::POST,
                    constants_str::catalog::PUT,
                    constants_str::test_fixtures::TRACE,
                ]
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(method))
            })
            .try_for_each(|(method, operation)| {
                let Some(operation_id) = operation
                    .get(constants_str::test_fixtures::OPERATION_ID_JSON)
                    .and_then(serde_json::Value::as_str)
                else {
                    return Err(
                        crate::open_api_validation_error::OpenApiValidationError::MissingOperationId(
                            crate::open_api_contract_text::OpenApiContractText::try_from(
                                method.clone(),
                            )
                            .map_err(
                                crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                            )?,
                            crate::open_api_contract_text::OpenApiContractText::try_from(path.clone())
                                .map_err(
                                    crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                                )?,
                        ),
                    );
                };
                let _previous = documented.insert(
                    (method.to_ascii_uppercase(), path.clone()),
                    operation_id.to_owned(),
                );
                Ok(())
            })
    })?;
    runtime_routes.iter().try_for_each(|route| {
        let method = route.method().as_ref().to_ascii_uppercase();
        let path = route.path().as_ref().to_owned();
        let Some(operation_id) = documented.get(&(method.clone(), path.clone())) else {
            return Err(
                crate::open_api_validation_error::OpenApiValidationError::RuntimeRouteMissing(
                    crate::open_api_contract_text::OpenApiContractText::try_from(method).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                    crate::open_api_contract_text::OpenApiContractText::try_from(path).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                ),
            );
        };
        if operation_id == route.openapi_operation_id().as_ref() {
            Ok(())
        } else {
            Err(
                crate::open_api_validation_error::OpenApiValidationError::OperationIdMismatch(
                    crate::open_api_contract_text::OpenApiContractText::try_from(method).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                    crate::open_api_contract_text::OpenApiContractText::try_from(path).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                    crate::open_api_contract_text::OpenApiContractText::try_from(
                        route.openapi_operation_id().as_ref().to_owned(),
                    )
                    .map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                    crate::open_api_contract_text::OpenApiContractText::try_from(
                        operation_id.clone(),
                    )
                    .map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                ),
            )
        }
    })?;
    documented.into_iter().try_for_each(|((method, path), _)| {
        if runtime_routes.iter().any(|route| {
            route
                .method()
                .as_ref()
                .eq_ignore_ascii_case(method.as_str())
                && route.path().as_ref() == path
        }) {
            Ok(())
        } else {
            Err(
                crate::open_api_validation_error::OpenApiValidationError::OpenApiRouteMissing(
                    crate::open_api_contract_text::OpenApiContractText::try_from(method).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                    crate::open_api_contract_text::OpenApiContractText::try_from(path).map_err(
                        crate::open_api_validation_error::OpenApiValidationError::TextTooLong,
                    )?,
                ),
            )
        }
    })
}
