#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[borrow]
pub struct SynBuiltGeneratePgTableInput(crate::generate_pg_table_model::GeneratePgTableModel);
