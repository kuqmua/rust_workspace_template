#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminDataTableQuery {
    #[serde(flatten)]
    filter: crate::admin_data_table_filter_query::AdminDataTableFilterQuery,
    #[serde(flatten)]
    page: crate::admin_table_query::AdminTableQuery,
}
impl utoipa::IntoParams for AdminDataTableQuery {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        let parameter_in = parameter_in_provider();
        let mut parameters =
            <crate::admin_data_table_filter_query::AdminDataTableFilterQuery as utoipa::IntoParams>::into_params(|| {
                parameter_in.clone()
            });
        parameters.extend(
            <crate::admin_table_query::AdminTableQuery as utoipa::IntoParams>::into_params(|| {
                parameter_in.clone()
            }),
        );
        parameters
    }
}
