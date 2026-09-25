#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

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
    let supports_filters = bool::from(admin_data_table_view.table().supports_filters());
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
    let limit = u16::from(admin_csr_query.limit());
    let limit_text = limit.to_string();
    let range = crate::admin_page_range::AdminPageRange::new(
        admin_csr_query.offset(),
        admin_csr_query.limit(),
        total,
    );
    let filter_field = supports_filters
        .then_some(admin_csr_query.filter_field())
        .flatten();
    let filter_operation = supports_filters
        .then_some(admin_csr_query.filter_operation())
        .flatten();
    let filter_value = supports_filters
        .then_some(admin_csr_query.filter_value())
        .flatten();
    let filter_end = supports_filters
        .then_some(admin_csr_query.filter_end())
        .flatten();
    let grid = crate::admin_data_table_grid::admin_data_table_grid(
        &admin_data_table_view,
        admin_csr_query.filter_field(),
        admin_csr_query.filter_operation(),
        admin_csr_query.filter_value(),
        admin_csr_query.filter_end(),
        admin_csr_query.limit(),
        &table_path,
        is_sessions,
        can_update,
    );
    let page_size_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let previous_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let next_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let table_query_inputs = || {
        (is_roles || is_sessions).then(|| {
            crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
                admin_csr_query.search(),
                admin_csr_query.sort(),
                &crate::admin_table_query_direction::AdminTableQueryDirection::Csr(
                    admin_csr_query.direction().cloned(),
                ),
                admin_csr_query.limit(),
            )
        })
    };
    let page_size_query = table_query_inputs();
    let previous_query = table_query_inputs();
    let next_query = table_query_inputs();
    let page_size_action = table_path.as_ref().to_owned();
    let previous_action = table_path.as_ref().to_owned();
    let next_action = table_path.as_ref().to_owned();
    let previous_limit = limit_text.clone();
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
    let update_user = (is_users && can_update).then(|| leptos::view! {
        <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersUpdate.get()>{constants_str::PG_CRUD_UPDATE_RULE_ACTION}</crate::admin_button_link::AdminButtonLink>
    });
    let update_role = (is_roles && can_update).then(|| leptos::view! {
        <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesUpdate.get()>{constants_str::PG_CRUD_UPDATE_RULE_ACTION}</crate::admin_button_link::AdminButtonLink>
    });
    let resource_actions = (is_users || is_roles).then(|| {
        leptos::view! {
            <div class="resource-actions">{create_user}{update_user}{create_role}{update_role}</div>
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
            <singlestage::Pagination attr:data-name="Pagination" attr:aria-label=constants_str::ADMIN_UI_TABLE_PAGES class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
                <singlestage::PaginationContent class="contents">
                <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=page_size_action>
                    {page_size_query}
                    {page_size_filter}
                    <input type="hidden" name="offset" value="0" />
                    <crate::admin_input_group::AdminInputGroup>
                        <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROWS><crate::admin_input::AdminInput admin_input_name="limit" admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::admin_field::AdminField>
                        <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                    </crate::admin_input_group::AdminInputGroup>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                    {previous_query}
                    <input type="hidden" name="limit" value=previous_limit />
                    {previous_filter}
                    <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.previous_disabled())>{constants_str::ADMIN_BUTTON_PREVIOUS}</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><span>{format!("{}_{}_{}_{}_{}", u64::from(range.start()), constants_str::ADMIN_UI_TO, u64::from(range.end()), constants_str::ADMIN_UI_OF, u64::from(total))}</span></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=next_action>
                    {next_query}
                    <input type="hidden" name="limit" value=limit_text />
                    {next_filter}
                    <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.next_disabled())>{constants_str::ADMIN_BUTTON_NEXT}</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                </singlestage::PaginationContent>
            </singlestage::Pagination>
        </section>
    }
}
