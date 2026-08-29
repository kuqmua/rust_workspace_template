#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub fn render_admin_sessions_page(
    page: &server_admin_contract::admin_sessions_page::AdminSessionsPage,
    query: &server_admin_contract::admin_table_query::AdminTableQuery,
    admin: &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    branding: &server_admin_contract::admin_branding_view::AdminBrandingView,
) -> crate::admin_ssr_html::AdminSsrHtml {
    let rows = page.items().iter().map(|item| {
        let session_id = item.id().to_string();
        let hidden_session_id = session_id.clone();
        let dialog_id = format!("revoke-session-{hidden_session_id}");
        let form_id = format!("revoke-session-form-{hidden_session_id}");
        let created_at = item.created_at().to_string();
        let expires_at = item.expires_at().to_string();
        let is_current = bool::from(item.is_current());
        let current_text = item.is_current().to_string();
        let confirm_form_id = form_id.clone();
        let dialog = crate::with_owner::with_owner(move || {
            leptos::view! {
                <singlestage::Dialog alert=true id=dialog_id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg" dialog_trigger=singlestage::DialogTrigger::builder().children(leptos::prelude::ToChildren::to_children(move || leptos::view! {
                    <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger kind=crate::admin_button_kind::AdminButtonKind::Button>"Revoke session"</crate::admin_button::AdminButton>
                })).build()>
                    <singlestage::DialogContent attr:data-name="AlertDialogContent" class="flex flex-col gap-4">
                        <div data-name="AlertDialogBody" class="contents">
                            <singlestage::DialogHeader attr:data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                                <singlestage::DialogTitle attr:data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">"Revoke session?"</singlestage::DialogTitle>
                                <singlestage::DialogDescription attr:data-name="AlertDialogDescription" class="text-sm text-muted-foreground">"This administrator session will be signed out immediately."</singlestage::DialogDescription>
                            </singlestage::DialogHeader>
                            <form id=form_id method="post" action=server_admin_contract::admin_html_action::AdminHtmlAction::SessionRevoke.get()>
                                <input type="hidden" name="session_id" value=hidden_session_id />
                                <singlestage::Label attr:data-name="Label" class="flex items-center gap-2 text-sm leading-none font-medium select-none"><crate::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"Confirm session revocation"</singlestage::Label>
                            </form>
                            <singlestage::DialogFooter attr:data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                                <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Secondary>"Cancel"</crate::admin_button::AdminButton>
                                <crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Danger form=confirm_form_id>"Revoke session"</crate::admin_button::AdminButton>
                            </singlestage::DialogFooter>
                        </div>
                    </singlestage::DialogContent>
                </singlestage::Dialog>
            }
        });
        leptos::view! {
            <crate::table_row::TableRow>
                <crate::table_cell::TableCell data_label="session">{session_id}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="created">{created_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="expires">{expires_at}</crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="current"><crate::admin_badge::AdminBadge variant=if is_current { crate::admin_badge_variant::AdminBadgeVariant::Success } else { crate::admin_badge_variant::AdminBadgeVariant::Neutral }>{current_text}</crate::admin_badge::AdminBadge></crate::table_cell::TableCell>
                <crate::table_cell::TableCell data_label="actions">
                    {dialog}
                </crate::table_cell::TableCell>
            </crate::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::table_wrapper::TableWrapper><crate::table::Table><crate::table_header::TableHeader><crate::table_row::TableRow><crate::table_head::TableHead>"session"</crate::table_head::TableHead><crate::table_head::TableHead>"created"</crate::table_head::TableHead><crate::table_head::TableHead>"expires"</crate::table_head::TableHead><crate::table_head::TableHead>"current"</crate::table_head::TableHead><crate::table_head::TableHead>"actions"</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
        <crate::table_body::TableBody>{rows}</crate::table_body::TableBody></crate::table::Table></crate::table_wrapper::TableWrapper>
        {crate::table_pagination::table_pagination(server_admin_contract::admin_page::AdminPage::Sessions, query, page.total(), None, None)}
        </section>
    };
    let content = crate::render_view::render_view(content_view);
    crate::render_admin_page_with_access::render_admin_page_with_access(
        server_admin_contract::admin_page::AdminPage::Sessions,
        content,
        Some(admin),
        Some(branding),
    )
}
