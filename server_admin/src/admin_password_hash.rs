#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    generate_accessor::Getters,
)]
pub struct AdminPasswordHash(pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret);
