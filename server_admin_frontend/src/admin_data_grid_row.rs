#![allow(
    clippy::unused_trait_names,
    reason = "the Leptos data row is composed once by the shared grid"
)]

pub(super) fn admin_data_grid_row(
    view: &server_admin_contract::domain_types::AdminDataTableView,
    row: &server_admin_contract::domain_types::AdminDataRow,
) -> impl leptos::prelude::IntoView + use<> {
    let cells = row.values().iter().enumerate().map(|(index, value)| {
        let column = view.columns().get(index);
        let label = column.map_or_else(String::new, |item| item.label().to_string());
        let field = column.map_or_else(String::new, |item| item.name().to_string());
        let numeric = column.is_some_and(|item| matches!(item.input_kind(), frontend_contract::domain_types::InputKind::Number));
        let value_text = value.to_string();
        leptos::view! { <crate::domain_types::with_owner::tables::table_cell::TableCell data_label=label data_field=field class=if numeric { "numeric-cell" } else { "" }>{value_text}</crate::domain_types::with_owner::tables::table_cell::TableCell> }
    }).collect::<Vec<_>>();
    leptos::view! {
        <crate::domain_types::with_owner::tables::table_row::TableRow>{cells}</crate::domain_types::with_owner::tables::table_row::TableRow>
    }
}
