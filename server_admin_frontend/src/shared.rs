#![allow(
    clippy::shadow_reuse,
    clippy::single_call_fn,
    clippy::unused_trait_names,
    reason = "Leptos view expansion requires attribute traits, consumes converted query values, and each target uses the shared renderer once"
)]

use leptos::prelude::{
    AriaAttributes, ClassAttribute, CustomAttribute, ElementChild, GlobalAttributes,
};

#[cfg(target_arch = "wasm32")]
pub(crate) fn admin_table_filters(
    action: server_admin_contract::AdminFrontendPath,
    search: &server_admin_contract::AdminTableSearch,
    sort: &server_admin_contract::AdminTableSortKey,
    direction: Option<&server_admin_contract::AdminText>,
    limit: server_admin_contract::AdminPageLimit,
    sort_fields: &[server_admin_contract::AdminTableSortField],
) -> impl leptos::prelude::IntoView + use<> {
    let search = search.as_ref().to_owned();
    let sort = sort.as_ref().to_owned();
    let direction =
        direction.map_or_else(|| String::from(str_constants::ASC_ALT), ToString::to_string);
    let limit = u16::from(limit).to_string();
    leptos::view! {
        <form class="table-tools" method="get" action=action.get()>
            <label><span>"Search"</span><input name="search" value=search /></label>
            <label><span>"Sort"</span><select name="sort">
                <option value="" selected=sort.is_empty()>"Default"</option>
                {sort_fields.iter().copied().map(|field| {
                    let key = field.key().as_ref().to_owned();
                    let selected = sort == key;
                    leptos::view! { <option value=key selected=selected>{field.label().as_ref().to_owned()}</option> }
                }).collect::<Vec<_>>()}
            </select></label>
            <select name="direction"><option value="asc" selected=direction == "asc">"Ascending"</option><option value="desc" selected=direction == "desc">"Descending"</option></select>
            <input name="limit" type="number" min="1" max="100" value=limit /><input name="offset" type="hidden" value="0" /><button type="submit">"Apply"</button>
        </form>
    }
}

pub(crate) fn admin_filter_hidden_inputs(
    field: Option<&server_admin_contract::AdminFilterField>,
    operation: Option<&server_admin_contract::AdminFilterOperationKey>,
    value: Option<&server_admin_contract::AdminFilterValue>,
    end: Option<&server_admin_contract::AdminFilterValue>,
) -> impl leptos::prelude::IntoView + use<> {
    let field = field.map(ToString::to_string);
    let operation = operation.map(ToString::to_string);
    let value = value.map(ToString::to_string);
    let end = end.map(ToString::to_string);
    leptos::view! {
        {field.map(|field_text| leptos::view! { <input type="hidden" name="filter_field" value=field_text /> })}
        {operation.map(|operation_text| leptos::view! { <input type="hidden" name="filter_operation" value=operation_text /> })}
        {value.map(|filter_text| leptos::view! { <input type="hidden" name="filter_value" value=filter_text /> })}
        {end.map(|end_text| leptos::view! { <input type="hidden" name="filter_end" value=end_text /> })}
    }
}

pub(crate) fn admin_audit_hidden_inputs(
    action: Option<&server_admin_contract::AdminText>,
    resource: Option<&server_admin_contract::AdminText>,
    resource_id: Option<&server_admin_contract::AdminText>,
    user_login: Option<&server_admin_contract::AdminLogin>,
) -> impl leptos::prelude::IntoView + use<> {
    let action = action.map(ToString::to_string);
    let resource = resource.map(ToString::to_string);
    let resource_id = resource_id.map(ToString::to_string);
    let user_login = user_login.map(ToString::to_string);
    leptos::view! {
        {action.map(|value| leptos::view! { <input type="hidden" name="action" value=value /> })}
        {resource.map(|value| leptos::view! { <input type="hidden" name="resource" value=value /> })}
        {resource_id.map(|value| leptos::view! { <input type="hidden" name="resource_id" value=value /> })}
        {user_login.map(|value| leptos::view! { <input type="hidden" name="user_login" value=value /> })}
    }
}

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
                leptos::view! {
                    <th data-field=field.clone() data-filter-count=filter_count>
                        <div class="table-column-heading">
                            <span>{label}</span>
                            {(supports_filters && !column.filters().is_empty()).then(|| leptos::view! {
                                <details class="table-column-filter" open=is_active_field>
                                    <summary class=("active", is_active_field) aria-label=filter_label.clone()><span class="table-filter-open-label">"Filter"</span><span class="table-filter-close-label">"Close"</span></summary>
                                    <div class="table-filter-operations" role="dialog" aria-modal="true" aria-label=filter_label>
                                        <h2>{filter_title}</h2>
                                        {is_active_field.then(|| leptos::view! { <a class="table-filter-clear" href=clear_href.clone()>"Clear"</a> })}
                                        {column.filters().iter().map(|filter| {
                                            let operation = filter.operation();
                                            let operation_key = server_admin_contract::AdminFilterOperationKey::from(operation).to_string();
                                            let is_active = is_active_field && active_operation.as_deref() == Some(operation_key.as_str());
                                            let value = is_active.then(|| active_value.clone()).flatten().unwrap_or_default();
                                            let end = is_active.then(|| active_end.clone()).flatten().unwrap_or_default();
                                            let needs_value = bool::from(filter.requires_value());
                                            let needs_end = bool::from(filter.requires_end());
                                            leptos::view! {
                                                <form class="table-filter-form" method="get" action=action.clone()>
                                                    <input type="hidden" name="filter_field" value=field.clone() />
                                                    <input type="hidden" name="filter_operation" value=operation_key />
                                                    <input type="hidden" name="limit" value=limit.clone() />
                                                    <input type="hidden" name="offset" value="0" />
                                                    <span>{format!("{operation:?}")}</span>
                                                    {needs_value.then(|| leptos::view! { <input name="filter_value" type=input_type value=value required /> })}
                                                    {needs_end.then(|| leptos::view! { <input name="filter_end" type=input_type value=end required /> })}
                                                    <button type="submit">"Apply"</button>
                                                </form>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </details>
                            })}
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
