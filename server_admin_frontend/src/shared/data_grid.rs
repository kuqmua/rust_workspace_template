#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "shared Leptos renderers stay adjacent to their field metadata; view expansion requires attribute traits, consumes converted query values, and each target uses the shared renderer once"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes, OnAttribute,
};

pub(crate) fn admin_data_table_grid(
    view: &server_admin_contract::AdminDataTableView,
    active_field: Option<&server_admin_contract::AdminFilterField>,
    active_operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    active_value: Option<&server_admin_contract::AdminFilterValue>,
    active_end: Option<&server_admin_contract::AdminFilterValue>,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let table_path = view.table().frontend_path();
    let action = table_path.to_string();
    let supports_filters = bool::from(view.table().supports_filters());
    let limit = u16::from(limit).to_string();
    let active_field = active_field.map(ToString::to_string);
    let active_operation = active_operation.map(ToString::to_string);
    let active_value = active_value.map(ToString::to_string);
    let active_end = active_end.map(ToString::to_string);
    let clear_href = table_path.to_string();
    leptos::view! {
        <div class="table-scroll"><table>
            <thead><tr>{view.columns().iter().map(|column| {
                let field = column.name().to_string();
                let label = column.label().to_string();
                let filter_count = column.filters().len().to_string();
                let input_type = match column.input_kind() {
                    server_admin_contract::AdminDataInputKind::Date => str_constants::HTML_DATE_INPUT_TYPE,
                    server_admin_contract::AdminDataInputKind::DateTime => str_constants::HTML_DATETIME_LOCAL_INPUT_TYPE,
                    server_admin_contract::AdminDataInputKind::Number => str_constants::HTML_NUMBER_INPUT_TYPE,
                    server_admin_contract::AdminDataInputKind::Time => str_constants::HTML_TIME_INPUT_TYPE,
                    server_admin_contract::AdminDataInputKind::Checkbox
                    | server_admin_contract::AdminDataInputKind::Text
                    | server_admin_contract::AdminDataInputKind::Uuid => str_constants::HTML_TEXT_INPUT_TYPE,
                };
                let is_active_field = active_field.as_deref() == Some(field.as_str());
                let filter_label = format!("Filter {label}");
                let filter_title = format!("Filter by {label}");
                let selected_operation = leptos::prelude::RwSignal::new(
                    is_active_field
                        .then(|| active_operation.clone())
                        .flatten()
                        .unwrap_or_else(|| {
                            column
                                .filters()
                                .first()
                                .map(|filter| {
                                    server_admin_contract::AdminFilterOperationKey::from(
                                        filter.operation(),
                                    )
                                    .to_string()
                                })
                                .unwrap_or_default()
                        }),
                );
                leptos::view! {
                    <th data-field=field data-filter-count=filter_count>
                        <div class="table-column-heading">
                            <span>{label}</span>
                            {(supports_filters && !column.filters().is_empty()).then(|| leptos::prelude::IntoAny::into_any(leptos::view! {
                                <details class="table-column-filter">
                                    <summary class=("active", is_active_field) aria-label=filter_label.clone()><span class="table-filter-open-label">"Filter"</span><span class="table-filter-close-label">"Close"</span></summary>
                                    <div class="table-filter-operations" role="dialog" aria-modal="true" aria-label=filter_label>
                                        <div class="table-filter-header"><h2>{filter_title}</h2></div>
                                        <form class="table-filter-form" method="get" action=action.clone()>
                                            <input type="hidden" name="filter_field" value=field.clone() />
                                            <input type="hidden" name="limit" value=limit.clone() />
                                            <input type="hidden" name="offset" value="0" />
                                            <div class="table-filter-options">
                                                {column.filters().iter().map(|filter| {
                                                    let operation = filter.operation();
                                                    let operation_key = server_admin_contract::AdminFilterOperationKey::from(operation).to_string();
                                                    let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                                                    let value = is_active.then(|| active_value.clone()).flatten().unwrap_or_default();
                                                    let end = is_active.then(|| active_end.clone()).flatten().unwrap_or_default();
                                                    let needs_value = bool::from(filter.requires_value());
                                                    let needs_end = bool::from(filter.requires_end());
                                                    let checked_operation = operation_key.clone();
                                                    let changed_operation = operation_key.clone();
                                                    let disabled_value_operation = operation_key.clone();
                                                    let disabled_end_operation = operation_key.clone();
                                                    leptos::prelude::IntoAny::into_any(leptos::view! {
                                                        <div class="table-filter-option">
                                                            <label class="table-filter-operation-label">
                                                                <input
                                                                    type="radio"
                                                                    name="filter_operation"
                                                                    value=operation_key
                                                                    checked=move || leptos::prelude::Get::get(&selected_operation) == checked_operation
                                                                    on:change=move |_event| leptos::prelude::Set::set(&selected_operation, changed_operation.clone())
                                                                />
                                                                <span>{format!("{operation:?}")}</span>
                                                            </label>
                                                            {needs_value.then(|| {
                                                                let value_label = if needs_end { "Start" } else { "Value" };
                                                                let value_placeholder = needs_end.then_some(value_label);
                                                                leptos::prelude::IntoAny::into_any(leptos::view! {
                                                                    <label class="table-filter-input-label">
                                                                        <span>{value_label}</span>
                                                                        <input
                                                                            name="filter_value"
                                                                            type=input_type
                                                                            value=value
                                                                            placeholder=value_placeholder
                                                                            required
                                                                            disabled=move || leptos::prelude::Get::get(&selected_operation) != disabled_value_operation
                                                                        />
                                                                    </label>
                                                                })
                                                            })}
                                                            {needs_end.then(|| leptos::prelude::IntoAny::into_any(leptos::view! {
                                                                <label class="table-filter-input-label">
                                                                    <span>"End"</span>
                                                                    <input
                                                                        name="filter_end"
                                                                        type=input_type
                                                                        value=end
                                                                        placeholder="End"
                                                                        required
                                                                        disabled=move || leptos::prelude::Get::get(&selected_operation) != disabled_end_operation
                                                                    />
                                                                </label>
                                                            }))}
                                                        </div>
                                                    })
                                                }).collect::<Vec<_>>()}
                                            </div>
                                            <button type="submit">"Apply"</button>
                                        </form>
                                        {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                                    </div>
                                </details>
                            }))}
                        </div>
                    </th>
                }
            }).collect::<Vec<_>>()}</tr></thead>
            <tbody>{view.items().iter().map(|row| leptos::view! {
                <tr>{row.values().iter().enumerate().map(|(index, value)| {
                    let column = view.columns().get(index);
                    let label = column.map_or_else(String::new, |item| item.label().to_string());
                    let field = column.map_or_else(String::new, |item| item.name().to_string());
                    let numeric = column.is_some_and(|item| matches!(item.input_kind(), server_admin_contract::AdminDataInputKind::Number));
                    leptos::view! { <td class=("numeric-cell", numeric) data-field=field data-label=label>{value.to_string()}</td> }
                }).collect::<Vec<_>>()}</tr>
            }).collect::<Vec<_>>()}</tbody>
        </table></div>
    }
}
