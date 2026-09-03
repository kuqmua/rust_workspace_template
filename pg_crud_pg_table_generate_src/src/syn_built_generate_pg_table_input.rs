#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
    proc_macro_newtype::IntoInner,
)]
#[borrow]
pub struct SynBuiltGeneratePgTableInput(crate::generate_pg_table_model::GeneratePgTableModel);
