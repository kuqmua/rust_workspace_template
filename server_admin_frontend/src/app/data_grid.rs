use leptos::prelude::{AriaAttributes, ClassAttribute, CustomAttribute, ElementChild};

#[leptos::component]
#[allow(
    unreachable_pub,
    reason = "Leptos component visibility is required for composition from the parent app module"
)]
pub(in crate::app) fn AdminDataGrid(
    query: super::query::AdminCsrQuery,
    view: server_admin_contract::AdminDataTableView,
) -> impl leptos::prelude::IntoView {
    let supports_filters = bool::from(view.table().supports_filters());
    let table_path = view.table().frontend_path();
    let total = view.total();
    let limit = u16::from(query.limit);
    let limit_text = limit.to_string();
    let range = crate::shared::pagination::AdminPageRange::new(query.offset, query.limit, total);
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
    leptos::view! {
        <section class="table-page" data-renderer="csr">
            {crate::shared::data_grid::admin_data_table_grid(
                &view,
                query.filter_field.as_ref(),
                query.filter_operation.as_ref(),
                query.filter_value.as_ref(),
                query.filter_end.as_ref(),
                query.limit,
            )}
            <nav class="table-pagination" aria-label="Table pages">
                <form class="table-page-size" method="get" action=table_path.as_ref().to_owned()>
                    {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value="0" />
                    <label><span>"Rows"</span><input name="limit" type="number" min=server_admin_contract::AdminPageLimit::MIN max=server_admin_contract::AdminPageLimit::MAX value=limit_text.clone() /></label>
                    <button type="submit">"Apply"</button>
                </form>
                <form method="get" action=table_path.as_ref().to_owned()>
                    <input type="hidden" name="limit" value=limit_text.clone() />
                    {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value=u32::from(range.previous_offset()).to_string() /><button type="submit" disabled=bool::from(range.previous_disabled())>"Previous"</button>
                </form>
                <span>{format!("{}-{} of {}", u64::from(range.start()), u64::from(range.end()), u64::from(total))}</span>
                <form method="get" action=table_path.as_ref().to_owned()>
                    <input type="hidden" name="limit" value=limit_text />
                    {crate::shared::table_filters::filter::admin_filter_hidden_inputs(filter_field, filter_operation, filter_value, filter_end)}
                    <input type="hidden" name="offset" value=u32::from(range.next_offset()).to_string() /><button type="submit" disabled=bool::from(range.next_disabled())>"Next"</button>
                </form>
            </nav>
        </section>
    }
}
