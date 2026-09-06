#![allow(
    clippy::field_scoped_visibility_modifiers,
    clippy::same_name_method,
    reason = "Leptos emits sibling props fields and builder methods with framework-defined visibility and names from the single component in this module"
)]

use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from sibling table modules"
)]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Leptos props own page data so the generated component factory can move it across reactive render closures"
)]
pub(crate) fn AdminPagination(
    admin_frontend_path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
    admin_csr_query: super::admin_csr_query::AdminCsrQuery,
    admin_page_total: server_admin_contract::admin_page_total::AdminPageTotal,
) -> impl leptos::prelude::IntoView {
    let range = crate::admin_page_range::AdminPageRange::new(
        admin_csr_query.offset(),
        admin_csr_query.limit(),
        admin_page_total,
    );
    let total_value = u64::from(admin_page_total);
    let limit = u16::from(admin_csr_query.limit()).to_string();
    let page_size_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_csr_query.search(),
        admin_csr_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Csr(
            admin_csr_query.direction().cloned(),
        ),
        admin_csr_query.limit(),
    );
    let previous_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_csr_query.search(),
        admin_csr_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Csr(
            admin_csr_query.direction().cloned(),
        ),
        admin_csr_query.limit(),
    );
    let next_query = crate::admin_table_query_hidden_inputs::admin_table_query_hidden_inputs(
        admin_csr_query.search(),
        admin_csr_query.sort(),
        &crate::admin_table_query_direction::AdminTableQueryDirection::Csr(
            admin_csr_query.direction().cloned(),
        ),
        admin_csr_query.limit(),
    );
    leptos::view! {
        <singlestage::Pagination attr:data-name="Pagination" attr:aria-label="Table pages" class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
            <singlestage::PaginationContent class="contents">
            <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=admin_frontend_path.get()>
                {page_size_query}
                <input type="hidden" name="offset" value="0" />
                <crate::admin_input_group::AdminInputGroup>
                    <crate::admin_field::AdminField admin_field_label="Rows"><crate::admin_input::AdminInput admin_input_name="limit" admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit /></crate::admin_field::AdminField>
                    <crate::admin_button::AdminButton>"Apply"</crate::admin_button::AdminButton>
                </crate::admin_input_group::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=admin_frontend_path.get()>
                {previous_query}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.previous_disabled())>"Previous"</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), total_value)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=admin_frontend_path.get()>
                {next_query}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.next_disabled())>"Next"</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}
