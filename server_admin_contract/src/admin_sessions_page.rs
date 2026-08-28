#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
pub struct AdminSessionsPage {
    items: super::super::AdminSessionViews,
    #[schema(value_type = u64)]
    total: super::super::AdminPageTotal,
}
impl AdminSessionsPage {
    #[must_use]
    pub const fn items(&self) -> &[super::admin_session_view::AdminSessionView] {
        self.items.as_slice()
    }
    #[must_use]
    pub const fn total(&self) -> super::super::AdminPageTotal {
        self.total
    }
}
