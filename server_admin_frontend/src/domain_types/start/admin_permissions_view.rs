use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::domain_types::app) fn AdminPermissionsView(
    page: server_admin_contract::domain_types::AdminPermissionsPage,
    query: super::query::AdminCsrQuery,
) -> impl leptos::prelude::IntoView {
    let total = page.total();
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::domain_types::ui::table::TableRow><crate::domain_types::ui::table::TableCell data_label="id">{id}</crate::domain_types::ui::table::TableCell><crate::domain_types::ui::table::TableCell data_label="permission">{permission}</crate::domain_types::ui::table::TableCell></crate::domain_types::ui::table::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::domain_types::ui::table::TableWrapper><crate::domain_types::ui::table::Table><crate::domain_types::ui::table::TableHeader><crate::domain_types::ui::table::TableRow><crate::domain_types::ui::table::TableHead>"id"</crate::domain_types::ui::table::TableHead><crate::domain_types::ui::table::TableHead>"permission"</crate::domain_types::ui::table::TableHead></crate::domain_types::ui::table::TableRow></crate::domain_types::ui::table::TableHeader>
            <crate::domain_types::ui::table::TableBody>{rows}</crate::domain_types::ui::table::TableBody></crate::domain_types::ui::table::Table></crate::domain_types::ui::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::domain_types::AdminFrontendPath::Permissions query=query total=total />
        </section>
    }
}
