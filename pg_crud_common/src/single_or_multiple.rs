#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum SingleOrMultiple<T: std::fmt::Debug + PartialEq + Clone> {
    Multiple(crate::not_empty_unique_vec::NotEmptyUniqueVec<T>),
    Single(T),
}

impl<T> utoipa::PartialSchema for SingleOrMultiple<T>
where
    T: std::fmt::Debug + PartialEq + Clone + utoipa::PartialSchema,
{
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::Schema::from(
            utoipa::openapi::OneOfBuilder::new()
                .item(
                    utoipa::openapi::ObjectBuilder::new()
                        .property(
                            stringify!(Multiple),
                            <crate::not_empty_unique_vec::NotEmptyUniqueVec<T> as utoipa::PartialSchema>::schema(),
                        )
                        .required(stringify!(Multiple)),
                )
                .item(
                    utoipa::openapi::ObjectBuilder::new()
                        .property(stringify!(Single), <T as utoipa::PartialSchema>::schema())
                        .required(stringify!(Single)),
                )
                .build(),
        )
        .into()
    }
}

impl<T> utoipa::ToSchema for SingleOrMultiple<T>
where
    T: std::fmt::Debug + PartialEq + Clone + utoipa::ToSchema,
{
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(
            bounded_types::utoipa_schema_type_name::UtoipaSchemaTypeName::for_type::<Self>()
                .into_inner(),
        )
    }

    fn schemas(
        vec: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        bounded_types::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(vec)
            .register_item_schema::<crate::not_empty_unique_vec::NotEmptyUniqueVec<T>>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_single_or_multiple_schema_preserves_tagged_alternatives_and_array_bounds() {
        let schema = <crate::single_or_multiple::SingleOrMultiple<crate::operator::Operator> as utoipa::PartialSchema>::schema();
        assert!(
            matches!(schema, utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::OneOf(alternatives)) if {
                alternatives.items.len() == 2
                    && alternatives.items.iter().zip([stringify!(Multiple), stringify!(Single)]).all(|(alternative, name)| {
                        match alternative {
                            utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object)) => {
                                object.properties.len() == 1
                                    && object.required == [String::from(name)]
                                    && object.properties.get(name).is_some_and(|property| {
                                        let item_schema = <crate::operator::Operator as utoipa::PartialSchema>::schema();
                                        if name == stringify!(Single) {
                                            *property == item_schema
                                        } else {
                                            matches!(property, utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Array(array)) if {
                                                array.min_items == Some(1usize)
                                                    && array.max_items == Some(crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN)
                                                    && matches!(&array.items, utoipa::openapi::schema::ArrayItems::RefOrSchema(items) if **items == item_schema)
                                            })
                                        }
                                    })
                            }
                        utoipa::openapi::RefOr::Ref(_) | utoipa::openapi::RefOr::T(_) => false,
                        }
                    })
            })
        );
    }

    #[test]
    fn test_single_or_multiple_json_preserves_variants_and_collection_validation() {
        let multiple = crate::not_empty_unique_vec::NotEmptyUniqueVec::try_from(
            crate::duplicate_candidates::DuplicateCandidates::from(vec![
                crate::operator::Operator::And,
                crate::operator::Operator::Or,
            ]),
        );
        assert!(matches!(&multiple, Ok(values) if values.as_slice().len() == 2));
        assert!(std::iter::once(crate::single_or_multiple::SingleOrMultiple::Single(
            crate::operator::Operator::And,
        ))
        .chain(multiple.into_iter().map(crate::single_or_multiple::SingleOrMultiple::Multiple))
        .all(|value| {
            let expected = match &value {
                crate::single_or_multiple::SingleOrMultiple::Single(operator) => {
                    serde_json::json!({(stringify!(Single)): operator})
                }
                crate::single_or_multiple::SingleOrMultiple::Multiple(values) => {
                    serde_json::json!({(stringify!(Multiple)): values.as_slice()})
                }
            };
            matches!(serde_json::to_value(&value), Ok(json) if json == expected)
                && matches!(
                    serde_json::from_value::<crate::single_or_multiple::SingleOrMultiple<crate::operator::Operator>>(expected),
                    Ok(decoded) if decoded == value
                )
        }));
        assert!(
            [
                Vec::new(),
                vec![
                    crate::operator::Operator::And,
                    crate::operator::Operator::And
                ],
            ]
            .into_iter()
            .all(|operators| {
                serde_json::from_value::<
                    crate::single_or_multiple::SingleOrMultiple<crate::operator::Operator>,
                >(serde_json::json!({(stringify!(Multiple)): operators}))
                .is_err()
            })
        );
    }

    #[test]
    fn test_openapi_schema_registers_single_or_multiple_item() {
        let mut schemas = Vec::new();
        <crate::single_or_multiple::SingleOrMultiple<crate::pagination_base::PaginationBase> as utoipa::ToSchema>::schemas(
            &mut schemas,
        );
        assert!(schemas.iter().any(|(name, _schema)| {
            name == <crate::pagination_base::PaginationBase as utoipa::ToSchema>::name().as_ref()
        }));
        assert!(schemas.iter().any(|(name, _schema)| {
            name == <crate::not_empty_unique_vec::NotEmptyUniqueVec<
                crate::pagination_base::PaginationBase,
            > as utoipa::ToSchema>::name()
            .as_ref()
        }));
    }
}
