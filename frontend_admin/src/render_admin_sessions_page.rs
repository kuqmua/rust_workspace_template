#![allow(
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{AddAnyAttr, AriaAttributes, ClassAttribute, ElementChild, GlobalAttributes};

#[must_use]
pub fn render_admin_sessions_page(
    admin_sessions_page: &server_admin_contract::admin_sessions_page::AdminSessionsPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    authenticated_admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    admin_branding_view: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rows = admin_sessions_page.items().iter().map(|item| {
        let session_id = item.id().to_string();
        let hidden_session_id = session_id.clone();
        let dialog_id = format!("revoke-session-{hidden_session_id}");
        let form_id = format!("revoke-session-form-{hidden_session_id}");
        let created_at = item.created_at().to_string();
        let expires_at = item.expires_at().to_string();
        let current_text = item.is_current().to_string();
        let confirm_form_id = form_id.clone();
        let action_dialog_id = dialog_id.clone();
        let read_path = server_admin_contract::admin_access_session_id::AdminAccessSessionId::try_from(
            item.id().to_string(),
        )
        .ok()
        .map(server_admin_contract::admin_route_path::AdminRoutePath::from);
        let read_session_id = session_id.clone();
        let read_created_at = created_at.clone();
        let read_expires_at = expires_at.clone();
        let read_current_text = current_text.clone();
        let read_action = read_path.map(|read_path| leptos::prelude::IntoAny::into_any(leptos::view! {
            <crate::admin_read_action::AdminReadAction read_path=read_path>
                <div class="health-label">{constants_str::SQL_NAMES_ID}</div>
                <div class="health-result">{read_session_id}</div>
                <div class="health-label">"created"</div>
                <div class="health-result">{read_created_at}</div>
                <div class="health-label">"expires"</div>
                <div class="health-result">{read_expires_at}</div>
                <div class="health-label">"current"</div>
                <div class="health-result">{read_current_text}</div>
            </crate::admin_read_action::AdminReadAction>
        }));
        let dialog = crate::with_owner::with_owner(move || {
            leptos::view! {
                <dialog id=dialog_id class="singlestage-dialog" aria-label=constants_str::ADMIN_UI_REVOKE_SESSION>
                    <form id=form_id method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::SessionRevoke.get()>
                        <input type="hidden" name="session_id" value=hidden_session_id />
                        <singlestage::Label attr:data-name="Label" class="flex items-center gap-2 text-sm leading-none font-medium select-none"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" bool=true />{constants_str::ADMIN_UI_CONFIRM_SESSION_REVOCATION}</singlestage::Label>
                        <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Danger form=confirm_form_id>{constants_str::ADMIN_BUTTON_REVOKE_SESSION}</crate::admin_button::AdminButton>
                        <crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary attr:formmethod="dialog" attr:formnovalidate=true>{constants_str::ADMIN_BUTTON_CANCEL}</crate::admin_button::AdminButton>
                    </form>
                </dialog>
            }
        });
        leptos::view! {
            <crate::table_row::TableRow>
                <crate::table_cell::TableCell data_label=constants_str::SQL_NAMES_ID>{session_id}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="created">{created_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="expires">{expires_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="current">{current_text}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="actions" bool=true>
                    <crate::admin_table_actions::AdminTableActions read_action=read_action command_for=action_dialog_id>
                        {dialog}
                    </crate::admin_table_actions::AdminTableActions>
                </crate::table_cell::TableCell>
            </crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>{constants_str::SQL_NAMES_ID}</crate::table_head::TableHead><crate::table_head::TableHead>"created"</crate::table_head::TableHead><crate::table_head::TableHead>"expires"</crate::table_head::TableHead><crate::table_head::TableHead>"current"</crate::table_head::TableHead><crate::table_head::TableHead>"actions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Sessions, admin_table_query, admin_sessions_page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Sessions,
        content,
        Some(authenticated_admin),
        Some(admin_branding_view),
    )
}
