use super::{
    AdminPageLimit, AdminPageOffset, AdminSortDirection, AdminTableSearch, AdminTableSortKey,
};

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
    search: AdminTableSearch,
    #[serde(default)]
    #[param(value_type = String, max_length = 32)]
    sort: AdminTableSortKey,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: AdminPageLimit,
    #[serde(default)]
    #[param(inline)]
    direction: AdminSortDirection,
}
impl AdminTableQuery {
    #[must_use]
    pub fn pagination(limit: AdminPageLimit, offset: AdminPageOffset) -> Self {
        Self {
            offset,
            limit,
            ..Self::default()
        }
    }
    #[must_use]
    pub const fn limit(&self) -> AdminPageLimit {
        self.limit
    }
    #[must_use]
    pub const fn offset(&self) -> AdminPageOffset {
        self.offset
    }
    #[must_use]
    pub const fn search(&self) -> &AdminTableSearch {
        &self.search
    }
    #[must_use]
    pub const fn sort(&self) -> &AdminTableSortKey {
        &self.sort
    }
    #[must_use]
    pub const fn direction(&self) -> AdminSortDirection {
        self.direction
    }
}
