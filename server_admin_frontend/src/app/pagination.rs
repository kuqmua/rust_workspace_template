use leptos::prelude::{AriaAttributes, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from sibling table modules"
)]
pub(in crate::app) fn AdminPagination(
    action: server_admin_contract::AdminFrontendPath,
    query: super::query::AdminCsrQuery,
    total: server_admin_contract::AdminPageTotal,
) -> impl leptos::prelude::IntoView {
    let range = crate::shared::pagination::AdminPageRange::new(query.offset, query.limit, total);
    let total_value = u64::from(total);
    leptos::view! {
        <nav data-name="Pagination" class="table-pagination mx-auto flex w-full items-center justify-center gap-2" aria-label="Table pages">
            <form method="get" action=action.get()>
                {crate::shared::table_filters::query::admin_table_query_hidden_inputs(&query.search, &query.sort, &crate::shared::table_filters::query::AdminTableQueryDirection::Csr(query.direction.clone()), query.limit)}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::ui::button::AdminButton>
            </form>
            <span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total_value)}</span>
            <form method="get" action=action.get()>
                {crate::shared::table_filters::query::admin_table_query_hidden_inputs(&query.search, &query.sort, &crate::shared::table_filters::query::AdminTableQueryDirection::Csr(query.direction), query.limit)}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::ui::button::AdminButton>
            </form>
        </nav>
    }
}
