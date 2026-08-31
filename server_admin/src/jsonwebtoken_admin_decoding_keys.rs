#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::AsRefTarget,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub(crate) struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
