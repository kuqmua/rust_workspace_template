#![allow(
    unused_imports,
    clippy::unused_trait_names,
    reason = "table pagination and data-grid Leptos views require different attribute traits after macro expansion"
)]

use leptos::prelude::{
    AddAnyAttr, AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

pub(super) fn table_pagination(
    admin_page: server_admin_contract::admin_page::AdminPage,
    admin_table_query: &server_admin_contract::admin_table_query::AdminTableQuery,
    admin_page_total: server_admin_contract::admin_page_total::AdminPageTotal,
    table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
    table_filter: Option<
        &server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery,
    >,
) -> impl leptos::prelude::IntoView {
    let action = table.map_or_else(
        || String::from(admin_page.path()),
        |value| value.frontend_path().to_string(),
    );
    let limit = u16::from(admin_table_query.limit());
    let range = crate::admin_page_range::AdminPageRange::new(
        admin_table_query.offset(),
        admin_table_query.limit(),
        admin_page_total,
    );
    let filter_operation = table_filter
        .and_then(server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::operation)
        .map(server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey::from);
    let filter_field = table_filter.and_then(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::field,
    );
    let filter_value = table_filter.and_then(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::value,
    );
    let filter_end = table_filter.and_then(
        server_admin_contract::admin_data_table_filter_query::AdminDataTableFilterQuery::end,
    );
    let page_size_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_table_query.search(),
        admin_table_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Ssr(
            admin_table_query.direction(),
        ),
        admin_table_query.limit(),
    );
    let page_size_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation.as_ref(),
        filter_value,
        filter_end,
    );
    let previous_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_table_query.search(),
        admin_table_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Ssr(
            admin_table_query.direction(),
        ),
        admin_table_query.limit(),
    );
    let previous_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation.as_ref(),
        filter_value,
        filter_end,
    );
    let next_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_table_query.search(),
        admin_table_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Ssr(
            admin_table_query.direction(),
        ),
        admin_table_query.limit(),
    );
    let next_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
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
                <crate::admin_input_group::AdminInputGroup>
                    <crate::admin_field::AdminField admin_field_label="Rows"><crate::admin_input::AdminInput admin_input_name="limit" admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::admin_field::AdminField>
                    <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                </crate::admin_input_group::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                {previous_query}
                {previous_filter}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.previous_disabled())>{constants_str::ADMIN_BUTTON_PREVIOUS}</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), admin_page_total)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action>
                {next_query}
                {next_filter}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.next_disabled())>{constants_str::ADMIN_BUTTON_NEXT}</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}
