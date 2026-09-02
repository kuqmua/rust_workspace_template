#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugRedacted,
    proc_macro_newtype::FromInner,
    proc_macro_getters::Getters,
)]
pub struct AdminPasswordHash(pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret);
