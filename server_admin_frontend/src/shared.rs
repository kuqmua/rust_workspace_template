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

#[derive(Clone, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(crate) struct AdminSettingInputValue(Box<str>);

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct AdminSettingDisabled(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
struct AdminSettingRequired(bool);
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct LeptosAdminSettingSignal(leptos::prelude::RwSignal<String>);
impl LeptosAdminSettingSignal {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn value(self) -> AdminSettingInputValue {
        AdminSettingInputValue::from(leptos::prelude::Get::get(&self.0).into_boxed_str())
    }
}

pub(crate) fn admin_setting_input(
    field: server_admin_contract::AdminSetting,
    value: LeptosAdminSettingSignal,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    let spec = field.spec();
    let label = spec.label().as_ref().to_owned();
    let name = spec.name().as_ref().to_owned();
    let required = bool::from(spec.required());
    let disabled = bool::from(disabled);
    let value = value.0;
    match spec.input_kind() {
        server_admin_contract::AdminSettingInputKind::Text
        | server_admin_contract::AdminSettingInputKind::Url => {
            let input_type = match spec.input_kind() {
                server_admin_contract::AdminSettingInputKind::Url => {
                    str_constants::HTML_URL_INPUT_TYPE
                }
                server_admin_contract::AdminSettingInputKind::Text
                | server_admin_contract::AdminSettingInputKind::TextArea => {
                    str_constants::HTML_TEXT_INPUT_TYPE
                }
            };
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <label><span>{label}</span><input
                    name=name
                    type=input_type
                    required=required
                    disabled=disabled
                    value=leptos::prelude::Get::get(&value)
                    on:input=move |event| leptos::prelude::Set::set(
                        &value,
                        leptos::prelude::event_target_value(&event),
                    )
                /></label>
            })
        }
        server_admin_contract::AdminSettingInputKind::TextArea => {
            leptos::prelude::IntoAny::into_any(leptos::view! {
                <label><span>{label}</span><textarea
                    name=name
                    required=required
                    disabled=disabled
                    on:input=move |event| leptos::prelude::Set::set(
                        &value,
                        leptos::prelude::event_target_value(&event),
                    )
                >{leptos::prelude::Get::get(&value)}</textarea></label>
            })
        }
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminSettingsFormSignals(
    [LeptosAdminSettingSignal; server_admin_contract::AdminSetting::COUNT],
);
impl AdminSettingsFormSignals {
    pub(crate) fn new(values: &AdminSettingsFormValues) -> Self {
        Self::from(server_admin_contract::AdminSetting::ALL.map(|setting| {
            LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.get(setting).as_ref().to_owned(),
            ))
        }))
    }
    pub(crate) const fn get(
        self,
        setting: server_admin_contract::AdminSetting,
    ) -> LeptosAdminSettingSignal {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        self.0[setting.index()]
    }
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn optional_settings_to_clear(
        self,
    ) -> Result<
        server_admin_contract::AdminOptionalSettings,
        server_admin_contract::AdminCollectionError,
    > {
        let values = server_admin_contract::AdminSetting::ALL
            .into_iter()
            .filter_map(|setting| match setting.spec().optionality() {
                server_admin_contract::AdminSettingOptionality::Clearable(optional)
                    if self.get(setting).value().as_ref().is_empty() =>
                {
                    Some(optional)
                }
                server_admin_contract::AdminSettingOptionality::Clearable(_)
                | server_admin_contract::AdminSettingOptionality::Required => None,
            })
            .collect::<Vec<_>>();
        server_admin_contract::AdminOptionalSettings::try_from(values)
    }
}

pub(crate) fn admin_setting_inputs(
    signals: AdminSettingsFormSignals,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {server_admin_contract::AdminSetting::ALL.into_iter().map(|setting| {
            admin_setting_input(setting, signals.get(setting), disabled)
        }).collect::<Vec<_>>()}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AdminSettingsFormValues(
    [AdminSettingInputValue; server_admin_contract::AdminSetting::COUNT],
);
impl From<&server_admin_contract::AdminSettingsView> for AdminSettingsFormValues {
    fn from(value: &server_admin_contract::AdminSettingsView) -> Self {
        fn optional<Value>(value: Option<&Value>) -> AdminSettingInputValue
        where
            Value: AsRef<str>,
        {
            AdminSettingInputValue::from(
                value
                    .map(|item| item.as_ref().to_owned())
                    .unwrap_or_default()
                    .into_boxed_str(),
            )
        }
        Self(server_admin_contract::AdminSetting::ALL.map(|setting| {
            match setting {
                server_admin_contract::AdminSetting::DefaultRoute => AdminSettingInputValue::from(
                    value
                        .default_admin_route()
                        .as_ref()
                        .to_owned()
                        .into_boxed_str(),
                ),
                server_admin_contract::AdminSetting::MainLogo => optional(value.main_logo()),
                server_admin_contract::AdminSetting::OrganizationContacts => {
                    optional(value.organization_contacts())
                }
                server_admin_contract::AdminSetting::OrganizationName => {
                    optional(value.organization_name())
                }
                server_admin_contract::AdminSetting::PrimaryColor => {
                    optional(value.primary_color())
                }
                server_admin_contract::AdminSetting::SiteName => AdminSettingInputValue::from(
                    value.site_name().as_ref().to_owned().into_boxed_str(),
                ),
                server_admin_contract::AdminSetting::SupportUrl => optional(value.support_url()),
                server_admin_contract::AdminSetting::TabTitle => optional(value.tab_title()),
            }
        }))
    }
}
impl AdminSettingsFormValues {
    pub(crate) const fn get(
        &self,
        setting: server_admin_contract::AdminSetting,
    ) -> &AdminSettingInputValue {
        #[allow(
            clippy::indexing_slicing,
            reason = "UnitEnumIndex generates a total index below AdminSetting::COUNT"
        )]
        &self.0[setting.index()]
    }
}

#[derive(Clone, Copy, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct AdminPageNavDisabled(bool);
#[derive(Clone, Copy, Debug)]
pub(crate) struct AdminPageRange {
    end: server_admin_contract::AdminPageTotal,
    next_disabled: AdminPageNavDisabled,
    next_offset: server_admin_contract::AdminPageOffset,
    previous_disabled: AdminPageNavDisabled,
    previous_offset: server_admin_contract::AdminPageOffset,
    start: server_admin_contract::AdminPageTotal,
}
impl AdminPageRange {
    pub(crate) fn new(
        offset: server_admin_contract::AdminPageOffset,
        limit: server_admin_contract::AdminPageLimit,
        total: server_admin_contract::AdminPageTotal,
    ) -> Self {
        let offset_value = u32::from(offset);
        let limit_value = u16::from(limit);
        let total_value = u64::from(total);
        let previous_offset = offset_value.saturating_sub(u32::from(limit_value));
        let next_offset = offset_value.saturating_add(u32::from(limit_value));
        Self {
            end: server_admin_contract::AdminPageTotal::from(
                u64::from(offset_value)
                    .saturating_add(u64::from(limit_value))
                    .min(total_value),
            ),
            next_disabled: AdminPageNavDisabled::from(u64::from(next_offset) >= total_value),
            next_offset: server_admin_contract::AdminPageOffset::from(next_offset),
            previous_disabled: AdminPageNavDisabled::from(offset_value == 0u32),
            previous_offset: server_admin_contract::AdminPageOffset::from(previous_offset),
            start: server_admin_contract::AdminPageTotal::from(
                u64::from(offset_value)
                    .saturating_add(1u64)
                    .min(total_value),
            ),
        }
    }
    pub(crate) const fn end(self) -> server_admin_contract::AdminPageTotal {
        self.end
    }
    pub(crate) const fn next_disabled(self) -> AdminPageNavDisabled {
        self.next_disabled
    }
    pub(crate) const fn next_offset(self) -> server_admin_contract::AdminPageOffset {
        self.next_offset
    }
    pub(crate) const fn previous_disabled(self) -> AdminPageNavDisabled {
        self.previous_disabled
    }
    pub(crate) const fn previous_offset(self) -> server_admin_contract::AdminPageOffset {
        self.previous_offset
    }
    pub(crate) const fn start(self) -> server_admin_contract::AdminPageTotal {
        self.start
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum AdminTableFilterDirection {
    Asc,
    Desc,
    #[cfg(target_arch = "wasm32")]
    Other,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum AdminTableFilterPresentation {
    #[cfg(target_arch = "wasm32")]
    Csr,
    #[cfg(not(target_arch = "wasm32"))]
    Ssr,
}

impl AdminTableFilterDirection {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_csr(value: Option<&server_admin_contract::AdminText>) -> Self {
        match value.map(|direction| direction.as_ref().as_str()) {
            None | Some(str_constants::ASC_ALT) => Self::Asc,
            Some(str_constants::DESC_ALT) => Self::Desc,
            Some(_) => Self::Other,
        }
    }
}

impl From<server_admin_contract::AdminSortDirection> for AdminTableFilterDirection {
    fn from(value: server_admin_contract::AdminSortDirection) -> Self {
        match value {
            server_admin_contract::AdminSortDirection::Asc => Self::Asc,
            server_admin_contract::AdminSortDirection::Desc => Self::Desc,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AdminTableQueryDirection {
    #[cfg(target_arch = "wasm32")]
    Csr(Option<server_admin_contract::AdminText>),
    #[cfg(not(target_arch = "wasm32"))]
    Ssr(server_admin_contract::AdminSortDirection),
}

pub(crate) fn admin_table_query_hidden_inputs(
    search: &server_admin_contract::AdminTableSearch,
    sort: &server_admin_contract::AdminTableSortKey,
    direction: &AdminTableQueryDirection,
    limit: server_admin_contract::AdminPageLimit,
) -> impl leptos::prelude::IntoView + use<> {
    let search = search.as_ref().to_owned();
    let sort = sort.as_ref().to_owned();
    let direction = match direction {
        #[cfg(target_arch = "wasm32")]
        AdminTableQueryDirection::Csr(value) => {
            value.as_ref().map(ToString::to_string).unwrap_or_default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        AdminTableQueryDirection::Ssr(value) => value.as_ref().to_owned(),
    };
    let limit = u16::from(limit).to_string();
    leptos::view! {
        <input type="hidden" name="search" value=search /><input type="hidden" name="sort" value=sort />
        <input type="hidden" name="direction" value=direction /><input type="hidden" name="limit" value=limit />
    }
}

pub(crate) fn admin_table_filters(
    action: server_admin_contract::AdminFrontendPath,
    search: &server_admin_contract::AdminTableSearch,
    sort: &server_admin_contract::AdminTableSortKey,
    direction: AdminTableFilterDirection,
    limit: server_admin_contract::AdminPageLimit,
    sort_fields: &[server_admin_contract::AdminTableSortField],
    presentation: AdminTableFilterPresentation,
) -> impl leptos::prelude::IntoView + use<> {
    let search = search.as_ref().to_owned();
    let sort = sort.as_ref().to_owned();
    let ascending = matches!(direction, AdminTableFilterDirection::Asc);
    let descending = matches!(direction, AdminTableFilterDirection::Desc);
    let editable_limit = match presentation {
        #[cfg(target_arch = "wasm32")]
        AdminTableFilterPresentation::Csr => true,
        #[cfg(not(target_arch = "wasm32"))]
        AdminTableFilterPresentation::Ssr => false,
    };
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
            <label><span>"Direction"</span><select name="direction"><option value="asc" selected=ascending>"Ascending"</option><option value="desc" selected=descending>"Descending"</option></select></label>
            {editable_limit.then(|| leptos::view! {
                <input name="limit" type="number" min=server_admin_contract::AdminPageLimit::MIN max=server_admin_contract::AdminPageLimit::MAX value=limit.clone() />
            })}
            {(!editable_limit).then(|| leptos::view! {
                <input name="limit" type="hidden" value=limit />
            })}
            <input name="offset" type="hidden" value="0" /><button type="submit">"Apply"</button>
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

#[cfg(test)]
mod tests {
    fn page_range(offset: u32, limit: u16, total: u64) -> super::AdminPageRange {
        let Ok(limit) = server_admin_contract::AdminPageLimit::try_from(limit) else {
            panic!("1543efb0");
        };
        super::AdminPageRange::new(
            server_admin_contract::AdminPageOffset::from(offset),
            limit,
            server_admin_contract::AdminPageTotal::from(total),
        )
    }

    #[test]
    fn page_range_handles_empty_and_first_pages() {
        let empty = page_range(0u32, 20u16, 0u64);
        assert_eq!(u64::from(empty.start()), 0u64);
        assert_eq!(u64::from(empty.end()), 0u64);
        assert!(bool::from(empty.previous_disabled()));
        assert!(bool::from(empty.next_disabled()));

        let first = page_range(0u32, 20u16, 41u64);
        assert_eq!(u64::from(first.start()), 1u64);
        assert_eq!(u64::from(first.end()), 20u64);
        assert_eq!(u32::from(first.next_offset()), 20u32);
        assert!(!bool::from(first.next_disabled()));
    }

    #[test]
    fn page_range_handles_partial_out_of_range_and_overflow_pages() {
        let partial = page_range(40u32, 20u16, 41u64);
        assert_eq!(u64::from(partial.start()), 41u64);
        assert_eq!(u64::from(partial.end()), 41u64);
        assert_eq!(u32::from(partial.previous_offset()), 20u32);
        assert!(bool::from(partial.next_disabled()));

        let out_of_range = page_range(80u32, 20u16, 41u64);
        assert_eq!(u64::from(out_of_range.start()), 41u64);
        assert_eq!(u64::from(out_of_range.end()), 41u64);

        let overflow = page_range(u32::MAX, 100u16, u64::MAX);
        assert_eq!(u32::from(overflow.next_offset()), u32::MAX);
        assert_eq!(u64::from(overflow.start()), u64::from(u32::MAX) + 1u64);
        assert_eq!(u64::from(overflow.end()), u64::from(u32::MAX) + 100u64);
    }
}
