#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_deref_mut_inner::DerefMutInner,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_getters::Getters,
)]
pub(crate) struct SqlxAdminTransaction<'transaction_lt>(
    sqlx::Transaction<'transaction_lt, sqlx::Postgres>,
);
