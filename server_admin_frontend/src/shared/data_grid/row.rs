#![allow(
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "the Leptos data row is composed once by the shared grid"
)]

use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};

pub(super) fn admin_data_grid_row(
    view: &server_admin_contract::AdminDataTableView,
    row: &server_admin_contract::AdminDataRow,
) -> impl leptos::prelude::IntoView + use<> {
    leptos::view! {
        <tr data-name="TableRow" class="border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50">{row.values().iter().enumerate().map(|(index, value)| {
                    let column = view.columns().get(index);
                    let label = column.map_or_else(String::new, |item| item.label().to_string());
                    let field = column.map_or_else(String::new, |item| item.name().to_string());
                    let numeric = column.is_some_and(|item| matches!(item.input_kind(), server_admin_contract::AdminDataInputKind::Number));
                    leptos::view! { <td data-name="TableCell" class="p-4 align-middle [&:has([role=checkbox])]:pr-0 [&:has([role=checkbox])]:pl-3" class=("numeric-cell", numeric) data-field=field data-label=label>{value.to_string()}</td> }
                }).collect::<Vec<_>>()}</tr>
    }
}
