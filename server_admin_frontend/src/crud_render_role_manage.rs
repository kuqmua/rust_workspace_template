#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the server-rendered CRUD forms require Leptos attribute traits after macro expansion"
)]

#[allow(
    unused_import_braces,
    reason = "grouped Leptos prelude imports are required by workspace source policy"
)]
#[rustfmt::skip]
use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes};

#[allow(clippy::single_call_fn)] // named UI component or render stage has one composition owner
pub(super) fn crud_render_role_manage(
    page: &server_admin_contract::domain_types::AdminRolesPage,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesUpdate),
    );
    let can_delete = bool::from(
        admin.has_permission(server_admin_contract::domain_types::AdminPermission::RolesDelete),
    );
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("role-{}", item.id());
        let title = format!("{} (#{})", item.name(), item.id());
        let name = item.name().to_string();
        let is_system = bool::from(item.is_system());
        let status = if is_system {
            constants_str::VALUE_91C86A3E
        } else {
            constants_str::VALUE_5B58E07E
        };
        leptos::view! {
            <crate::domain_types::with_owner::card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::RoleUpdate.get()>
                    <input type="hidden" name="role_id" value=id.clone() />
                    <crate::domain_types::with_owner::field::AdminField label="Role name"><crate::domain_types::with_owner::input::AdminInput name="name" initial_value=name required=true disabled=is_system /></crate::domain_types::with_owner::field::AdminField>
                    <div class="crud-actions"><crate::domain_types::with_owner::button::AdminButton disabled=is_system>"Save changes"</crate::domain_types::with_owner::button::AdminButton></div>
                </form> })}
                {(can_delete && !is_system).then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::domain_types::AdminHtmlAction::RoleDelete.get()>
                    <input type="hidden" name="role_id" value=id />
                    <div><p>"Permanently remove this role and its assignments."</p><label class="crud-confirm"><crate::domain_types::with_owner::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Danger>"Delete role"</crate::domain_types::with_owner::button::AdminButton>
                </form> })}
            </article></crate::domain_types::with_owner::card::AdminCard>
        }
    }).collect::<Vec<_>>();
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::domain_types::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Manage roles"</h1><p>"Rename custom roles or remove roles that are no longer needed."</p></div><crate::domain_types::with_owner::button::AdminButtonLink href=server_admin_contract::domain_types::AdminFrontendPath::Roles.get() variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Back to roles"</crate::domain_types::with_owner::button::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
