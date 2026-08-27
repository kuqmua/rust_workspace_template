use super::{
    OpenApiContractText, OpenApiContractTextError, OpenApiValidationError, RuntimeRoutesRef,
    SerdeJsonOpenApiSerializationError, openapi_schema_references,
};

pub fn validate_openapi_contract<Document>(
    document: &Document,
    runtime_routes: RuntimeRoutesRef<'_>,
) -> Result<(), OpenApiValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiValidationError::DocumentSerialization(SerdeJsonOpenApiSerializationError::from(
            error,
        ))
    })?;
    let references = openapi_schema_references(&document_value)?;
    references.validate(&document_value)?;
    let schemas = document_value
        .pointer(constants_str::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingSchemas)?;
    schemas.keys().try_for_each(|name| {
        let contract_name = OpenApiContractText::try_from(name.clone()).map_err(|error| {
            OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
        })?;
        if references.0.contains(&contract_name) {
            Ok(())
        } else {
            Err(OpenApiValidationError::UnusedSchema(contract_name))
        }
    })?;

    let paths = document_value
        .get(constants_str::PATHS)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingPaths)?;
    let mut documented = std::collections::BTreeMap::new();
    paths.iter().try_for_each(|(path, path_item)| {
        path_item
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(method, _)| {
                [
                    constants_str::DELETE,
                    constants_str::GET,
                    constants_str::HEAD,
                    constants_str::OPTIONS,
                    constants_str::PATCH,
                    constants_str::POST,
                    constants_str::PUT,
                    constants_str::TRACE,
                ]
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(method))
            })
            .try_for_each(|(method, operation)| {
                let Some(operation_id) = operation
                    .get(constants_str::OPERATION_ID_JSON)
                    .and_then(serde_json::Value::as_str)
                else {
                    return Err(OpenApiValidationError::MissingOperationId(
                        OpenApiContractText::try_from(method.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(
                                error,
                            ))
                        })?,
                        OpenApiContractText::try_from(path.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(
                                error,
                            ))
                        })?,
                    ));
                };
                let _previous = documented.insert(
                    (method.to_ascii_uppercase(), path.clone()),
                    operation_id.to_owned(),
                );
                Ok(())
            })
    })?;
    runtime_routes.0.iter().try_for_each(|route| {
        let method = route.method().as_ref().to_ascii_uppercase();
        let path = route.path().as_ref().to_owned();
        let Some(operation_id) = documented.get(&(method.clone(), path.clone())) else {
            return Err(OpenApiValidationError::RuntimeRouteMissing(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ));
        };
        if operation_id == route.openapi_operation_id().as_ref() {
            Ok(())
        } else {
            Err(OpenApiValidationError::OperationIdMismatch(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(route.openapi_operation_id().as_ref().to_owned())
                    .map_err(|error| {
                        OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                    })?,
                OpenApiContractText::try_from(operation_id.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })?;
    documented.into_iter().try_for_each(|((method, path), _)| {
        if runtime_routes.0.iter().any(|route| {
            route
                .method()
                .as_ref()
                .eq_ignore_ascii_case(method.as_str())
                && route.path().as_ref() == path
        }) {
            Ok(())
        } else {
            Err(OpenApiValidationError::OpenApiRouteMissing(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError::from(error))
                })?,
            ))
        }
    })
}
