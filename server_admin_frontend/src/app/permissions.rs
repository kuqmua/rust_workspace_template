use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminPermissionsView(
    page: server_admin_contract::AdminPermissionsPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <div data-name="TableWrapper" class="table-scroll max-h-96 overflow-auto rounded-md border"><table data-name="Table" class="w-full max-w-7xl text-sm caption-bottom"><thead data-name="TableHeader" class="[&_tr]:border-b sticky top-0 z-10 bg-card"><tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"id"</th><th data-name="TableHead" class="h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]">"permission"</th></tr></thead>
            <tbody data-name="TableBody" class="[&_tr:last-child]:border-0">{page.items().iter().map(|item| leptos::view! {
                <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50"><td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="id">{item.id().to_string()}</td><td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" data-label="permission">{item.name().to_string()}</td></tr>
            }).collect::<Vec<_>>()}</tbody></table></div>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Permissions query=query total=page.total() />
        </section>
    }
}
