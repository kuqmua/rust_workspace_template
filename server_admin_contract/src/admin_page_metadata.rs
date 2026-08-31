#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor::Getters,
)]
pub struct AdminPageMetadata {
    client_mode: crate::admin_page_client_mode::AdminPageClientMode,
    navigation: Option<crate::admin_page_navigation::AdminPageNavigation>,
}
impl AdminPageMetadata {
    pub(super) const fn new(
        client_mode: crate::admin_page_client_mode::AdminPageClientMode,
        navigation: Option<crate::admin_page_navigation::AdminPageNavigation>,
    ) -> Self {
        Self {
            client_mode,
            navigation,
        }
    }
}
