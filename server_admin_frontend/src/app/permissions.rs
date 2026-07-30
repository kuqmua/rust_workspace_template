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
            {crate::shared::table_filters::form::admin_table_filters(server_admin_contract::AdminFrontendPath::Permissions, &query.search, &query.sort, crate::shared::table_filters::form::AdminTableFilterDirection::from_csr(query.direction.as_ref()), query.limit, &server_admin_contract::AdminTableSortField::PERMISSION, crate::shared::table_filters::form::AdminTableFilterPresentation::Csr)}
            <div class="table-scroll"><table><thead><tr><th>"id"</th><th>"permission"</th></tr></thead>
            <tbody>{page.items().iter().map(|item| leptos::view! {
                <tr><td data-label="id">{item.id().to_string()}</td><td data-label="permission">{item.name().to_string()}</td></tr>
            }).collect::<Vec<_>>()}</tbody></table></div>
            <p>{format!("{} total", page.total())}</p>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Permissions query=query total=page.total() />
        </section>
    }
}
