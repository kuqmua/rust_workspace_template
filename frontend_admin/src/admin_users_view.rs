#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminUsersView(
    authenticated_admin: server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_users_page: server_admin_contract::admin_users_page::AdminUsersPage,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let csr_admin_user_row =
        |admin_user_summary: &server_admin_contract::admin_user_summary::AdminUserSummary| {
            let user_path = server_admin_contract::admin_route_path::AdminRoutePath::from(
                admin_user_summary.id(),
            )
            .to_string();
            let id = admin_user_summary.id().to_string();
            let login = admin_user_summary.login().to_string();
            let display_name = admin_user_summary.display_name().to_string();
            let banned = admin_user_summary.is_banned().to_string();
            let roles =
                crate::admin_user_roles::admin_user_roles(admin_user_summary, &admin_users_page);
            leptos::view! {
                <crate::table_row::TableRow>
                    <crate::table_cell::TableCell data_label="id">{id}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="login">{login}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="display_name">{display_name}</crate::table_cell::TableCell>
                    <crate::table_cell::TableCell data_label="banned">{banned}</crate::table_cell::TableCell>
                    {roles}
                    <crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true><singlestage::Link class=crate::admin_button_variant::AdminButtonVariant::Secondary.class() href=user_path attr:aria-label=constants_str::PG_CRUD_READ_PERMISSION_ACTION attr:title=constants_str::PG_CRUD_READ_PERMISSION_ACTION>
                        <svg viewBox="0 0 24 24" aria-hidden=constants_str::TRUE fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"></path>
                            <circle cx="12" cy="12" r="3"></circle>
                        </svg>
                    </singlestage::Link></crate::table_cell::TableCell>
                </crate::table_row::TableRow>
            }
        };

    let can_create = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersCreate),
    );
    let total = admin_users_page.total();
    let rows = admin_users_page
        .items()
        .iter()
        .map(csr_admin_user_row)
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
    let users_path = server_admin_contract::admin_data_table::AdminDataTable::Users.frontend_path();
    let identifier_filter = identifier.as_ref().map(|identifier| {
        crate::admin_column_filter::admin_column_filter(
            &users_path,
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
    let text_filters = [
        frontend_contract::filter_operation::FilterOperation::Eq,
        frontend_contract::filter_operation::FilterOperation::In,
    ]
    .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let text_filter = |admin_text: &server_admin_contract::admin_text::AdminText| {
        crate::admin_column_filter::admin_column_filter(
            &users_path,
            admin_text,
            frontend_contract::input_kind::InputKind::Text,
            &text_filters,
            admin_csr_query.filter_field(),
            admin_csr_query.filter_operation(),
            admin_csr_query.filter_value(),
            admin_csr_query.filter_end(),
            admin_csr_query.limit(),
        )
    };
    let login =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::LOGIN.to_owned())
            .ok();
    let display_name = server_admin_contract::admin_text::AdminText::try_from(
        constants_str::DISPLAY_NAME.to_owned(),
    )
    .ok();
    let login_filter = login.as_ref().map(text_filter);
    let display_name_filter = display_name.as_ref().map(text_filter);
    let boolean_filters = [frontend_contract::filter_operation::FilterOperation::Eq]
        .map(server_admin_contract::admin_data_filter::AdminDataFilter::from);
    let is_banned =
        server_admin_contract::admin_text::AdminText::try_from(constants_str::IS_BANNED.to_owned())
            .ok();
    let is_banned_filter = is_banned.as_ref().map(|is_banned| {
        crate::admin_column_filter::admin_column_filter(
            &users_path,
            is_banned,
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
        <section class="table-admin_users_page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersCreate.get()>{constants_str::PG_CRUD_CREATE_PERMISSION_ACTION}</crate::admin_button_link::AdminButtonLink> })}
            </div>
            <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead data_field=constants_str::SQL_NAMES_ID.to_owned() data_filter_count=identifier_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::SQL_NAMES_ID}</span>{identifier_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::LOGIN.to_owned() data_filter_count=text_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::LOGIN}</span>{login_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::DISPLAY_NAME.to_owned() data_filter_count=text_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::DISPLAY_NAME}</span>{display_name_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead data_field=constants_str::IS_BANNED.to_owned() data_filter_count=boolean_filters.len().to_string()><div class="table-column-heading"><span>{constants_str::IS_BANNED}</span>{is_banned_filter}</div></crate::table_head::TableHead><crate::table_head::TableHead>"roles"</crate::table_head::TableHead><crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination admin_frontend_path=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users admin_csr_query=admin_csr_query admin_page_total=total />
        </section>
    }
}
