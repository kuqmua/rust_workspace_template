#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "table pagination and data-grid Leptos views require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

pub(super) fn table_pagination(
    page: server_admin_contract::domain_types::AdminPage,
    query: &server_admin_contract::domain_types::AdminTableQuery,
    total: server_admin_contract::domain_types::AdminPageTotal,
    table: Option<server_admin_contract::domain_types::AdminDataTable>,
    table_filter: Option<&server_admin_contract::domain_types::AdminDataTableFilterQuery>,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || String::from(page.path()),
        |value| value.frontend_path().to_string(),
    );
    let limit = u16::from(query.limit());
    let range = crate::domain_types::shared::pagination::AdminPageRange::new(
        query.offset(),
        query.limit(),
        total,
    );
    let filter_operation = table_filter
        .and_then(server_admin_contract::domain_types::AdminDataTableFilterQuery::operation)
        .map(server_admin_contract::domain_types::AdminFilterOperationKey::from);
    let filter_field = table_filter
        .and_then(server_admin_contract::domain_types::AdminDataTableFilterQuery::field);
    let filter_value = table_filter
        .and_then(server_admin_contract::domain_types::AdminDataTableFilterQuery::value);
    let filter_end =
        table_filter.and_then(server_admin_contract::domain_types::AdminDataTableFilterQuery::end);
    let page_size_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            query.search(),
            query.sort(),
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Ssr(
                query.direction(),
            ),
            query.limit(),
        );
    let page_size_filter =
        crate::domain_types::shared::table_filters::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
            filter_field,
            filter_operation.as_ref(),
            filter_value,
            filter_end,
        );
    let previous_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            query.search(),
            query.sort(),
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Ssr(
                query.direction(),
            ),
            query.limit(),
        );
    let previous_filter =
        crate::domain_types::shared::table_filters::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
            filter_field,
            filter_operation.as_ref(),
            filter_value,
            filter_end,
        );
    let next_query =
        crate::domain_types::shared::table_filters::query::admin_table_query_hidden_inputs(
            query.search(),
            query.sort(),
            &crate::domain_types::shared::table_filters::query::AdminTableQueryDirection::Ssr(
                query.direction(),
            ),
            query.limit(),
        );
    let next_filter =
        crate::domain_types::shared::table_filters::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
            filter_field,
            filter_operation.as_ref(),
            filter_value,
            filter_end,
        );
    let page_size_action = action.clone();
    let previous_action = action.clone();
    leptos::view! {
        <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
            <singlestage::PaginationContent class="contents">
            <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=page_size_action>
                {page_size_query}
                {page_size_filter}
                <input type="hidden" name="offset" value="0" />
                <crate::domain_types::with_owner::input::AdminInputGroup>
                    <crate::domain_types::with_owner::field::AdminField label="Rows"><crate::domain_types::with_owner::input::AdminInput name="limit" kind=crate::domain_types::with_owner::input::AdminInputKind::Number min=server_admin_contract::domain_types::AdminPageLimit::MIN max=server_admin_contract::domain_types::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::domain_types::with_owner::field::AdminField>
                    <crate::domain_types::with_owner::button::AdminButton>"Apply"</crate::domain_types::with_owner::button::AdminButton>
                </crate::domain_types::with_owner::input::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                {previous_query}
                {previous_filter}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::domain_types::with_owner::button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action>
                {next_query}
                {next_filter}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::domain_types::with_owner::button::AdminButton variant=crate::domain_types::with_owner::button::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::domain_types::with_owner::button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}

pub(super) fn data_table_grid(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    query: &server_admin_contract::domain_types::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = query
        .filter()
        .operation()
        .map(server_admin_contract::domain_types::AdminFilterOperationKey::from);
    crate::domain_types::shared::admin_data_table_grid::admin_data_table_grid(
        view,
        query.filter().field(),
        operation.as_ref(),
        query.filter().value(),
        query.filter().end(),
        query.page().limit(),
    )
}
