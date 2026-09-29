#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct ExplicitValue<T> {
    value: T,
}

impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for ExplicitValue<T> {
    #[allow(
        unused_variables,
        reason = "the schema trait implementation preserves the type-based parameter name"
    )]
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(stringify!(value), <T as utoipa::PartialSchema>::schema())
            .required(stringify!(value))
            .build()
            .into()
    }
}

impl<T: utoipa::ToSchema> utoipa::ToSchema for ExplicitValue<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Owned(
            bounded_types::utoipa_schema_type_name::UtoipaSchemaTypeName::for_type::<Self>()
                .into_inner(),
        )
    }

    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        bounded_types::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(schemas)
            .register_item_schema::<T>();
    }
}
