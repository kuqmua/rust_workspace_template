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

#[cfg(test)]
mod tests {
    #[test]
    fn test_data_table_query_parameters_preserve_component_catalogs_and_provider_calls() {
        let locations = [
            None,
            Some(utoipa::openapi::path::ParameterIn::Query),
            Some(utoipa::openapi::path::ParameterIn::Header),
            Some(utoipa::openapi::path::ParameterIn::Path),
            Some(utoipa::openapi::path::ParameterIn::Cookie),
        ];
        assert!(locations.into_iter().all(|parameter_in| {
            let calls = std::cell::Cell::new(0usize);
            let actual = <super::AdminDataTableQuery as utoipa::IntoParams>::into_params(|| {
                calls.set(calls.get() + constants_usize::ONE);
                parameter_in.clone()
            });
            let expected = <crate::admin_data_table_filter_query::AdminDataTableFilterQuery as utoipa::IntoParams>::into_params(|| parameter_in.clone())
                .into_iter().chain(<crate::admin_table_query::AdminTableQuery as utoipa::IntoParams>::into_params(|| parameter_in.clone())).collect::<Vec<_>>();
            let names = [stringify!(filter_field), stringify!(filter_value), stringify!(filter_end), stringify!(filter_operation), stringify!(search), stringify!(sort), stringify!(offset), stringify!(limit), stringify!(direction)];
            calls.get() == constants_usize::ONE
                && actual.len() == names.len()
                && serde_json::to_value(actual).is_ok_and(|wire| {
                    serde_json::to_value(expected).is_ok_and(|expected_wire| wire == expected_wire)
                        && wire.as_array().is_some_and(|parameters| parameters.iter().zip(names).all(|(parameter, name)| parameter.get(stringify!(name)).and_then(serde_json::Value::as_str) == Some(name)))
                })
        }));
    }
}
