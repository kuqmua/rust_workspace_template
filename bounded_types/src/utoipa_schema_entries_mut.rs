#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype_from_inner::FromInner,
)]
pub struct UtoipaSchemaEntriesMut<'schema>(
    &'schema mut Vec<(
        String,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    )>,
);

impl std::fmt::Debug for UtoipaSchemaEntriesMut<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

impl UtoipaSchemaEntriesMut<'_> {
    pub fn register_item_schema<Item>(self)
    where
        Item: utoipa::ToSchema,
    {
        self.0.push((
            Item::name().into_owned(),
            <Item as utoipa::PartialSchema>::schema(),
        ));
        Item::schemas(self.0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register_item_schema_includes_the_item_component() {
        let mut schemas = Vec::new();
        crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas)
            .register_item_schema::<u8>();
        assert!(
            schemas
                .iter()
                .any(|(name, _schema)| { name == <u8 as utoipa::ToSchema>::name().as_ref() })
        );
    }
}
