#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
