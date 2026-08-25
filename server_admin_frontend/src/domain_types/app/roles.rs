use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod row;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::domain_types::app) fn AdminRolesView(
    admin: server_admin_contract::domain_types::AuthenticatedAdmin,
    page: server_admin_contract::domain_types::AdminRolesPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesCreate),
    );
    let can_manage = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesUpdate),
    ) || bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesDelete),
    );
    let total = page.total();
    let rows = page
        .items()
        .iter()
        .map(|item| row::admin_role_row(item, &page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::domain_types::ui::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::domain_types::ui::button::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::domain_types::ui::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::RolesManage.get() variant=crate::domain_types::ui::button::AdminButtonVariant::Secondary>"Manage roles"</crate::domain_types::ui::button::AdminButtonLink> })}
            </div>
            <crate::domain_types::ui::table::TableWrapper><crate::domain_types::ui::table::Table><crate::domain_types::ui::table::TableHeader><crate::domain_types::ui::table::TableRow><crate::domain_types::ui::table::TableHead>"id"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"name"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"system"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"permissions"</crate::domain_types::ui::table::TableHead></crate::domain_types::ui::table::TableRow></crate::domain_types::ui::table::TableHeader>
            <crate::domain_types::ui::table::TableBody>{rows}</crate::domain_types::ui::table::TableBody></crate::domain_types::ui::table::Table></crate::domain_types::ui::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::domain_types::AdminFrontendPath::Roles query=query total=total />
        </section>
    }
}
