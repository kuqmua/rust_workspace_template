#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum ConfirmedDeleteTarget {
    Role(crate::role_id_form::RoleIdForm),
    User(crate::user_id_form::UserIdForm),
}
