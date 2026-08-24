use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild};

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
    let limit = u16::from(query.limit).to_string();
    let page_size_query = crate::shared::table_filters::query::admin_table_query_hidden_inputs(
        &query.search,
        &query.sort,
        &crate::shared::table_filters::query::AdminTableQueryDirection::Csr(
            query.direction.clone(),
        ),
        query.limit,
    );
    let previous_query = crate::shared::table_filters::query::admin_table_query_hidden_inputs(
        &query.search,
        &query.sort,
        &crate::shared::table_filters::query::AdminTableQueryDirection::Csr(
            query.direction.clone(),
        ),
        query.limit,
    );
    let next_query = crate::shared::table_filters::query::admin_table_query_hidden_inputs(
        &query.search,
        &query.sort,
        &crate::shared::table_filters::query::AdminTableQueryDirection::Csr(query.direction),
        query.limit,
    );
    leptos::view! {
        <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
            <singlestage::PaginationContent class="contents">
            <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=action.get()>
                {page_size_query}
                <input type="hidden" name="offset" value="0" />
                <crate::ui::input::AdminInputGroup>
                    <crate::ui::field::AdminField label="Rows"><crate::ui::input::AdminInput name="limit" kind=crate::ui::input::AdminInputKind::Number min=server_admin_contract::AdminPageLimit::MIN max=server_admin_contract::AdminPageLimit::MAX initial_value=limit /></crate::ui::field::AdminField>
                    <crate::ui::button::AdminButton>"Apply"</crate::ui::button::AdminButton>
                </crate::ui::input::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action.get()>
                {previous_query}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::ui::button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total_value)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action.get()>
                {next_query}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::ui::button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}
