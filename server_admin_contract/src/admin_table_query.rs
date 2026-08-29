#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    #[serde(default)]
    #[param(value_type = u32)]
    offset: crate::admin_page_offset::AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: crate::admin_page_limit::AdminPageLimit,
    #[serde(default)]
    #[param(inline)]
    direction: crate::admin_sort_direction::AdminSortDirection,
}
impl AdminTableQuery {
    #[must_use]
    pub fn pagination(
        limit: crate::admin_page_limit::AdminPageLimit,
        offset: crate::admin_page_offset::AdminPageOffset,
    ) -> Self {
        Self {
            offset,
            limit,
            ..Self::default()
        }
    }
    #[must_use]
    pub const fn limit(&self) -> crate::admin_page_limit::AdminPageLimit {
        self.limit
    }
    #[must_use]
    pub const fn offset(&self) -> crate::admin_page_offset::AdminPageOffset {
        self.offset
    }
    #[must_use]
    pub const fn search(&self) -> &crate::admin_table_search::AdminTableSearch {
        &self.search
    }
    #[must_use]
    pub const fn sort(&self) -> &crate::admin_table_sort_key::AdminTableSortKey {
        &self.sort
    }
    #[must_use]
    pub const fn direction(&self) -> crate::admin_sort_direction::AdminSortDirection {
        self.direction
    }
}
