// This wrapper preserves explicit nested nulls when serde would otherwise skip a parent Option.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub struct ExplicitValue<T> {
    value: T,
}

impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for ExplicitValue<T> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::PG_CRUD_VALUES_FIELD,
                <T as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::PG_CRUD_VALUES_FIELD)
            .build()
            .into()
    }
}

impl<T: utoipa::ToSchema> utoipa::ToSchema for ExplicitValue<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::V)
    }
}
