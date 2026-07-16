const OPENAPI_CONTRACT_TEXT_MAX_LEN: usize = 1_048_576usize;

#[derive(Clone, Debug, Eq, PartialEq, newtype::BoundedString)]
#[bounded_string(max = OPENAPI_CONTRACT_TEXT_MAX_LEN)]
pub struct OpenApiContractText(String);

#[derive(Clone, Copy, Debug)]
pub struct OpenApiContractTextError(OpenApiContractTextTryFromStringError);
impl std::fmt::Display for OpenApiContractTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct SerdeJsonOpenApiSerializationError(serde_json::Error);
impl std::fmt::Display for SerdeJsonOpenApiSerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for SerdeJsonOpenApiSerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RuntimeRoutesRef<'value_lt>(&'value_lt [crate::RouteMetadata]);
impl<'value_lt> From<&'value_lt [crate::RouteMetadata]> for RuntimeRoutesRef<'value_lt> {
    fn from(value: &'value_lt [crate::RouteMetadata]) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum OpenApiValidationError {
    DocumentSerialization(SerdeJsonOpenApiSerializationError),
    MissingOperationId(OpenApiContractText, OpenApiContractText),
    MissingPaths,
    MissingSchemaReference(OpenApiContractText),
    MissingSchemas,
    OpenApiRouteMissing(OpenApiContractText, OpenApiContractText),
    OperationIdMismatch(
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
        OpenApiContractText,
    ),
    RuntimeRouteMissing(OpenApiContractText, OpenApiContractText),
    TextTooLong(OpenApiContractTextError),
    UnusedSchema(OpenApiContractText),
}
impl std::fmt::Display for OpenApiValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for OpenApiValidationError {}

pub fn validate_openapi_contract<Document>(
    document: &Document,
    runtime_routes: RuntimeRoutesRef<'_>,
) -> Result<(), OpenApiValidationError>
where
    Document: serde::Serialize,
{
    let document_value = serde_json::to_value(document).map_err(|error| {
        OpenApiValidationError::DocumentSerialization(SerdeJsonOpenApiSerializationError(error))
    })?;
    let schemas = document_value
        .pointer(str_constants::COMPONENTS_SCHEMAS_ALT)
        .and_then(serde_json::Value::as_object)
        .ok_or(OpenApiValidationError::MissingSchemas)?;
    let mut references = std::collections::BTreeSet::new();
    let mut pending = vec![&document_value];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => {
                if let Some(name) = values
                    .get(str_constants::DOLLAR_REF)
                    .and_then(serde_json::Value::as_str)
                    .and_then(|reference| reference.strip_prefix(str_constants::COMPONENTS_SCHEMAS))
                {
                    let _inserted: bool = references.insert(name.to_owned());
                }
                pending.extend(values.values());
            }
            serde_json::Value::Bool(_)
            | serde_json::Value::Null
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    references.iter().try_for_each(|reference| {
        if schemas.contains_key(reference) {
            Ok(())
        } else {
            Err(OpenApiValidationError::MissingSchemaReference(
                OpenApiContractText::try_from(reference.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
            ))
        }
    })?;
    schemas.keys().try_for_each(|name| {
        if references.contains(name) {
            Ok(())
        } else {
            Err(OpenApiValidationError::UnusedSchema(
                OpenApiContractText::try_from(name.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
            ))
        }
    })?;

    let paths = document_value
        .get(str_constants::PATHS)
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
                    str_constants::DELETE,
                    str_constants::GET,
                    str_constants::HEAD,
                    str_constants::OPTIONS,
                    str_constants::PATCH,
                    str_constants::POST,
                    str_constants::PUT,
                    str_constants::TRACE,
                ]
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(method))
            })
            .try_for_each(|(method, operation)| {
                let Some(operation_id) = operation
                    .get(str_constants::OPERATION_ID_JSON)
                    .and_then(serde_json::Value::as_str)
                else {
                    return Err(OpenApiValidationError::MissingOperationId(
                        OpenApiContractText::try_from(method.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                        })?,
                        OpenApiContractText::try_from(path.clone()).map_err(|error| {
                            OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
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
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
            ));
        };
        if operation_id == route.openapi_operation_id().as_ref() {
            Ok(())
        } else {
            Err(OpenApiValidationError::OperationIdMismatch(
                OpenApiContractText::try_from(method).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
                OpenApiContractText::try_from(route.openapi_operation_id().as_ref().to_owned())
                    .map_err(|error| {
                        OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                    })?,
                OpenApiContractText::try_from(operation_id.clone()).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
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
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
                OpenApiContractText::try_from(path).map_err(|error| {
                    OpenApiValidationError::TextTooLong(OpenApiContractTextError(error))
                })?,
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_document_matches_runtime_route_and_references() {
        let document = serde_json::json!({
            str_constants::PATHS: { str_constants::TEST_OPENAPI_PATH: { str_constants::GET_LOWERCASE: {
                str_constants::OPERATION_ID_JSON: str_constants::TEST_OPENAPI_OPERATION_ID,
                str_constants::RESPONSES: { str_constants::STATUS_OK: { str_constants::OPENAPI_CONTENT: { str_constants::APPLICATION_JSON: {
                    str_constants::JSON_SCHEMA: { str_constants::DOLLAR_REF: str_constants::TEST_OPENAPI_SCHEMA_REF }
                }}}}
            }}},
            str_constants::COMPONENTS: { str_constants::SCHEMAS: { str_constants::TEST_OPENAPI_SCHEMA: { str_constants::JSON_TYPE: str_constants::OBJECT }}}
        });
        let routes = [crate::RouteMetadata::new(
            str_constants::GET.into(),
            str_constants::TEST_OPENAPI_OPERATION_ID.into(),
            str_constants::TEST_OPENAPI_PATH.into(),
        )];
        assert!(matches!(
            super::validate_openapi_contract(&document, routes.as_slice().into()),
            Ok(())
        ));
    }

    #[test]
    fn dangling_reference_is_rejected() {
        let document = serde_json::json!({
            str_constants::PATHS: {},
            str_constants::COMPONENTS: { str_constants::SCHEMAS: { str_constants::TEST_OPENAPI_SCHEMA: {
                str_constants::DOLLAR_REF: str_constants::TEST_OPENAPI_MISSING_SCHEMA_REF
            }}}
        });
        assert!(matches!(
            super::validate_openapi_contract(&document, (&[][..]).into()),
            Err(super::OpenApiValidationError::MissingSchemaReference(_))
        ));
    }
}
