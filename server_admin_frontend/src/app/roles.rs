use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

mod row;

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
    let _admin = admin;
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border"><table data-name="Table" class="w-full max-w-7xl text-sm caption-bottom"><thead data-name="TableHeader" class="[&_tr]:border-b sticky top-0 z-10 bg-card"><tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"id"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"name"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"system"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"permissions"</th></tr></thead>
            <tbody data-name="TableBody" class="[&_tr:last-child]:border-0">{page.items().iter().map(|item| {
                row::admin_role_row(item, &page)
            }).collect::<Vec<_>>()}</tbody></table></div>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Roles query=query total=page.total() />
        </section>
    }
}
