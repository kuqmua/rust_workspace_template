#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_inner::AsRefInner,
)]
pub(crate) struct AdminUserUpdateSlice<'updates>(
    &'updates [server_admin_contract::admin_user_update::AdminUserUpdate],
);
