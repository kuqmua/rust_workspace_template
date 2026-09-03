#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
