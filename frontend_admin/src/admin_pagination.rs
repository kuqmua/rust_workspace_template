use leptos::prelude::{AddAnyAttr, ClassAttribute, ElementChild};

#[allow(
    clippy::single_call_fn,
    reason = "SSR and CSR pagination share this renderer through mutually exclusive target-specific call sites"
)]
pub(crate) fn admin_pagination(
    admin_data_table_frontend_path: &server_admin_contract::admin_data_table_frontend_path::AdminDataTableFrontendPath,
    admin_pagination_query: &crate::admin_pagination_query::AdminPaginationQuery<'_>,
    admin_page_total: server_admin_contract::admin_page_total::AdminPageTotal,
) -> impl leptos::prelude::IntoView + use<> {
    let action = admin_data_table_frontend_path.as_ref().to_owned();
    let page_size_action = action.clone();
    let previous_action = action.clone();
    let limit = u16::from(admin_pagination_query.limit());
    let range = crate::admin_page_range::AdminPageRange::new(
        admin_pagination_query.offset(),
        admin_pagination_query.limit(),
        admin_page_total,
    );
    let page_size_inputs = admin_pagination_query.hidden_inputs(None);
    let previous_inputs =
        admin_pagination_query.hidden_inputs(Some(admin_pagination_query.limit()));
    let next_inputs = admin_pagination_query.hidden_inputs(Some(admin_pagination_query.limit()));
    leptos::view! {
        <singlestage::Pagination attr:data-name="Pagination" attr:aria-label=constants_str::ADMIN_UI_TABLE_PAGES class="table-pagination mx-auto flex w-full items-center justify-center gap-2">
            <singlestage::PaginationContent class="contents">
            <singlestage::PaginationItem class="contents"><form class="table-page-size" method="get" action=page_size_action>
                {page_size_inputs}
                <input type="hidden" name="offset" value="0" />
                <crate::admin_input_group::AdminInputGroup>
                    <crate::admin_field::AdminField admin_field_label=constants_str::ADMIN_UI_ROWS><crate::admin_input::AdminInput admin_input_name="limit" admin_input_kind=crate::admin_input_kind::AdminInputKind::Number min=server_admin_contract::admin_page_limit::AdminPageLimit::MIN max=server_admin_contract::admin_page_limit::AdminPageLimit::MAX initial_value=limit.to_string() /></crate::admin_field::AdminField>
                    <crate::admin_button::AdminButton>{constants_str::ADMIN_BUTTON_APPLY}</crate::admin_button::AdminButton>
                </crate::admin_input_group::AdminInputGroup>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=previous_action>
                {previous_inputs}
                <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.previous_disabled())>{constants_str::ADMIN_BUTTON_PREVIOUS}</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><span>{format!("{}_{}_{}_{}_{}", u64::from(range.start()), constants_str::ADMIN_UI_TO, u64::from(range.end()), constants_str::ADMIN_UI_OF, admin_page_total)}</span></singlestage::PaginationItem>
            <singlestage::PaginationItem class="contents"><form method="get" action=action>
                {next_inputs}
                <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><crate::admin_button::AdminButton admin_button_variant=crate::admin_button_variant::AdminButtonVariant::Secondary bool=bool::from(range.next_disabled())>{constants_str::ADMIN_BUTTON_NEXT}</crate::admin_button::AdminButton>
            </form></singlestage::PaginationItem>
            </singlestage::PaginationContent>
        </singlestage::Pagination>
    }
}
