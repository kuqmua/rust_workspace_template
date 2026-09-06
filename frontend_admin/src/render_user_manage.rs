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
pub fn render_user_manage(
    admin_users_page: &server_admin_contract::admin_users_page::AdminUsersPage,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let can_update = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersUpdate),
    );
    let can_delete = bool::from(
        authenticated_admin
            .has_permission(server_admin_contract::admin_permission::AdminPermission::UsersDelete),
    );
    let cards = admin_users_page.items().iter().map(|item| {
        let id = item.id().to_string();
        let article_id = format!("user-{}", item.id());
        let title = format!("{} (#{})", item.login(), item.id());
        let login = item.login().to_string();
        let display_name = item.display_name().to_string();
        let status = if bool::from(item.is_banned()) {
            constants_str::ADMIN_UI_BANNED
        } else {
            constants_str::ADMIN_UI_ACTIVE
        };
        leptos::view! {
            <crate::admin_card::AdminCard><article class="crud-record" id=article_id>
                <div class="crud-record-heading"><h2>{title}</h2><span>{status}</span></div>
                {can_update.then(|| leptos::view! { <form class="crud-form crud-form-compact" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get()>
                    <input type="hidden" name="user_id" value=id.clone() />
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_LOGIN><crate::admin_input::AdminInput admin_input_name="login" initial_value=login required=true /></crate::admin_field::AdminField>
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_DISPLAY_NAME><crate::admin_input::AdminInput admin_input_name="display_name" initial_value=display_name required=true /></crate::admin_field::AdminField>
                    <div class="crud-actions"><crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_SAVE_CHANGES}</crate::admin_button::AdminButton></div>
                </form> })}
                {can_delete.then(|| leptos::view! { <form class="crud-delete" method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get()>
                    <input type="hidden" name="user_id" value=id />
                    <div><p>{constants_str::ADMIN_UI_PERMANENTLY_REMOVE_THIS_USER_AND_THEIR_ADMINISTRATOR_ACCESS}</p><label class="crud-confirm"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" bool=true />{constants_str::ADMIN_UI_I_UNDERSTAND_THIS_CANNOT_BE_UNDONE}</label></div><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger>{constants_str::ADMIN_BUTTON_DELETE_USER}</crate::admin_button::AdminButton>
                </form> })}
            </article></crate::admin_card::AdminCard>
        }
    }).collect::<Vec<_>>();
    super::crud_render_shell::crud_render_shell(
        server_admin_contract::admin_page::AdminPage::Users,
        leptos::view! {
            <section class="crud-page"><div class="crud-heading"><div><p class="eyebrow">{constants_str::ADMIN_UI_USERS}</p><h1>{constants_str::ADMIN_UI_MANAGE_USERS}</h1><p>{constants_str::ADMIN_UI_UPDATE_ACCOUNT_DETAILS_OR_PERMANENTLY_DELETE_AN_ACCOUNT}</p></div><crate::admin_button_link::AdminButtonLink str=server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get() admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary>{constants_str::ADMIN_BUTTON_BACK_TO_USERS}</crate::admin_button_link::AdminButtonLink></div><div class="crud-list">{cards}</div></section>
        },
        authenticated_admin,
        admin_branding_view,
    )
}
