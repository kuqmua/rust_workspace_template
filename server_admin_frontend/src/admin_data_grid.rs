use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(crate) fn AdminDataGrid(
    query: super::admin_csr_query::AdminCsrQuery,
    view: server_admin_contract::admin_data_table_view::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    let supports_filters = bool::from(view.table().supports_filters());
    let table_path = view.table().frontend_path();
    let total = view.total();
    let limit = u16::from(query.limit);
    let limit_text = limit.to_string();
    let range = crate::admin_page_range::AdminPageRange::new(query.offset, query.limit, total);
    let filter_field = supports_filters
        .then_some(query.filter_field.as_ref())
        .flatten();
    let filter_operation = supports_filters
        .then_some(query.filter_operation.as_ref())
        .flatten();
    let filter_value = supports_filters
        .then_some(query.filter_value.as_ref())
        .flatten();
    let filter_end = supports_filters
        .then_some(query.filter_end.as_ref())
        .flatten();
    let grid = crate::domain_types::shared::admin_data_table_grid::admin_data_table_grid(
        &view,
        query.filter_field.as_ref(),
        query.filter_operation.as_ref(),
        query.filter_value.as_ref(),
        query.filter_end.as_ref(),
        query.limit,
    );
    let page_size_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let previous_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let next_filter = crate::admin_filter_hidden_inputs::admin_filter_hidden_inputs(
        filter_field,
        filter_operation,
        filter_value,
        filter_end,
    );
    let page_size_action = table_path.as_ref().to_owned();
    let previous_action = table_path.as_ref().to_owned();
    let next_action = table_path.as_ref().to_owned();
    let previous_limit = limit_text.clone();
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {grid}
            <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
                <singlestage::PaginationContent class="contents">
                <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=page_size_action>
                    {page_size_filter}
                    <input type="hidden" name="offset" value="0" />
                    <crate::admin_input_group::AdminInputGroup>
                        <crate::admin_field::AdminField label="Rows"><crate::admin_input::AdminInput name="limit" kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::admin_field::AdminField>
                        <crate::admin_button::AdminButton>"Apply"</crate::admin_button::AdminButton>
                    </crate::admin_input_group::AdminInputGroup>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                    <input type="hidden" name="limit" value=previous_limit />
                    {previous_filter}
                    <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Secondary disabled=bool::from(range.previous_disabled())>"Previous"</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), u64::from(total))}</span></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=next_action>
                    <input type="hidden" name="limit" value=limit_text />
                    {next_filter}
                    <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton variant=crate::admin_button_variant::AdminButtonVariant::Secondary disabled=bool::from(range.next_disabled())>"Next"</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                </singlestage::PaginationContent>
            </singlestage::Pagination>
        </section>
    }
}
