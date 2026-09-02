#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
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
