#![allow(
    unused_imports,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "table pagination and data-grid Leptos views require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

pub(super) fn table_pagination(
    page: server_admin_contract::AdminPage,
    query: &server_admin_contract::AdminTableQuery,
    total: server_admin_contract::AdminPageTotal,
    table: Option<server_admin_contract::AdminDataTable>,
    table_filter: Option<&server_admin_contract::AdminDataTableFilterQuery>,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || String::from(page.path()),
        |value| value.frontend_path().to_string(),
    );
    let limit = u16::from(query.limit());
    let range =
        crate::shared::pagination::AdminPageRange::new(query.offset(), query.limit(), total);
    let filter_operation = table_filter
        .and_then(server_admin_contract::AdminDataTableFilterQuery::operation)
        .map(server_admin_contract::AdminFilterOperationKey::from);
    let filter_field =
        table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::field);
    let filter_value =
        table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::value);
    let filter_end = table_filter.and_then(server_admin_contract::AdminDataTableFilterQuery::end);
    leptos::view! {
        <nav data-name="Pagination" class="table-pagination mx-auto flex w-full items-center justify-center gap-2" aria-label="Table pages">
            <form class="table-page-size" method="get" action=action.clone()>
                {crate::shared::table_filters::query::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::table_filters::query::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value="0" />
                <crate::ui::field::AdminField label="Rows"><crate::ui::input::AdminInput name="limit" kind=crate::ui::input::AdminInputKind::Number min=server_admin_contract::AdminPageLimit::MIN max=server_admin_contract::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::ui::field::AdminField>
                <crate::ui::button::AdminButton>"Apply"</crate::ui::button::AdminButton>
            </form>
            <form method="get" action=action.clone()>
                {crate::shared::table_filters::query::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::table_filters::query::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::ui::button::AdminButton>
            </form>
            <span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total)}</span>
            <form method="get" action=action>
                {crate::shared::table_filters::query::admin_table_query_hidden_inputs(query.search(), query.sort(), &crate::shared::table_filters::query::AdminTableQueryDirection::Ssr(query.direction()), query.limit())}
                {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation.as_ref(), filter_value, filter_end)}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::ui::button::AdminButton variant=crate::ui::button::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::ui::button::AdminButton>
            </form>
        </nav>
    }
}

pub(super) fn data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    query: &server_admin_contract::AdminDataTableQuery,
) -> impl leptos::prelude::IntoView {
    let operation = query
        .filter()
        .operation()
        .map(server_admin_contract::AdminFilterOperationKey::from);
    crate::shared::data_grid::admin_data_table_grid(
        view,
        query.filter().field(),
        operation.as_ref(),
        query.filter().value(),
        query.filter().end(),
        query.page().limit(),
    )
}
