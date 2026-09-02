#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
