#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_new::New,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminTableQuery {
    #[serde(default)]
    #[param(value_type = String, max_length = 128)]
    search: crate::admin_table_search::AdminTableSearch,
    #[serde(default)]
    #[param(value_type = String, max_length = 32)]
    sort: crate::admin_table_sort_key::AdminTableSortKey,
    #[getters(copy)]
    #[serde(default)]
    #[param(value_type = u32)]
    offset: crate::admin_page_offset::AdminPageOffset,
    #[getters(copy)]
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: crate::admin_page_limit::AdminPageLimit,
    #[getters(copy)]
    #[serde(default)]
    #[param(inline)]
    direction: crate::admin_sort_direction::AdminSortDirection,
}
impl AdminTableQuery {
    #[must_use]
    pub fn pagination(
        admin_page_limit: crate::admin_page_limit::AdminPageLimit,
        admin_page_offset: crate::admin_page_offset::AdminPageOffset,
    ) -> Self {
        Self {
            offset: admin_page_offset,
            limit: admin_page_limit,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_table_pagination_constructor_preserves_bounds_and_query_defaults() {
        assert!(
            [(1u16, 0u32), (100u16, u32::MAX), (7u16, 11u32)]
                .into_iter()
                .all(|(limit, offset)| {
                    let pagination_values = (
                        crate::admin_page_limit::AdminPageLimit::try_from(limit),
                        crate::admin_page_offset::AdminPageOffset::from(offset),
                    );
                    let (Ok(admin_page_limit), admin_page_offset) = pagination_values else {
                        return false;
                    };
                    let query =
                        super::AdminTableQuery::pagination(admin_page_limit, admin_page_offset);
                    serde_json::to_value(&query).is_ok_and(|wire| wire == serde_json::json!({
                (stringify!(search)): constants_str::EMPTY,
                (stringify!(sort)): constants_str::EMPTY,
                (stringify!(offset)): offset,
                (stringify!(limit)): limit,
                (stringify!(direction)): crate::admin_sort_direction::AdminSortDirection::Ascending
            })) && query.offset() == admin_page_offset && query.limit() == admin_page_limit
                })
        );
    }
}
