#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_getters::Getters,
)]
pub struct AdminPageMetadata {
    client_mode: crate::admin_page_client_mode::AdminPageClientMode,
    navigation: Option<crate::admin_page_navigation::AdminPageNavigation>,
}
impl AdminPageMetadata {
    pub(super) const fn new(
        admin_page_client_mode: crate::admin_page_client_mode::AdminPageClientMode,
        option: Option<crate::admin_page_navigation::AdminPageNavigation>,
    ) -> Self {
        Self {
            client_mode: admin_page_client_mode,
            navigation: option,
        }
    }
}
