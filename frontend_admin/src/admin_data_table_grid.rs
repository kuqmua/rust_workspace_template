#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos grid cells and column headings require attribute traits after macro expansion"
)]

use leptos::prelude::{ClassAttribute, ElementChild};

#[allow(
    clippy::single_call_fn,
    reason = "admin data table grid remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn admin_data_table_grid(
    admin_data_table_view: &server_admin_contract::admin_data_table_view::AdminDataTableView,
    active_field: Option<&server_admin_contract::admin_filter_field::AdminFilterField>,
    active_operation: Option<
        &server_admin_contract::admin_filter_operation_key::AdminFilterOperationKey,
    >,
    active_value: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    active_end: Option<&server_admin_contract::admin_filter_value::AdminFilterValue>,
    admin_page_limit: server_admin_contract::admin_page_limit::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let columns = admin_data_table_view
        .columns()
        .iter()
        .map(|column| {
            let field = column.name().to_string();
            let label = column.name().to_string();
            let filter_count = column.filters().len().to_string();
            let table_path = admin_data_table_view.table().frontend_path();
            let filter = (bool::from(admin_data_table_view.table().supports_filters())
                && !column.filters().is_empty())
            .then(|| {
                crate::admin_column_filter::admin_column_filter(
                    &table_path,
                    column.name(),
                    column.input_kind(),
                    column.filters(),
                    active_field,
                    active_operation,
                    active_value,
                    active_end,
                    admin_page_limit,
                )
            });
            leptos::view! {
                <crate::table_head::TableHead data_field=field data_filter_count=filter_count>
                    <div class="table-column-heading">
                        <span>{label}</span>
                        {filter}
                    </div>
                </crate::table_head::TableHead>
            }
        })
        .collect::<Vec<_>>();
    let rows = admin_data_table_view
        .items()
        .iter()
        .map(|item| {
            let cells = item
                .values()
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let column = admin_data_table_view.columns().get(index);
                    let label =
                        column.map_or_else(String::new, |column| column.name().to_string());
                    let field =
                        column.map_or_else(String::new, |column| column.name().to_string());
                    let numeric = column.is_some_and(|column| {
                        matches!(column.input_kind(), frontend_contract::input_kind::InputKind::Number)
                    });
                    let value_text = value.to_string();
                    leptos::view! { <crate::table_cell::TableCell data_label=label data_field=field class=if numeric { "numeric-cell" } else { "" }>{value_text}</crate::table_cell::TableCell> }
                })
                .collect::<Vec<_>>();
            leptos::view! {
                <crate::table_row::TableRow>{cells}<crate::table_cell::TableCell data_label=constants_str::ADMIN_UI_ACTIONS bool=true>{constants_str::EMPTY}</crate::table_cell::TableCell></crate::table_row::TableRow>
            }
        })
        .collect::<Vec<_>>();
    leptos::view! {
        <crate::table_wrapper::TableWrapper><crate::table::Table>
            <crate::table_header::TableHeader><crate::table_row::TableRow>{columns}<crate::table_head::TableHead>{constants_str::ADMIN_UI_ACTIONS}</crate::table_head::TableHead></crate::table_row::TableRow></crate::table_header::TableHeader>
            <crate::table_body::TableBody>{rows}</crate::table_body::TableBody>
        </crate::table::Table></crate::table_wrapper::TableWrapper>
    }
}
