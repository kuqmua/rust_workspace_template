use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod row;

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the shell module"
)]
pub(in crate::app) fn AdminUsersView(
    admin: server_admin_contract::AuthenticatedAdmin,
    page: server_admin_contract::AdminUsersPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersCreate));
    let can_manage =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate))
            || bool::from(
                admin.has_permission(server_admin_contract::AdminPermission::UsersDelete),
            );
    let total = page.total();
    let rows = page
        .items()
        .iter()
        .map(|item| row::admin_user_row(item, &page))
        .collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div class="resource-actions">
                {can_create.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::UsersCreate.get()>"Create user"</crate::ui::button::AdminButtonLink> })}
                {can_manage.then(|| leptos::view! { <crate::ui::button::AdminButtonLink href=server_admin_contract::AdminFrontendPath::UsersManage.get() variant=crate::ui::button::AdminButtonVariant::Secondary>"Manage users"</crate::ui::button::AdminButtonLink> })}
            </div>
            <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"id"</crate::ui::table::TableHead><crate::ui::table::TableHead>"login"</crate::ui::table::TableHead><crate::ui::table::TableHead>"display_name"</crate::ui::table::TableHead><crate::ui::table::TableHead>"banned"</crate::ui::table::TableHead><crate::ui::table::TableHead>"roles"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
            <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Users query=query total=total />
        </section>
    }
}
