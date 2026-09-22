#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AssignmentFormTarget {
    RoleRules(crate::role_rules_form::RoleRulesForm),
    UserRoles(crate::user_roles_form::UserRolesForm),
}
