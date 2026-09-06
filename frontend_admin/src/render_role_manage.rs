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
    admin_roles_page: &server_admin_contract::admin_roles_page::AdminRolesPage,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_update = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesUpdate),
    );
    let can_delete = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::RolesDelete),
    );
    let cards = admin_roles_page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("role-{}", item.id());
        let title = format!("{} (#{})", item.name(), item.id());
        let name = item.name().to_string();
        let is_system = bool::from(item.is_system());
        let status = if is_system {
            constants_str::ADMIN_UI_SYSTEM_ROLE
        } else {
            constants_str::ADMIN_UI_CUSTOM_ROLE
        };
        leptos::view! {
            <crate::admin_card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get()>
                    <input type="hidden" name="role_id" value=id.clone() />
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROLE_NAME><crate::admin_input::AdminInput admin_input_name="name" initial_value=name required=true disabled=is_system /></crate::admin_field::AdminField>
                    <div class="crud-actions"><crate::admin_button::AdminButton bool=is_system>{constants_str::ADMIN_BUTTON_SAVE_CHANGES}</crate::admin_button::AdminButton></div>
                </form> })}
                {(can_delete && !is_system).then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get()>
                    <input type="hidden" name="role_id" value=id />
                    <div><p>{constants_str::ADMIN_UI_PERMANENTLY_REMOVE_THIS_ROLE_AND_ITS_ASSIGNMENTS}</p><label class="crud-confirm"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" bool=true />{constants_str::ADMIN_UI_I_UNDERSTAND_THIS_CANNOT_BE_UNDONE}</label></div><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger>{constants_str::ADMIN_BUTTON_DELETE_ROLE}</crate::admin_button::AdminButton>
                </form> })}
            </article></crate::admin_card::AdminCard>
        }
    }).collect::<Vec<_>>();
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Roles,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">{constants_str::ADMIN_UI_ROLES}</p><h1>{constants_str::ADMIN_UI_MANAGE_ROLES}</h1><p>{constants_str::ADMIN_UI_RENAME_CUSTOM_ROLES_OR_REMOVE_ROLES_THAT_ARE_NO_LONGER_NEEDED}</p></div><crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{constants_str::ADMIN_BUTTON_BACK_TO_ROLES}</crate::admin_button_link::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
