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
)]
pub struct V<T> {
    pub v: T,
}

impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for V<T> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::catalog::PG_CRUD_V_FIELD,
                <T as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::catalog::PG_CRUD_V_FIELD)
            .build()
            .into()
    }
}

impl<T: utoipa::ToSchema> utoipa::ToSchema for V<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::catalog::V)
    }
}
