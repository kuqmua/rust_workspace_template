#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
    #[getters(skip)]
    items: crate::admin_session_views::AdminSessionViews,
    #[getters(copy)]
    #[schema(value_type = u64)]
    total: crate::admin_page_total::AdminPageTotal,
}
impl AdminSessionsPage {
    #[must_use]
    pub const fn items(&self) -> &[super::admin_session_view::AdminSessionView] {
        self.items.as_slice()
    }
}
