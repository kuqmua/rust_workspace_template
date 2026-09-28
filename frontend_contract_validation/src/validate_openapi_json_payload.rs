pub fn validate_openapi_json_payload<Payload, Schema, Document>(
    payload: &Payload,
    schema: &Schema,
    document: &Document,
) -> Result<(), crate::open_api_payload_validation_error::OpenApiPayloadValidationError>
where
    Payload: std::borrow::Borrow<serde_json::Value>,
    Schema: std::borrow::Borrow<serde_json::Value>,
    Document: std::borrow::Borrow<serde_json::Value>,
{
    let payload_value = <Payload as std::borrow::Borrow<serde_json::Value>>::borrow(payload);
    let schema_value = <Schema as std::borrow::Borrow<serde_json::Value>>::borrow(schema);
    let document_value = <Document as std::borrow::Borrow<serde_json::Value>>::borrow(document);
    let mut comparison_pairs = Vec::new();
    let mut equal_json = |left_value, right_value| {
        comparison_pairs.clear();
        comparison_pairs.push((
            std::borrow::Borrow::<serde_json::Value>::borrow(left_value),
            std::borrow::Borrow::<serde_json::Value>::borrow(right_value),
        ));
        'comparison: {
            while let Some((left_json, right_json)) = comparison_pairs.pop() {
                match (left_json, right_json) {
                    (
                        serde_json::Value::Array(left_items),
                        serde_json::Value::Array(right_items),
                    ) => {
                        if left_items.len() != right_items.len() {
                            break 'comparison false;
                        }
                        comparison_pairs.extend(left_items.iter().zip(right_items));
                    }
                    (
                        serde_json::Value::Object(left_fields),
                        serde_json::Value::Object(right_fields),
                    ) => {
                        if left_fields.len() != right_fields.len()
                            || !left_fields.iter().all(|(field, left_field)| {
                                right_fields.get(field).is_some_and(|right_field| {
                                    comparison_pairs.push((left_field, right_field));
                                    true
                                })
                            })
                        {
                            break 'comparison false;
                        }
                    }
                    _ => {
                        if left_json != right_json {
                            break 'comparison false;
                        }
                    }
                }
            }
            true
        }
    };
    let create_frame = |payload_value_ref, schema_value_ref| {
        (
            payload_value_ref,
            schema_value_ref,
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Resolve,
            constants_usize::ZERO,
            constants_usize::ZERO,
            constants_usize::ZERO,
            None::<serde_json::map::Iter<'_>>,
        )
    };
    let mismatch = |open_api_schema_mismatch| {
        crate::open_api_payload_validation_error::OpenApiPayloadValidationError::Mismatch(
            open_api_schema_mismatch,
        )
    };
    let mut frames = vec![create_frame(payload_value, schema_value)];
    let mut active_references = std::collections::BTreeSet::new();
    let mut reference_history = Vec::new();
    let mut outcome = Ok(());
    while let Some((
        payload_value_ref,
        schema_value_ref,
        stage,
        index,
        matches,
        reference_base,
        fields,
    )) = frames.last_mut()
    {
        match *stage {
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Resolve => {
                outcome = Ok(());
                *reference_base = reference_history.len();
                while let Some(reference) = schema_value_ref.get(constants_str::DOLLAR_REF).and_then(serde_json::Value::as_str) {
                    let key = (crate::open_api_schema_reference_ref::OpenApiSchemaReferenceRef::from(reference), std::ptr::from_ref(*payload_value_ref));
                    if !active_references.insert(key) {
                        outcome = Err(mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::ReferenceCycle));
                        break;
                    }
                    reference_history.push(key);
                    if let Some(resolved) = reference.strip_prefix(constants_str::COMPONENTS_SCHEMAS).and_then(|name| {
                        document_value.pointer(constants_str::COMPONENTS_SCHEMAS_ALT)?.get(name)
                    }) {
                        *schema_value_ref = resolved;
                    } else {
                        outcome = Err(mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::MissingReference));
                        break;
                    }
                }
                *stage = if outcome.is_err() || (payload_value_ref.is_null() && schema_value_ref.get(constants_str::NULLABLE).and_then(serde_json::Value::as_bool).unwrap_or(false)) {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AllOf
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AllOf => {
                if let Some(candidate) = schema_value_ref.get(constants_str::ALL_OF).and_then(serde_json::Value::as_array).and_then(|schemas| schemas.get(*index)) {
                    *index = index.saturating_add(constants_usize::ONE);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AllOfChild;
                    let frame = create_frame(*payload_value_ref, candidate);
                    frames.push(frame);
                } else {
                    *index = constants_usize::ZERO;
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::OneOf;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AllOfChild => {
                *stage = if outcome.is_err() {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AllOf
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::OneOf => {
                let candidates = schema_value_ref.get(constants_str::ONE_OF).and_then(serde_json::Value::as_array);
                if let Some(candidate) = candidates.and_then(|schemas| schemas.get(*index)) {
                    *index = index.saturating_add(constants_usize::ONE);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::OneOfChild;
                    let frame = create_frame(*payload_value_ref, candidate);
                    frames.push(frame);
                } else if candidates.is_some() && *matches != constants_usize::ONE {
                    outcome = Err(mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::OneOf));
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete;
                } else {
                    outcome = Ok(());
                    *index = constants_usize::ZERO;
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AnyOf;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::OneOfChild => {
                if outcome.is_ok() {
                    *matches = matches.saturating_add(constants_usize::ONE);
                }
                *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::OneOf;
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AnyOf => {
                let candidates = schema_value_ref.get(constants_str::ANY_OF).and_then(serde_json::Value::as_array);
                if let Some(candidate) = candidates.and_then(|schemas| schemas.get(*index)) {
                    *index = index.saturating_add(constants_usize::ONE);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AnyOfChild;
                    let frame = create_frame(*payload_value_ref, candidate);
                    frames.push(frame);
                } else if candidates.is_some() {
                    outcome = Err(mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AnyOf));
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete;
                } else {
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::CheckValue;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AnyOfChild => {
                *stage = if outcome.is_ok() {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::CheckValue
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AnyOf
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::CheckValue => {
                outcome = 'value_check: {
                    if let Some(expected_type) = schema_value_ref
                        .get(constants_str::JSON_TYPE)
                        .and_then(serde_json::Value::as_str)
                    {
                        let type_matches = match expected_type {
                            constants_str::ARRAY => payload_value_ref.is_array(),
                            constants_str::BOOLEAN => payload_value_ref.is_boolean(),
                            constants_str::INTEGER => {
                                payload_value_ref.as_i64().is_some() || payload_value_ref.as_u64().is_some()
                            }
                            constants_str::JSON_NULL => payload_value_ref.is_null(),
                            constants_str::NUMBER => payload_value_ref.is_number(),
                            constants_str::OBJECT => payload_value_ref.is_object(),
                            constants_str::STRING => payload_value_ref.is_string(),
                            _ => false,
                        };
                            if !type_matches {
                                break 'value_check Err(
                                    mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Type),
                                );
                            }
                        }
                        if schema_value_ref
                            .get(constants_str::ENUM)
                            .and_then(serde_json::Value::as_array)
                            .is_some_and(|values| !values.iter().any(|enum_value| equal_json(*payload_value_ref, enum_value)))
                        {
                            break 'value_check Err(
                                mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Enum),
                            );
                        }
                        if schema_value_ref
                            .get(constants_str::CONST)
                            .is_some_and(|value| !equal_json(value, *payload_value_ref))
                        {
                            break 'value_check Err(
                                mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::Const),
                            );
                        }
                        if let Some(object) = payload_value_ref.as_object() {
                            let properties = schema_value_ref
                                .get(constants_str::PROPERTIES)
                                .and_then(serde_json::Value::as_object);
                            if let Some(required) = schema_value_ref
                                .get(constants_str::REQUIRED)
                                .and_then(serde_json::Value::as_array)
                                && required.iter().any(|field| {
                                    field
                                        .as_str()
                                        .is_some_and(|field_name| !object.contains_key(field_name))
                                })
                            {
                                break 'value_check Err(
                                    mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::RequiredProperty),
                                );
                            }
                            if schema_value_ref
                                .get(constants_str::ADDITIONAL_PROPERTIES)
                                .and_then(serde_json::Value::as_bool)
                                == Some(false)
                                && object.keys().any(|field| {
                                    properties.is_none_or(|defined_properties| !defined_properties.contains_key(field))
                                })
                            {
                                break 'value_check Err(
                                    mismatch(crate::open_api_schema_mismatch::OpenApiSchemaMismatch::AdditionalProperty),
                                );
                            }
                        }
                        Ok(())
                };
                if outcome.is_err() {
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete;
                } else {
                    *fields = schema_value_ref.get(constants_str::PROPERTIES).and_then(serde_json::Value::as_object).map(serde_json::Map::iter);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Properties;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Properties => {
                let child = fields.as_mut().and_then(|property_entries| property_entries.find_map(|(field, field_schema)| payload_value_ref.get(field).map(|field_value| (field_value, field_schema))));
                if let Some((field_value, field_schema)) = child {
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::PropertyChild;
                    let frame = create_frame(field_value, field_schema);
                    frames.push(frame);
                } else {
                    *fields = payload_value_ref.as_object().map(serde_json::Map::iter);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AdditionalProperties;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::PropertyChild => {
                *stage = if outcome.is_err() {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Properties
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AdditionalProperties => {
                let additional_schema = schema_value_ref.get(constants_str::ADDITIONAL_PROPERTIES).filter(|additional_value| additional_value.is_object());
                let properties = schema_value_ref.get(constants_str::PROPERTIES).and_then(serde_json::Value::as_object);
                let child = additional_schema.and_then(|additional_value| {
                    let additional_fields = fields.as_mut()?;
                    additional_fields.find_map(|(field, field_value)| {
                        properties.is_none_or(|defined| !defined.contains_key(field)).then_some((field_value, additional_value))
                    })
                });
                if let Some((field_value, field_schema)) = child {
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AdditionalPropertyChild;
                    let frame = create_frame(field_value, field_schema);
                    frames.push(frame);
                } else {
                    *index = constants_usize::ZERO;
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Items;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AdditionalPropertyChild => {
                *stage = if outcome.is_err() {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::AdditionalProperties
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Items => {
                let child = schema_value_ref.get(constants_str::ITEMS).and_then(|items_schema| payload_value_ref.as_array().and_then(|items| items.get(*index)).map(|item| (item, items_schema)));
                if let Some((item, items_schema)) = child {
                    *index = index.saturating_add(constants_usize::ONE);
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::ItemChild;
                    let frame = create_frame(item, items_schema);
                    frames.push(frame);
                } else {
                    *stage = crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete;
                }
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::ItemChild => {
                *stage = if outcome.is_err() {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete
                } else {
                    crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Items
                };
            }
            crate::open_api_payload_validation_stage::OpenApiPayloadValidationStage::Complete => {
                while reference_history.len() > *reference_base {
                    if let Some(key) = reference_history.pop() {
                        let _: bool = active_references.remove(&key);
                    }
                }
                let _completed_frame = frames.pop();
            }
        }
    }
    outcome
}
