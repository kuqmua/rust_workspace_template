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
    fn test_schema_entries_debug_hides_components_in_both_formats() {
        let render = |utoipa_schema_entries_mut: crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut<'_>| {
            [format!("{utoipa_schema_entries_mut:?}"), format!("{utoipa_schema_entries_mut:#?}")]
        };
        let mut schemas = Vec::new();
        let empty_debug =
            render(crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas));
        crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas)
            .register_item_schema::<u8>();
        let populated_debug =
            render(crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas));
        assert_eq!(populated_debug, empty_debug);
        assert!(
            populated_debug
                .iter()
                .all(|debug| debug.starts_with(std::any::type_name::<
                    crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut<'_>,
                >()))
        );
        assert_eq!(schemas.len(), 1usize);
        assert!(schemas.first().is_some_and(|(name, schema)| {
            name == <u8 as utoipa::ToSchema>::name().as_ref()
                && schema == &<u8 as utoipa::PartialSchema>::schema()
        }));
    }

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

    #[test]
    fn test_register_nested_item_schema_preserves_existing_entries_and_dependencies() {
        let mut schemas = Vec::new();
        crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas)
            .register_item_schema::<u8>();
        crate::utoipa_schema_entries_mut::UtoipaSchemaEntriesMut::from(&mut schemas)
            .register_item_schema::<crate::bounded_vec::BoundedVec<
                crate::bounded_string::BoundedString<1, 8, false>,
                0,
                3,
            >>();
        let names =
            [
                <u8 as utoipa::ToSchema>::name(),
                <crate::bounded_vec::BoundedVec<
                    crate::bounded_string::BoundedString<1, 8, false>,
                    0,
                    3,
                > as utoipa::ToSchema>::name(),
                <crate::bounded_string::BoundedString<1, 8, false> as utoipa::ToSchema>::name(),
            ];
        assert!(
            schemas
                .iter()
                .map(|(name, _schema)| name.as_str())
                .eq(names.iter().map(AsRef::<str>::as_ref))
        );
        assert!(
            schemas.first().map(|(_name, schema)| schema)
                == Some(&<u8 as utoipa::PartialSchema>::schema())
        );
        assert!(
            schemas.get(1).map(|(_name, schema)| schema)
                == Some(&<crate::bounded_vec::BoundedVec<
                    crate::bounded_string::BoundedString<1, 8, false>,
                    0,
                    3,
                > as utoipa::PartialSchema>::schema())
        );
        assert!(schemas.get(2).map(|(_name, schema)| schema) == Some(&<crate::bounded_string::BoundedString<1, 8, false> as utoipa::PartialSchema>::schema()));
    }
}
