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
    page: &server_admin_contract::AdminSessionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
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
        let dialog = crate::ui::with_owner(move || {
            leptos::view! {
                <singlestage::Dialog alert=true id=dialog_id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg" dialog_trigger=singlestage::DialogTrigger::builder().children(leptos::prelude::ToChildren::to_children(move || leptos::view! {
                    <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger kind=crate::ui::button::AdminButtonKind::Button>"Revoke session"</crate::ui::button::AdminButton>
                })).build()>
                    <singlestage::DialogContent attr:data-name="AlertDialogContent" class="flex flex-col gap-4">
                        <div data-name="AlertDialogBody" class="contents">
                            <singlestage::DialogHeader attr:data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                                <singlestage::DialogTitle attr:data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">"Revoke session?"</singlestage::DialogTitle>
                                <singlestage::DialogDescription attr:data-name="AlertDialogDescription" class="text-sm text-muted-foreground">"This administrator session will be signed out immediately."</singlestage::DialogDescription>
                            </singlestage::DialogHeader>
                            <form id=form_id method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()>
                                <input type="hidden" name="session_id" value=hidden_session_id />
                                <singlestage::Label attr:data-name="Label" class="flex items-center gap-2 text-sm leading-none font-medium select-none"><crate::ui::checkbox::AdminCheckbox name="confirmation" value="true" required=true />"Confirm session revocation"</singlestage::Label>
                            </form>
                            <singlestage::DialogFooter attr:data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                                <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary>"Cancel"</crate::ui::button::AdminButton>
                                <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger form=confirm_form_id>"Revoke session"</crate::ui::button::AdminButton>
                            </singlestage::DialogFooter>
                        </div>
                    </singlestage::DialogContent>
                </singlestage::Dialog>
            }
        });
        leptos::view! {
            <crate::ui::table::TableRow>
                <crate::ui::table::TableCell data_label="session">{session_id}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="created">{created_at}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="expires">{expires_at}</crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="current"><crate::ui::badge::AdminBadge variant=if is_current { crate::ui::badge::AdminBadgeVariant::Success } else { crate::ui::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::ui::badge::AdminBadge></crate::ui::table::TableCell>
                <crate::ui::table::TableCell data_label="actions">
                    {dialog}
                </crate::ui::table::TableCell>
            </crate::ui::table::TableRow>
        }
    }).collect::<Vec<_>>();
    let content_view = leptos::view! {
        <section class="table-page">
        <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"session"</crate::ui::table::TableHead><crate::ui::table::TableHead>"created"</crate::ui::table::TableHead><crate::ui::table::TableHead>"expires"</crate::ui::table::TableHead><crate::ui::table::TableHead>"current"</crate::ui::table::TableHead><crate::ui::table::TableHead>"actions"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
        <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
        {super::table_pagination(server_admin_contract::AdminPage::Sessions, query, page.total(), None, None)}
        </section>
    };
    let content = super::render_view(content_view);
    super::render_admin_page_with_access(
        server_admin_contract::AdminPage::Sessions,
        content,
        Some(admin),
        Some(branding),
    )
}
