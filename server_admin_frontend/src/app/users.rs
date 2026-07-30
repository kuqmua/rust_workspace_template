use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

mod create;

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
    let roles = page.roles().to_vec();
    let can_create = admin.has_permission(server_admin_contract::AdminPermission::UsersCreate);
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UsersUpdate));
    let can_update_roles =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::UserRolesUpdate));
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::table_filters::admin_table_filters(server_admin_contract::AdminFrontendPath::Users, &query.search, &query.sort, crate::shared::table_filters::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::USER, crate::shared::table_filters::AdminTableFilterPresentation::Csr)}
            <create::AdminCreateUser can_create=can_create />
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"login"</th><th>"display_name"</th><th>"banned"</th><th>"roles"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let login = leptos::prelude::RwSignal::new(item.login().to_string());
                let display_name = leptos::prelude::RwSignal::new(item.display_name().to_string());
                let password = leptos::prelude::RwSignal::new(String::new());
                let selected_roles = leptos::prelude::RwSignal::new(item.role_ids().to_vec());
                let expected_roles = item.role_ids().to_vec();
                let update_user_id = item.id();
                let password_user_id = item.id();
                let roles_user_id = item.id();
                let ban_user_id = item.id();
                let delete_user_id = item.id();
                let is_banned = item.is_banned();
                leptos::view! {
                <tr>
                    <td data-label="id">{item.id().to_string()}</td>
                    <td data-label="login"><input disabled=!can_update value=item.login().to_string() on:input=move |event| leptos::prelude::Set::set(&login, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="display_name"><input disabled=!can_update value=item.display_name().to_string() on:input=move |event| leptos::prelude::Set::set(&display_name, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="banned">{is_banned.to_string()}</td>
                    <td data-label="roles"><div class="table-options">{roles.iter().map(|role| {
                        let role_id = role.id();
                        let checked = item.role_ids().contains(&role_id);
                        leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_roles on:change=move |event| {
                            leptos::prelude::Update::update(&selected_roles, |ids| {
                                if leptos::prelude::event_target_checked(&event) {
                                    if !ids.contains(&role_id) { ids.push(role_id); }
                                } else { ids.retain(|value| *value != role_id); }
                            });
                        } />{role.name().to_string()}</label> }
                    }).collect::<Vec<_>>()}</div></td>
                    <td data-label="actions"><div class="table-actions">
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let request = server_admin_contract::AdminUpdateUserReq::new(
                                server_admin_contract::AdminDisplayName::try_from(leptos::prelude::Get::get(&display_name)).ok(),
                                server_admin_contract::AdminLogin::try_from(leptos::prelude::Get::get(&login)).ok(),
                            );
                            if let Ok(path) = super::http::admin_api_url(server_admin_contract::AdminRoute::UpdateUser(update_user_id)) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Patch, path, request);
                            }
                        }>"Save"</button> })}
                        {can_update.then(|| leptos::view! { <><input type="password" placeholder="New password" on:input=move |event| leptos::prelude::Set::set(&password, leptos::prelude::event_target_value(&event)) />
                        <button type="button" on:click=move |_event| {
                            if let (Ok(value), Ok(path)) = (
                                server_admin_contract::AdminNewPassword::try_from(leptos::prelude::Get::get(&password)),
                                super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserPassword(password_user_id)),
                            ) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Post, path, server_admin_contract::AdminSetUserPasswordReq::new(value));
                            }
                        }>"Change password"</button></> })}
                        {can_update_roles.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let expected = server_admin_contract::AdminRoleIds::try_from(expected_roles.clone());
                            let selected = server_admin_contract::AdminRoleIds::try_from(leptos::prelude::Get::get(&selected_roles));
                            if let (Ok(expected), Ok(selected), Ok(path)) = (
                                expected,
                                selected,
                                super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserRoles(roles_user_id)),
                            ) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Put, path, server_admin_contract::AdminSetUserRolesReq::new(expected, selected));
                            }
                        }>"Save roles"</button> })}
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            if let Ok(path) = super::http::admin_api_url(server_admin_contract::AdminRoute::SetUserBan(ban_user_id)) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Post, path, server_admin_contract::AdminSetUserBanReq::new(server_admin_contract::AdminBool::from(!bool::from(is_banned))));
                            }
                        }>{if bool::from(is_banned) { "Unban" } else { "Ban" }}</button> })}
                        {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" on:click=move |_event| {
                            if bool::from(super::mutation::mutation_confirmed(super::mutation::MutationConfirmationMessageRef::from("Delete this user?"))) && let Ok(path) = super::http::admin_api_url(server_admin_contract::AdminRoute::DeleteUser(delete_user_id)) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                            }
                        }>"Delete"</button> })}
                    </div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Users query=query total=page.total() />
        </section>
    }
}
