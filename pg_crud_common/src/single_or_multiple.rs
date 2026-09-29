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
