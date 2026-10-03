#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    dead_code,
    reason = "admin open api vec declares fixture or generated API members exercised outside ordinary reachability analysis"
)]
pub(crate) struct AdminOpenApiVec<T, const MAX: usize> {
    marker: crate::admin_open_api_vec_phantom_data::AdminOpenApiVecPhantomData<T>,
}
impl<T: utoipa::PartialSchema, const MAX: usize> utoipa::__dev::ComposeSchema
    for AdminOpenApiVec<T, MAX>
{
    fn compose(
        vec: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        drop(vec);
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .max_items(Some(MAX))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema, const MAX: usize> utoipa::ToSchema for AdminOpenApiVec<T, MAX> {
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
            .register_item_schema::<T>();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_admin_open_api_vec_schema_names_distinguish_bounds() {
        assert_ne!(
            <super::AdminOpenApiVec<u8, 2> as utoipa::ToSchema>::name(),
            <super::AdminOpenApiVec<u8, 3> as utoipa::ToSchema>::name()
        );
    }

    #[test]
    fn test_admin_open_api_vec_schema_names_distinguish_item_types() {
        assert_ne!(
            <super::AdminOpenApiVec<u8, 2> as utoipa::ToSchema>::name(),
            <super::AdminOpenApiVec<u16, 2> as utoipa::ToSchema>::name()
        );
    }

    #[test]
    fn test_admin_open_api_vec_composes_exact_bounds_and_item_schema() {
        fn schema_matches<const MAX: usize>() -> crate::admin_bool::AdminBool {
            let item = <crate::admin_optional_setting::AdminOptionalSetting as utoipa::PartialSchema>::schema();
            let composed = <super::AdminOpenApiVec<
                crate::admin_optional_setting::AdminOptionalSetting,
                MAX,
            > as utoipa::__dev::ComposeSchema>::compose(vec![
                <crate::admin_bool::AdminBool as utoipa::PartialSchema>::schema(),
            ]);
            crate::admin_bool::AdminBool::from(serde_json::to_value(composed).is_ok_and(|wire| {
                serde_json::to_value(item).is_ok_and(|item_wire| wire == serde_json::json!({(stringify!(type)): stringify!(array), (stringify!(items)): item_wire, (stringify!(maxItems)): MAX}))
            }))
        }
        assert_eq!(
            schema_matches::<0usize>(),
            crate::admin_bool::AdminBool::from(true)
        );
        assert_eq!(
            schema_matches::<6usize>(),
            crate::admin_bool::AdminBool::from(true)
        );
        assert_eq!(
            schema_matches::<10_000usize>(),
            crate::admin_bool::AdminBool::from(true)
        );
    }

    #[test]
    fn test_admin_open_api_vec_registers_item_and_preserves_existing_components() {
        let existing = <crate::admin_bool::AdminBool as utoipa::PartialSchema>::schema();
        let existing_wire = serde_json::to_value(&existing);
        assert!(existing_wire.is_ok());
        let Ok(expected_existing) = existing_wire else {
            return;
        };
        let mut schemas = vec![(constants_str::X.to_owned(), existing)];
        <super::AdminOpenApiVec<crate::admin_optional_setting::AdminOptionalSetting, 6usize> as utoipa::ToSchema>::schemas(&mut schemas);
        assert_eq!(schemas.len(), 2usize);
        assert!(
            schemas
                .first()
                .is_some_and(|(name, schema)| name == constants_str::X
                    && serde_json::to_value(schema).is_ok_and(|wire| wire == expected_existing))
        );
        assert!(schemas.get(1usize).is_some_and(|(name, schema)| {
            name == <crate::admin_optional_setting::AdminOptionalSetting as utoipa::ToSchema>::name().as_ref()
                && serde_json::to_value(schema).is_ok_and(|wire| serde_json::to_value(<crate::admin_optional_setting::AdminOptionalSetting as utoipa::PartialSchema>::schema()).is_ok_and(|item_wire| wire == item_wire))
        }));
    }
}
