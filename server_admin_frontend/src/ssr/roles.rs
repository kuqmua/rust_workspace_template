use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

trait AdminSsrViewExt {
    fn render_admin_ssr(self) -> super::AdminSsrHtml;
}
impl<View> AdminSsrViewExt for View
where
    View: leptos::prelude::IntoAny,
{
    fn render_admin_ssr(self) -> super::AdminSsrHtml {
        super::AdminSsrHtml::try_from(leptos::prelude::RenderHtml::to_html(
            leptos::prelude::IntoAny::into_any(self),
        ))
        .unwrap_or_else(super::AdminSsrHtml::from)
    }
}

#[allow(
    clippy::single_call_fn,
    reason = "the screen renderer is isolated behind the stable public SSR facade"
)]
pub(super) fn render(
    page: &server_admin_contract::AdminRolesPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate));
    let can_update_permissions = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate),
    );
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, query.search(), query.sort(), crate::shared::table_filters::form::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::ROLE, crate::shared::table_filters::form::AdminTableFilterPresentation::Ssr)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create role"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleCreate.get()><label><span>"Name"</span><input name="name" required /></label><button type="submit">"Create role"</button></form></details> })}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_permission_ids = item.permission_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="name">{item.name().to_string()}</td><td data-label="system">{item.is_system().to_string()}</td><td data-label="permissions">{can_update_permissions.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RolePermissions.get()><input type="hidden" name="role_id" value=item.id().to_string() />
                <input type="hidden" name="expected_permission_ids" value=expected_permission_ids />
                {page.permissions().iter().map(|permission| { let checked = item.permission_ids().contains(&permission.id()); let name = format!("permission_{}", permission.id()); leptos::view! { <label><input type="checkbox" name=name value=permission.id().to_string() checked=checked />{permission.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save permissions"</button></form> })}</td><td data-label="actions">
                {can_update.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::RoleUpdate.get()><input type="hidden" name="role_id" value=item.id().to_string() /><input name="name" value=item.name().to_string() required /><button type="submit">"Save"</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::RoleDelete.get()><input type="hidden" name="role_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit" disabled=bool::from(item.is_system())>"Delete role"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Roles, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Roles,
        content,
        Some(admin),
        Some(branding),
    )
}
