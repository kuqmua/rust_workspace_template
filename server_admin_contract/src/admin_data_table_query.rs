use super::{AdminDataTableFilterQuery, AdminTableQuery};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminDataTableQuery {
    #[serde(flatten)]
    filter: AdminDataTableFilterQuery,
    #[serde(flatten)]
    page: AdminTableQuery,
}
impl utoipa::IntoParams for AdminDataTableQuery {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        let parameter_in = parameter_in_provider();
        let mut parameters =
            <AdminDataTableFilterQuery as utoipa::IntoParams>::into_params(|| parameter_in.clone());
        parameters.extend(<AdminTableQuery as utoipa::IntoParams>::into_params(|| {
            parameter_in.clone()
        }));
        parameters
    }
}
impl AdminDataTableQuery {
    #[must_use]
    pub const fn new(filter: AdminDataTableFilterQuery, page: AdminTableQuery) -> Self {
        Self { filter, page }
    }
    #[must_use]
    pub const fn filter(&self) -> &AdminDataTableFilterQuery {
        &self.filter
    }
    #[must_use]
    pub const fn page(&self) -> &AdminTableQuery {
        &self.page
    }
}
