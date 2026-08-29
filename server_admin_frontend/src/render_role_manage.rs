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

#[must_use]
pub fn render_role_manage(
    page: &server_admin_contract::admin_roles_page::AdminRolesPage,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_update = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::RolesUpdate),
    );
    let can_delete = bool::from(
        admin.has_permission(server_admin_contract::admin_permission::AdminPermission::RolesDelete),
    );
    let cards = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("role-{}", item.id());
        let title = format!("{} (#{})", item.name(), item.id());
        let name = item.name().to_string();
        let is_system = bool::from(item.is_system());
        let status = if is_system {
            constants_str::test_fixtures::VALUE_91C86A3E
        } else {
            constants_str::test_fixtures::VALUE_5B58E07E
        };
        leptos::view! {
            <crate::admin_card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get()>
                    <input type="hidden" name="role_id" value=id.clone() />
                    <crate::admin_field::AdminField label="Role name"><crate::admin_input::AdminInput name="name" initial_value=name required=true disabled=is_system /></crate::admin_field::AdminField>
                    <div class="crud-actions"><crate::admin_button::AdminButton disabled=is_system>"Save changes"</crate::admin_button::AdminButton></div>
                </form> })}
                {(can_delete && !is_system).then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get()>
                    <input type="hidden" name="role_id" value=id />
                    <div><p>"Permanently remove this role and its assignments."</p><label class="crud-confirm"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"I understand this cannot be undone"</label></div><crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger>"Delete role"</crate::admin_button::AdminButton>
                </form> })}
            </article></crate::admin_card::AdminCard>
        }
    }).collect::<Vec<_>>();
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">"Roles"</p><h1>"Manage roles"</h1><p>"Rename custom roles or remove roles that are no longer needed."</p></div><crate::admin_button_link::AdminButtonLink href=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get() variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Back to roles"</crate::admin_button_link::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        admin,
        branding,
    )
}
