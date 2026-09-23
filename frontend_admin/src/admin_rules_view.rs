#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminRulesView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_data_table_view: server_admin_contract::admin_data_table_view::AdminDataTableView,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <crate::admin_data_grid::AdminDataGrid
            authenticated_admin=authenticated_admin
            admin_csr_query=admin_csr_query
            admin_data_table_view=admin_data_table_view
        />
    }
}
