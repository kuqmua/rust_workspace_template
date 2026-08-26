#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(super) fn render_sessions(
    page: &server_admin_contract::domain_types::AdminSessionsPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    admin: &server_admin_contract::domain_types::AuthenticatedAdmin,
    branding: &server_admin_contract::domain_types::AdminBrandingView,
) -> super::AdminSsrHtml {
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
        let dialog = crate::domain_types::with_owner::with_owner(move || {
            leptos::view! {
                <singlestage::Dialog alert=true id=dialog_id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg" dialog_trigger=singlestage::DialogTrigger::builder().children(leptos::prelude::ToChildren::to_children(move || leptos::view! {
                    <crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Danger kind=crate::domain_types::with_owner::button::AdminButtonKind::Button>"Revoke session"</crate::domain_types::with_owner::button::AdminButton>
                })).build()>
                    <singlestage::DialogContent attr:data-name="AlertDialogContent" class="flex flex-col gap-4">
                        <div data-name="AlertDialogBody" class="contents">
                            <singlestage::DialogHeader attr:data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                                <singlestage::DialogTitle attr:data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">"Revoke session?"</singlestage::DialogTitle>
                                <singlestage::DialogDescription attr:data-name="AlertDialogDescription" class="text-sm text-muted-foreground">"This administrator session will be signed out immediately."</singlestage::DialogDescription>
                            </singlestage::DialogHeader>
                            <form id=form_id method="post" action=server_admin_contract::domain_types::AdminHtmlAction::SessionRevoke.get()>
                                <input type="hidden" name="session_id" value=hidden_session_id />
                                <singlestage::Label attr:data-name="Label" class="flex items-center gap-2 text-sm leading-none font-medium select-none"><crate::domain_types::with_owner::admin_checkbox::AdminCheckbox name="confirmation" value="true" required=true />"Confirm session revocation"</singlestage::Label>
                            </form>
                            <singlestage::DialogFooter attr:data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                                <crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary>"Cancel"</crate::domain_types::with_owner::button::AdminButton>
                                <crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Danger form=confirm_form_id>"Revoke session"</crate::domain_types::with_owner::button::AdminButton>
                            </singlestage::DialogFooter>
                        </div>
                    </singlestage::DialogContent>
                </singlestage::Dialog>
            }
        });
        leptos::view! {
            <crate::domain_types::with_owner::table::table_row::TableRow>
                <crate::domain_types::with_owner::table::table_cell::TableCell data_label="session">{session_id}</crate::domain_types::with_owner::table::table_cell::TableCell>
                <crate::domain_types::with_owner::table::table_cell::TableCell data_label="created">{created_at}</crate::domain_types::with_owner::table::table_cell::TableCell>
                <crate::domain_types::with_owner::table::table_cell::TableCell data_label="expires">{expires_at}</crate::domain_types::with_owner::table::table_cell::TableCell>
                <crate::domain_types::with_owner::table::table_cell::TableCell data_label="current"><crate::domain_types::with_owner::badge::AdminBadge variant=if is_current { crate::domain_types::with_owner::badge::AdminBadgeVariant::Success } else { crate::domain_types::with_owner::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::domain_types::with_owner::badge::AdminBadge></crate::domain_types::with_owner::table::table_cell::TableCell>
                <crate::domain_types::with_owner::table::table_cell::TableCell data_label="actions">
                    {dialog}
                </crate::domain_types::with_owner::table::table_cell::TableCell>
            </crate::domain_types::with_owner::table::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::domain_types::with_owner::table::table_wrapper::TableWrapper><crate::domain_types::with_owner::table::table_impl::Table><crate::domain_types::with_owner::table::table_header::TableHeader><crate::domain_types::with_owner::table::table_row::TableRow><crate::domain_types::with_owner::table::table_head::TableHead>"session"</crate::domain_types::with_owner::table::table_head::TableHead><crate::domain_types::with_owner::table::table_head::TableHead>"created"</crate::domain_types::with_owner::table::table_head::TableHead><crate::domain_types::with_owner::table::table_head::TableHead>"expires"</crate::domain_types::with_owner::table::table_head::TableHead><crate::domain_types::with_owner::table::table_head::TableHead>"current"</crate::domain_types::with_owner::table::table_head::TableHead><crate::domain_types::with_owner::table::table_head::TableHead>"actions"</crate::domain_types::with_owner::table::table_head::TableHead></crate::domain_types::with_owner::table::table_row::TableRow></crate::domain_types::with_owner::table::table_header::TableHeader>
        <crate::domain_types::with_owner::table::table_body::TableBody>{rows}</crate::domain_types::with_owner::table::table_body::TableBody></crate::domain_types::with_owner::table::table_impl::Table></crate::domain_types::with_owner::table::table_wrapper::TableWrapper>
        {super::table_pagination(server_admin_contract::domain_types::AdminPage::Sessions, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::domain_types::AdminPage::Sessions,
        content,
        Some(admin),
        Some(branding),
    )
}
