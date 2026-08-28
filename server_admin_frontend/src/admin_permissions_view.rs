use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminPermissionsView(
    page: server_admin_contract::domain_types::AdminPermissionsPage,
    query: super::admin_csr_query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let total = page.total();
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_cell::TableCell data_label="id">{id}</crate::domain_types::with_owner::tables::table_cell::TableCell><crate::domain_types::with_owner::tables::table_cell::TableCell data_label="permission">{permission}</crate::domain_types::with_owner::tables::table_cell::TableCell></crate::domain_types::with_owner::tables::table_row::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::domain_types::with_owner::tables::table_wrapper::TableWrapper><crate::domain_types::with_owner::tables::table::Table><crate::domain_types::with_owner::tables::table_header::TableHeader><crate::domain_types::with_owner::tables::table_row::TableRow><crate::domain_types::with_owner::tables::table_head::TableHead>"id"</crate::domain_types::with_owner::tables::table_head::TableHead><crate::domain_types::with_owner::tables::table_head::TableHead>"permission"</crate::domain_types::with_owner::tables::table_head::TableHead></crate::domain_types::with_owner::tables::table_row::TableRow></crate::domain_types::with_owner::tables::table_header::TableHeader>
            <crate::domain_types::with_owner::tables::table_body::TableBody>{rows}</crate::domain_types::with_owner::tables::table_body::TableBody></crate::domain_types::with_owner::tables::table::Table></crate::domain_types::with_owner::tables::table_wrapper::TableWrapper>
            <super::admin_pagination::AdminPagination action=server_admin_contract::domain_types::AdminFrontendPath::Permissions query=query total=total />
        </section>
    }
}
