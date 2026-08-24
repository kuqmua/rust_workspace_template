use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod row;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::app) fn AdminRolesView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminRolesPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesCreate));
    let can_manage =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate))
            || bool::from(
                admin.has_permission(server_admin_contract::AdminPermission::RolesDelete),
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
                {can_create.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::RolesCreate.get()>"Create role"</crate::ui::button::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::RolesManage.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Manage roles"</crate::ui::button::AdminButtonLink> })}
            </div>
            <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"id"</crate::ui::table::TableHead><crate::ui::table::TableHead>"name"</crate::ui::table::TableHead><crate::ui::table::TableHead>"system"</crate::ui::table::TableHead><crate::ui::table::TableHead>"permissions"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
            <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=total />
        </section>
    }
}
