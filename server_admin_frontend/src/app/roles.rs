use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, OnAttribute};

mod create;

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
    let permissions = page.permissions().to_vec();
    let can_create = admin.has_permission(server_admin_contract::AdminPermission::RolesCreate);
    let can_delete =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesDelete));
    let can_update =
        bool::from(admin.has_permission(server_admin_contract::AdminPermission::RolesUpdate));
    let can_update_permissions = bool::from(
        admin.has_permission(server_admin_contract::AdminPermission::RolePermissionsUpdate),
    );
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::table_filters::admin_table_filters(server_admin_contract::AdminFrontendPath::Roles, &query.search, &query.sort, crate::shared::table_filters::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::ROLE, crate::shared::table_filters::AdminTableFilterPresentation::Csr)}
            <create::AdminCreateRole can_create=can_create />
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"name"</th><th>"system"</th><th>"permissions"</th><th>"actions"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| {
                let name = leptos::prelude::RwSignal::new(item.name().to_string());
                let selected_permissions = leptos::prelude::RwSignal::new(item.permission_ids().to_vec());
                let expected_permissions = item.permission_ids().to_vec();
                let update_role_id = item.id();
                let permissions_role_id = item.id();
                let delete_role_id = item.id();
                leptos::view! {
                <tr>
                    <td data-label="id">{item.id().to_string()}</td>
                    <td data-label="name"><input disabled=!can_update value=item.name().to_string() on:input=move |event| leptos::prelude::Set::set(&name, leptos::prelude::event_target_value(&event)) /></td>
                    <td data-label="system">{item.is_system().to_string()}</td>
                    <td data-label="permissions"><div class="table-options">{permissions.iter().map(|permission| {
                        let permission_id = permission.id();
                        let checked = item.permission_ids().contains(&permission_id);
                        leptos::view! { <label><input type="checkbox" checked=checked disabled=!can_update_permissions on:change=move |event| {
                            leptos::prelude::Update::update(&selected_permissions, |ids| {
                                if leptos::prelude::event_target_checked(&event) {
                                    if !ids.contains(&permission_id) { ids.push(permission_id); }
                                } else { ids.retain(|value| *value != permission_id); }
                            });
                        } />{permission.name().to_string()}</label> }
                    }).collect::<Vec<_>>()}</div></td>
                    <td data-label="actions"><div class="table-actions">
                        {can_update.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            if let (Ok(value), Ok(path)) = (
                                server_admin_contract::AdminRoleName::try_from(leptos::prelude::Get::get(&name)),
                                super::http::admin_api_url(server_admin_contract::AdminRoute::UpdateRole(update_role_id)),
                            ) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Patch, path, server_admin_contract::AdminUpdateRoleReq::new(value));
                            }
                        }>"Save"</button> })}
                        {can_update_permissions.then(|| leptos::view! { <button type="button" on:click=move |_event| {
                            let expected = server_admin_contract::AdminPermissionIds::try_from(expected_permissions.clone());
                            let selected = server_admin_contract::AdminPermissionIds::try_from(leptos::prelude::Get::get(&selected_permissions));
                            if let (Ok(expected), Ok(selected), Ok(path)) = (
                                expected,
                                selected,
                                super::http::admin_api_url(server_admin_contract::AdminRoute::SetRolePermissions(permissions_role_id)),
                            ) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Put, path, server_admin_contract::AdminSetRolePermissionsReq::new(expected, selected));
                            }
                        }>"Save permissions"</button> })}
                        {can_delete.then(|| leptos::view! { <button class="danger-button" type="button" disabled=bool::from(item.is_system()) on:click=move |_event| {
                            if bool::from(super::mutation::mutation_confirmed(super::mutation::MutationConfirmationMessageRef::from("Delete this role?"))) && let Ok(path) = super::http::admin_api_url(server_admin_contract::AdminRoute::DeleteRole(delete_role_id)) {
                                super::mutation::reload_after(super::mutation::AdminMutationMethod::Delete, path, server_admin_contract::AdminNoBody);
                            }
                        }>"Delete"</button> })}
                    </div></td>
                </tr>
            }}).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=page.total() />
        </section>
    }
}
