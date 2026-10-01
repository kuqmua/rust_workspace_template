#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminDataGrid(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
    admin_data_table_view: server_admin_contract::admin_data_table_view::AdminDataTableView,
    admin_frontend_path: Option<server_admin_contract::admin_frontend_path::AdminFrontendPath>,
) -> impl leptos::prelude::IntoView {
    let is_sessions = admin_frontend_path
        == Some(server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions);
    let is_roles = admin_data_table_view.table()
        == server_admin_contract::admin_data_table::AdminDataTable::Roles;
    let is_users = admin_data_table_view.table()
        == server_admin_contract::admin_data_table::AdminDataTable::Users;
    let can_update = (is_roles
        && bool::from(
            authenticated_admin.has_rule(server_admin_contract::admin_rule::AdminRule::RolesUpdate),
        ))
        || (is_users
            && bool::from(
                authenticated_admin
                    .has_rule(server_admin_contract::admin_rule::AdminRule::UsersUpdate),
            ));
    let table_path = admin_frontend_path.map_or_else(
        || admin_data_table_view.table().frontend_path(),
        server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath::from,
    );
    let total = admin_data_table_view.total();
    let grid = crate::admin_data_table_grid::admin_data_table_grid(
        &admin_data_table_view,
        admin_csr_query.filter_field(),
        admin_csr_query.filter_operation(),
        admin_csr_query.filter_value(),
        admin_csr_query.filter_end(),
        admin_csr_query.limit(),
        Some(&admin_csr_query),
        &table_path,
        is_sessions,
        can_update,
    );
    let pagination = crate::admin_pagination::admin_pagination(
        &table_path,
        &crate::admin_pagination_query::AdminPaginationQuery::Csr {
            admin_csr_query: &admin_csr_query,
            admin_data_table: admin_data_table_view.table(),
            admin_frontend_path,
        },
        total,
    );
    let create_user = (is_users
        && bool::from(authenticated_admin.has_rule(
            server_admin_contract::admin_rule::AdminRule::UsersCreate,
        )))
    .then(|| {
        leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()>{constants_str::PG_CRUD_CREATE_RULE_ACTION}</crate::admin_button_link::AdminButtonLink> }
    });
    let create_role = (is_roles
        && bool::from(authenticated_admin.has_rule(
            server_admin_contract::admin_rule::AdminRule::RolesCreate,
        )))
    .then(|| {
        leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()>{constants_str::PG_CRUD_CREATE_RULE_ACTION}</crate::admin_button_link::AdminButtonLink> }
    });
    let resource_actions = (is_users || is_roles).then(|| {
        leptos::view! {
            <div class="resource-actions">{create_user}{create_role}</div>
        }
    });
    let revoke_all = is_sessions.then(|| {
        leptos::view! {
            <div class="resource-actions">
                <crate::admin_alert_dialog::AdminAlertDialog
                    string=String::from(constants_str::ADMIN_REVOKE_ALL_SESSIONS_DIALOG)
                    title=constants_str::ADMIN_UI_REVOKE_ALL_SESSIONS
                    description=constants_str::ADMIN_UI_END_ALL_SESSIONS
                    trigger=constants_str::ADMIN_BUTTON_REVOKE_ALL_SESSIONS
                    confirm=constants_str::ADMIN_BUTTON_REVOKE_ALL_SESSIONS
                    callback=leptos::prelude::Callback::new(move |()| {
                        match crate::admin_api_url::admin_api_url(server_admin_contract::admin_route::AdminRoute::RevokeAllSessions) {
                            Ok(path) => crate::reload_after::reload_after(
                                crate::admin_mutation_method::AdminMutationMethod::Delete,
                                path,
                                server_admin_contract::admin_no_body::AdminNoBody,
                            ),
                            Err(error) => crate::show_mutation_error::show_mutation_error(&error),
                        }
                    })
                />
            </div>
        }
    });
    leptos::view! {
        <section class="table-page" class=("table-admin_users_page", is_users) class=("table-admin_roles_page", is_roles) class=("table-admin_sessions_page", is_sessions) data-renderer="csr">
            {resource_actions}
            {revoke_all}
            {grid}
            {pagination}
        </section>
    }
}
