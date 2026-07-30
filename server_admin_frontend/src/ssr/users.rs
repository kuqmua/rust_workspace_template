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
    page: &server_admin_contract::AdminUsersPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_create =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersCreate));
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate));
    let can_update_roles =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate));
    let content = leptos::view! {
        <section class="table-page">
        {crate::shared::table_filters::admin_table_filters(server_admin_contract::AdminFrontendPath::Users, query.search(), query.sort(), crate::shared::table_filters::AdminTableFilterDirection::from(query.direction()), query.limit(), &server_admin_contract::AdminTableSortField::USER, crate::shared::table_filters::AdminTableFilterPresentation::Ssr)}
        {can_create.then(|| leptos::view! { <details class="mutation-form"><summary>"Create user"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserCreate.get()>
            <label><span>"Login"</span><input name="login" required /></label><label><span>"Display name"</span><input name="display_name" required /></label>
            <label><span>"Password"</span><input name="password" type="password" required /></label><button type="submit">"Create user"</button>
        </form></details> })}
        <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
        <tbody>{page.items().iter().map(|item| { let expected_role_ids = item.role_ids().iter().map(ToString::to_string).collect::<Vec<_>>().join(","); leptos::view! {
            <tr><td data-label="id">{item.id().to_string()}</td><td data-label="login">{item.login().to_string()}</td><td data-label="display_name">{item.display_name().to_string()}</td><td data-label="banned">{item.is_banned().to_string()}</td>
            <td data-label="roles">{can_update_roles.then(|| leptos::view! { <form method="post" action=server_admin_contract::AdminHtmlAction::UserRoles.get()><input type="hidden" name="user_id" value=item.id().to_string() />
                <input type="hidden" name="expected_role_ids" value=expected_role_ids />
                {page.roles().iter().map(|role| { let checked = item.role_ids().contains(&role.id()); let name = format!("role_{}", role.id()); leptos::view! { <label><input type="checkbox" name=name value=role.id().to_string() checked=checked />{role.name().to_string()}</label> } }).collect::<Vec<_>>()}
                <button type="submit">"Save roles"</button></form> })}</td>
            <td data-label="actions">{can_update.then(|| leptos::view! { <details><summary>"Edit"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserUpdate.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="login" value=item.login().to_string() required /><input name="display_name" value=item.display_name().to_string() required /><button type="submit">"Save"</button></form></details>
                <details><summary>"Password"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserPassword.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input name="password" type="password" required /><button type="submit">"Change password"</button></form></details>
                <form method="post" action=server_admin_contract::AdminHtmlAction::UserBan.get()><input type="hidden" name="user_id" value=item.id().to_string() /><input type="hidden" name="is_banned" value=(!bool::from(item.is_banned())).to_string() /><button type="submit">{if bool::from(item.is_banned()) { "Unban" } else { "Ban" }}</button></form> })}
                {can_delete.then(|| leptos::view! { <details><summary>"Delete"</summary><form method="post" action=server_admin_contract::AdminHtmlAction::UserDelete.get()><input type="hidden" name="user_id" value=item.id().to_string() /><label><input type="checkbox" name="confirmation" value="true" required />"Confirm permanent deletion"</label><button class="danger-button" type="submit">"Delete user"</button></form></details> })}</td></tr>
        }}).collect::<Vec<_>>()}</tbody></table></div>
        {super::table_pagination(server_admin_contract::AdminPage::Users, query, page.total(), None, None)}
        </section>
    }.render_admin_ssr();
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Users,
        content,
        Some(admin),
        Some(branding),
    )
}
