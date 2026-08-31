#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
