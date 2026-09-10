#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_inner::AsRefInner,
)]
pub(crate) struct AdminRoleUpdateSlice<'updates>(
    &'updates [server_admin_contract::admin_role_update::AdminRoleUpdate],
);
impl<'updates> From<&'updates server_admin_contract::admin_role_updates::AdminRoleUpdates>
    for AdminRoleUpdateSlice<'updates>
{
    fn from(value: &'updates server_admin_contract::admin_role_updates::AdminRoleUpdates) -> Self {
        Self::from(value.as_ref())
    }
}
