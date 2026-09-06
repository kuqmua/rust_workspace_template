#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminDataGrid(
    admin_bool: server_admin_contract::admin_bool::AdminBool,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
    admin_data_table_view: server_admin_contract::admin_data_table_view::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    let can_export = admin_data_table_view.table()
        == server_admin_contract::admin_data_table::AdminDataTable::AuditLog
        && bool::from(admin_bool);
    let supports_filters = bool::from(admin_data_table_view.table().supports_filters());
    let table_path = admin_data_table_view.table().frontend_path();
    let total = admin_data_table_view.total();
    let limit = u16::from(admin_csr_query.limit());
    let limit_text = limit.to_string();
    let range = crate::admin_page_range::AdminPageRange::new(
        admin_csr_query.offset(),
        admin_csr_query.limit(),
        total,
    );
    let filter_field = supports_filters
        .then_some(admin_csr_query.filter_field())
        .flatten();
    let filter_operation = supports_filters
        .then_some(admin_csr_query.filter_operation())
        .flatten();
    let filter_value = supports_filters
        .then_some(admin_csr_query.filter_value())
        .flatten();
    let filter_end = supports_filters
        .then_some(admin_csr_query.filter_end())
        .flatten();
    let grid = crate::admin_data_table_grid::admin_data_table_grid(
        &admin_data_table_view,
        admin_csr_query.filter_field(),
        admin_csr_query.filter_operation(),
        admin_csr_query.filter_value(),
        admin_csr_query.filter_end(),
        admin_csr_query.limit(),
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
            {can_export.then(|| leptos::view! {
                <crate::admin_audit_download::AdminAuditDownload admin_page_limit=admin_csr_query.limit() admin_page_offset=admin_csr_query.offset() />
            })}
            {grid}
            <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
                <singlestage::PaginationContent class="contents">
                <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=page_size_action>
                    {page_size_filter}
                    <input type="hidden" name="offset" value="0" />
                    <crate::admin_input_group::AdminInputGroup>
                        <crate::admin_field::AdminField admin_field_label="Rows"><crate::admin_input::AdminInput admin_input_name="limit" admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::admin_field::AdminField>
                        <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                    </crate::admin_input_group::AdminInputGroup>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                    <input type="hidden" name="limit" value=previous_limit />
                    {previous_filter}
                    <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.previous_disabled())>{constants_str::ADMIN_BUTTON_PREVIOUS}</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), u64::from(total))}</span></singlestage::PaginationItem>
                <singlestage::PaginationItem class="contents"><form method="get" action=next_action>
                    <input type="hidden" name="limit" value=limit_text />
                    {next_filter}
                    <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.next_disabled())>{constants_str::ADMIN_BUTTON_NEXT}</crate::admin_button::AdminButton>
                </form></singlestage::PaginationItem>
                </singlestage::PaginationContent>
            </singlestage::Pagination>
        </section>
    }
}
