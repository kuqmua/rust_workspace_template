#[derive(Debug, Clone, optimal_memory_layout::OptimalMemoryLayout)]
pub enum DeriveOrImpl {
    Derive,
    Impl(macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream),
}
