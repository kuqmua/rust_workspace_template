use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from sibling table modules"
)]
pub(in crate::domain_types::start) fn AdminPagination(
    action: server_admin_contract::domain_types::AdminFrontendPath,
    query: super::query::AdminCsrQuery,
    total: server_admin_contract::domain_types::AdminPageTotal,
) -> impl leptos::prelude::IntoView {
    let range = crate::domain_types::shared::pagination::admin_page_range::AdminPageRange::new(
        query.offset,
        query.limit,
        total,
    );
    let total_value = u64::from(total);
    let limit = u16::from(query.limit).to_string();
    let page_size_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            &query.search,
            &query.sort,
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Csr(
                query.direction.clone(),
            ),
            query.limit,
        );
    let previous_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            &query.search,
            &query.sort,
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Csr(
                query.direction.clone(),
            ),
            query.limit,
        );
    let next_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            &query.search,
            &query.sort,
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Csr(
                query.direction,
            ),
            query.limit,
        );
    leptos::view! {
        <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
            <singlestage::PaginationContent class="contents">
            <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=action.get()>
                {page_size_query}
                <input type="hidden" name="offset" value="0" />
                <crate::domain_types::with_owner::input::AdminInputGroup>
                    <crate::domain_types::with_owner::field::AdminField label="Rows"><crate::domain_types::with_owner::input::AdminInput name="limit" kind=crate::domain_types::with_owner::input::AdminInputKind::Number min=server_admin_contract::domain_types::AdminPageLimit::MIN max=server_admin_contract::domain_types::AdminPageLimit::MAX initial_value=limit /></crate::domain_types::with_owner::field::AdminField>
                    <crate::domain_types::with_owner::button::AdminButton>"Apply"</crate::domain_types::with_owner::button::AdminButton>
                </crate::domain_types::with_owner::input::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action.get()>
                {previous_query}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::domain_types::with_owner::button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total_value)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action.get()>
                {next_query}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::domain_types::with_owner::button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}
