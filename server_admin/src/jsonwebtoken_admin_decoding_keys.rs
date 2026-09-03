#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
