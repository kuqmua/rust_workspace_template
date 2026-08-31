#[derive(
    Debug, serde::Serialize, serde::Deserialize, optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<crate::order::Order>,
}

impl<ColumnGeneric: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for OrderBy<ColumnGeneric> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
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
