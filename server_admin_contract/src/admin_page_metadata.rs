#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[constructor(pub(super))]
pub struct AdminPageMetadata {
    client_mode: crate::admin_page_client_mode::AdminPageClientMode,
    navigation: Option<crate::admin_page_navigation::AdminPageNavigation>,
}
