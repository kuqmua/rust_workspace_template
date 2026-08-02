#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the screen-local Leptos view branches require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
    InnerHtmlAttribute, StyleAttribute,
};

#[must_use]
pub(super) fn render_sessions(
    page: &server_admin_contract::AdminSessionsPage,
    query: &server_admin_contract::AdminTableQuery,
    admin: &server_admin_contract::AuthenticatedAdmin,
    branding: &server_admin_contract::AdminBrandingView,
) -> super::AdminSsrHtml {
    let content_view = leptos::view! {
        <section class="table-page">
        <div data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border"><table data-name="Table" class="w-full max-w-7xl text-sm caption-bottom"><thead data-name="TableHeader" class="[&_tr]:border-b sticky top-0 z-10 bg-card"><tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"session"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"created"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"expires"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"current"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"actions"</th></tr></thead>
        <tbody data-name="TableBody" class="[&_tr:last-child]:border-0">{page.items().iter().map(|item| {
            let session_id = item.id().to_string();
            let hidden_session_id = session_id.clone();
            let dialog_id = format!("revoke-session-{hidden_session_id}");
            let cancel_dialog_id = dialog_id.clone();
            let created_at = item.created_at().to_string();
            let expires_at = item.expires_at().to_string();
            let is_current = bool::from(item.is_current());
            let current_text = item.is_current().to_string();
            leptos::view! {
                <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="session">{session_id}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="created">{created_at}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="expires">{expires_at}</td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="current"><crate::ui::badge::AdminBadge variant=if is_current { crate::ui::badge::AdminBadgeVariant::Success } else { crate::ui::badge::AdminBadgeVariant::Neutral }>{current_text}</crate::ui::badge::AdminBadge></td>
                    <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="actions">
                        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger kind=crate::ui::button::AdminButtonKind::Button command_for=dialog_id.clone() command="show-modal">"Revoke session"</crate::ui::button::AdminButton>
                        <dialog data-name="AlertDialogContent" id=dialog_id class="w-full max-w-lg rounded-2xl border bg-background p-6 shadow-lg backdrop:bg-black/50">
                            <form method="post" action=server_admin_contract::AdminHtmlAction::SessionRevoke.get()>
                                <div data-name="AlertDialogBody" class="flex flex-col gap-4">
                                    <div data-name="AlertDialogHeader" class="flex flex-col gap-2 text-center sm:text-left">
                                        <h3 data-name="AlertDialogTitle" class="text-lg leading-none font-semibold">"Revoke session?"</h3>
                                        <p data-name="AlertDialogDescription" class="text-sm text-muted-foreground">"This administrator session will be signed out immediately."</p>
                                    </div>
                                    <input type="hidden" name="session_id" value=hidden_session_id />
                                    <label data-name="Label" class="flex items-center gap-2 text-sm leading-none font-medium select-none"><crate::ui::checkbox::AdminCheckbox name="confirmation" value="true" required=true />"Confirm session revocation"</label>
                                    <footer data-name="AlertDialogFooter" class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
                                        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary kind=crate::ui::button::AdminButtonKind::Button command_for=cancel_dialog_id command="close">"Cancel"</crate::ui::button::AdminButton>
                                        <crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Danger>"Revoke session"</crate::ui::button::AdminButton>
                                    </footer>
                                </div>
                            </form>
                        </dialog>
                    </td>
                </tr>
            }
        }).collect::<Vec<_>>()}</tbody></table></div>
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
