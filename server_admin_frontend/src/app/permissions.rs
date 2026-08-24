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
    let total = page.total();
    let rows = page.items().iter().map(|item| {
        let id = item.id().to_string();
        let permission = item.name().to_string();
        leptos::view! {
            <crate::ui::table::TableRow><crate::ui::table::TableCell data_label="id">{id}</crate::ui::table::TableCell><crate::ui::table::TableCell data_label="permission">{permission}</crate::ui::table::TableCell></crate::ui::table::TableRow>
        }
    }).collect::<Vec<_>>();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            <crate::ui::table::TableWrapper><crate::ui::table::Table><crate::ui::table::TableHeader><crate::ui::table::TableRow><crate::ui::table::TableHead>"id"</crate::ui::table::TableHead><crate::ui::table::TableHead>"permission"</crate::ui::table::TableHead></crate::ui::table::TableRow></crate::ui::table::TableHeader>
            <crate::ui::table::TableBody>{rows}</crate::ui::table::TableBody></crate::ui::table::Table></crate::ui::table::TableWrapper>
            <super::pagination::AdminPagination action=server_admin_contract::AdminFrontendPath::Permissions query=query total=total />
        </section>
    }
}
