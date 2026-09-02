#[derive(Debug, Clone, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum DeriveOrImpl {
    Derive,
    Impl(
        macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    ),
}
