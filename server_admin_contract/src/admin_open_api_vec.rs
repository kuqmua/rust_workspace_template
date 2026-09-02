#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(dead_code)] // schema-only generic carries its item type without runtime construction
pub(crate) struct AdminOpenApiVec<T, const MAX: usize> {
    marker: crate::admin_open_api_vec_phantom_data::AdminOpenApiVecPhantomData<T>,
}
impl<T: utoipa::PartialSchema, const MAX: usize> utoipa::__dev::ComposeSchema
    for AdminOpenApiVec<T, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .max_items(Some(MAX))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema, const MAX: usize> utoipa::ToSchema for AdminOpenApiVec<T, MAX> {
    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        schemas.push((
            T::name().into_owned(),
            <T as utoipa::PartialSchema>::schema(),
        ));
        T::schemas(schemas);
    }
}
