#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminRolesView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_roles_page: server_admin_contract::admin_roles_page::AdminRolesPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let csr_admin_role_row =
        |admin_role_summary: &server_admin_contract::admin_role_summary::AdminRoleSummary| {
            let id = admin_role_summary.id().to_string();
            let name = admin_role_summary.name().to_string();
            let system = admin_role_summary.is_system().to_string();
            let permissions = crate::admin_role_permissions::admin_role_permissions(
                admin_role_summary,
                &admin_roles_page,
            );
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="name">{name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="system">{system}</crate::table_cell::TableCell>
                    {permissions}
                    <crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true>{constants_str::EMPTY}</crate::table_cell::TableCell>
                </crate::table_row::TableRow>
            }
        };

    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesCreate),
    );
    let total = admin_roles_page.total();
    let rows = admin_roles_page
        .items()
        .iter()
        .map(csr_admin_role_row)
        .collect::<Vec<_>>();
    let identifier_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::GreaterThan,
        frontend_contract::filter_operation::FilterOperation::Between,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let identifier = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::SQL_NAMES_ID.to_owned(),
    )
    .ok();
    let roles_path = server_admin_contract::admin_data_table::AdminDataTable::Roles.frontend_path();
    let identifier_filter = identifier.as_ref().map(|identifier| {
        crate::admin_column_filter::admin_column_filter(
            &roles_path,
            identifier,
            frontend_contract::input_kind::InputKind::Number,
            &identifier_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    });
    let name_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let name =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::NAME.to_owned()).ok();
    let name_filter = name.as_ref().map(|name| {
        crate::admin_column_filter::admin_column_filter(
            &roles_path,
            name,
            frontend_contract::input_kind::InputKind::Text,
            &name_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    });
    let boolean_filters = [frontend_contract::filter_operation::FilterOperation::Eq]
        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let is_system =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::IS_SYSTEM.to_owned())
            .ok();
    let is_system_filter = is_system.as_ref().map(|is_system| {
        crate::admin_column_filter::admin_column_filter(
            &roles_path,
            is_system,
            frontend_contract::input_kind::InputKind::Checkbox,
            &boolean_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    });
    leptos::view! {
        <section class="table-admin_roles_page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::RolesCreate.get()>{constants_str::PG_CRUD_CREATE_PERMISSION_ACTION}</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead data_field=constants_str::SQL_NAMES_ID.to_owned() data_filter_count=identifier_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::SQL_NAMES_ID}</span>{identifier_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::NAME.to_owned() data_filter_count=name_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::NAME}</span>{name_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::IS_SYSTEM.to_owned() data_filter_count=boolean_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::IS_SYSTEM}</span>{is_system_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead>"permissions"</crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
