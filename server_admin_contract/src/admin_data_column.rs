#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminDataColumn {
    #[getters(skip)]
    #[constructor(order = 0)]
    filters: crate::admin_data_filters::AdminDataFilters,
    #[constructor(order = 2)]
    label: crate::admin_text::AdminText,
    #[constructor(order = 3)]
    name: crate::admin_text::AdminText,
    #[getters(copy)]
    #[constructor(order = 1)]
    input_kind: frontend_contract::input_kind::InputKind,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn filters(&self) -> &[crate::admin_data_filter::AdminDataFilter] {
        self.filters.as_slice()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_column_filter_accessor_preserves_empty_ordered_and_duplicate_filters() {
        let equal = crate::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::Eq,
        );
        let between = crate::admin_data_filter::AdminDataFilter::from(
            frontend_contract::filter_operation::FilterOperation::Between,
        );
        assert!(
            [Vec::new(), vec![equal, between, equal]]
                .into_iter()
                .all(|filters| {
                    let expected = serde_json::to_value(&filters);
                    let column_result =
                        serde_json::from_value::<super::AdminDataColumn>(serde_json::json!({
                            (stringify!(filters)): filters,
                            (stringify!(label)): constants_str::ADMIN,
                            (stringify!(name)): constants_str::LOGIN,
                            (stringify!(input_kind)): frontend_contract::input_kind::InputKind::Text
                        }));
                    column_result.is_ok_and(|column| {
                        column.filters() == filters
                            && serde_json::to_value(column.filters()).is_ok_and(|wire| {
                                expected.is_ok_and(|expected_wire| wire == expected_wire)
                            })
                    })
                })
        );
    }
}
