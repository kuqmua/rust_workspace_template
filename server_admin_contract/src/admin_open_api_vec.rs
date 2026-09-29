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
}
