#[derive(
    Debug,
    proc_macro_getters::Getters,
    proc_macro_new::New,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct OrderBy<ColumnGeneric> {
    column: ColumnGeneric,
    order: Option<crate::order::Order>,
}

impl<ColumnGeneric: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for OrderBy<ColumnGeneric> {
    #[allow(
        unused_variables,
        reason = "the schema trait implementation preserves the type-based parameter name"
    )]
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                constants_str::COLUMN,
                <ColumnGeneric as utoipa::PartialSchema>::schema(),
            )
            .property(
                constants_str::ORDER,
                <crate::order::Order as utoipa::PartialSchema>::schema(),
            )
            .required(constants_str::COLUMN)
            .build()
            .into()
    }
}

impl<ColumnGeneric: utoipa::ToSchema> utoipa::ToSchema for OrderBy<ColumnGeneric> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::ORDERBY)
    }
}
