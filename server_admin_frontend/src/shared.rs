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

#[derive(Clone, Copy, Debug, newtype::AsRefStr, newtype::FromInner)]
pub(crate) struct AdminSettingAttribute(&'static str);
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

#[derive(Clone, Copy, Debug)]
pub(crate) enum AdminSettingField {
    DefaultRoute,
    MainLogo,
    OrganizationContacts,
    OrganizationName,
    PrimaryColor,
    SiteName,
    SupportUrl,
    TabTitle,
}
#[derive(Clone, Copy, Debug)]
enum AdminSettingInputKind {
    Text,
    TextArea,
    Url,
}
impl AdminSettingField {
    const fn input_kind(self) -> AdminSettingInputKind {
        match self {
            Self::OrganizationContacts => AdminSettingInputKind::TextArea,
            Self::MainLogo | Self::SupportUrl => AdminSettingInputKind::Url,
            Self::DefaultRoute
            | Self::OrganizationName
            | Self::PrimaryColor
            | Self::SiteName
            | Self::TabTitle => AdminSettingInputKind::Text,
        }
    }
    fn label(self) -> AdminSettingAttribute {
        AdminSettingAttribute::from(match self {
            Self::DefaultRoute => str_constants::ADMIN_SETTING_DEFAULT_ROUTE_LABEL,
            Self::MainLogo => str_constants::ADMIN_SETTING_MAIN_LOGO_LABEL,
            Self::OrganizationContacts => str_constants::ADMIN_SETTING_ORGANIZATION_CONTACTS_LABEL,
            Self::OrganizationName => str_constants::ADMIN_SETTING_ORGANIZATION_NAME_LABEL,
            Self::PrimaryColor => str_constants::ADMIN_SETTING_PRIMARY_COLOR_LABEL,
            Self::SiteName => str_constants::ADMIN_SETTING_SITE_NAME_LABEL,
            Self::SupportUrl => str_constants::ADMIN_SETTING_SUPPORT_URL_LABEL,
            Self::TabTitle => str_constants::ADMIN_SETTING_TAB_TITLE_LABEL,
        })
    }
    fn name(self) -> AdminSettingAttribute {
        AdminSettingAttribute::from(match self {
            Self::DefaultRoute => str_constants::ADMIN_SETTING_DEFAULT_ROUTE_NAME,
            Self::MainLogo => str_constants::ADMIN_SETTING_MAIN_LOGO_NAME,
            Self::OrganizationContacts => str_constants::ADMIN_SETTING_ORGANIZATION_CONTACTS_NAME,
            Self::OrganizationName => str_constants::ADMIN_SETTING_ORGANIZATION_NAME_NAME,
            Self::PrimaryColor => str_constants::ADMIN_SETTING_PRIMARY_COLOR_NAME,
            Self::SiteName => str_constants::ADMIN_SETTING_SITE_NAME_NAME,
            Self::SupportUrl => str_constants::ADMIN_SETTING_SUPPORT_URL_NAME,
            Self::TabTitle => str_constants::ADMIN_SETTING_TAB_TITLE_NAME,
        })
    }
    fn required(self) -> AdminSettingRequired {
        AdminSettingRequired::from(matches!(self, Self::DefaultRoute | Self::SiteName))
    }
}

pub(crate) fn admin_setting_input(
    field: AdminSettingField,
    value: LeptosAdminSettingSignal,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    let label = field.label().as_ref().to_owned();
    let name = field.name().as_ref().to_owned();
    let required = bool::from(field.required());
    let disabled = bool::from(disabled);
    let value = value.0;
    match field.input_kind() {
        AdminSettingInputKind::Text | AdminSettingInputKind::Url => {
            let input_type = match field.input_kind() {
                AdminSettingInputKind::Url => str_constants::HTML_URL_INPUT_TYPE,
                AdminSettingInputKind::Text | AdminSettingInputKind::TextArea => {
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
        AdminSettingInputKind::TextArea => leptos::prelude::IntoAny::into_any(leptos::view! {
            <label><span>{label}</span><textarea
                name=name
                required=required
                disabled=disabled
                on:input=move |event| leptos::prelude::Set::set(
                    &value,
                    leptos::prelude::event_target_value(&event),
                )
            >{leptos::prelude::Get::get(&value)}</textarea></label>
        }),
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AdminSettingsFormSignals {
    default_route: LeptosAdminSettingSignal,
    main_logo: LeptosAdminSettingSignal,
    organization_contacts: LeptosAdminSettingSignal,
    organization_name: LeptosAdminSettingSignal,
    primary_color: LeptosAdminSettingSignal,
    site_name: LeptosAdminSettingSignal,
    support_url: LeptosAdminSettingSignal,
    tab_title: LeptosAdminSettingSignal,
}
impl AdminSettingsFormSignals {
    pub(crate) fn new(values: &AdminSettingsFormValues) -> Self {
        Self {
            default_route: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.default_route().as_ref().to_owned(),
            )),
            main_logo: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.main_logo().as_ref().to_owned(),
            )),
            organization_contacts: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.organization_contacts().as_ref().to_owned(),
            )),
            organization_name: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.organization_name().as_ref().to_owned(),
            )),
            primary_color: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.primary_color().as_ref().to_owned(),
            )),
            site_name: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.site_name().as_ref().to_owned(),
            )),
            support_url: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.support_url().as_ref().to_owned(),
            )),
            tab_title: LeptosAdminSettingSignal::from(leptos::prelude::RwSignal::new(
                values.tab_title().as_ref().to_owned(),
            )),
        }
    }
    pub(crate) const fn default_route(self) -> LeptosAdminSettingSignal {
        self.default_route
    }
    pub(crate) const fn main_logo(self) -> LeptosAdminSettingSignal {
        self.main_logo
    }
    pub(crate) const fn organization_contacts(self) -> LeptosAdminSettingSignal {
        self.organization_contacts
    }
    pub(crate) const fn organization_name(self) -> LeptosAdminSettingSignal {
        self.organization_name
    }
    pub(crate) const fn primary_color(self) -> LeptosAdminSettingSignal {
        self.primary_color
    }
    pub(crate) const fn site_name(self) -> LeptosAdminSettingSignal {
        self.site_name
    }
    pub(crate) const fn support_url(self) -> LeptosAdminSettingSignal {
        self.support_url
    }
    pub(crate) const fn tab_title(self) -> LeptosAdminSettingSignal {
        self.tab_title
    }
}

pub(crate) fn admin_setting_inputs(
    signals: AdminSettingsFormSignals,
    disabled: AdminSettingDisabled,
) -> impl leptos::prelude::IntoView {
    leptos::view! {
        {admin_setting_input(AdminSettingField::DefaultRoute, signals.default_route(), disabled)}
        {admin_setting_input(AdminSettingField::SiteName, signals.site_name(), disabled)}
        {admin_setting_input(AdminSettingField::TabTitle, signals.tab_title(), disabled)}
        {admin_setting_input(AdminSettingField::OrganizationName, signals.organization_name(), disabled)}
        {admin_setting_input(AdminSettingField::OrganizationContacts, signals.organization_contacts(), disabled)}
        {admin_setting_input(AdminSettingField::SupportUrl, signals.support_url(), disabled)}
        {admin_setting_input(AdminSettingField::PrimaryColor, signals.primary_color(), disabled)}
        {admin_setting_input(AdminSettingField::MainLogo, signals.main_logo(), disabled)}
    }
}

#[derive(Clone, Debug)]
pub(crate) struct AdminSettingsFormValues {
    default_route: AdminSettingInputValue,
    main_logo: AdminSettingInputValue,
    organization_contacts: AdminSettingInputValue,
    organization_name: AdminSettingInputValue,
    primary_color: AdminSettingInputValue,
    site_name: AdminSettingInputValue,
    support_url: AdminSettingInputValue,
    tab_title: AdminSettingInputValue,
}
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
        Self {
            default_route: AdminSettingInputValue::from(
                value
                    .default_admin_route()
                    .as_ref()
                    .to_owned()
                    .into_boxed_str(),
            ),
            main_logo: optional(value.main_logo()),
            organization_contacts: optional(value.organization_contacts()),
            organization_name: optional(value.organization_name()),
            primary_color: optional(value.primary_color()),
            site_name: AdminSettingInputValue::from(
                value.site_name().as_ref().to_owned().into_boxed_str(),
            ),
            support_url: optional(value.support_url()),
            tab_title: optional(value.tab_title()),
        }
    }
}
impl AdminSettingsFormValues {
    pub(crate) const fn default_route(&self) -> &AdminSettingInputValue {
        &self.default_route
    }
    pub(crate) const fn main_logo(&self) -> &AdminSettingInputValue {
        &self.main_logo
    }
    pub(crate) const fn organization_contacts(&self) -> &AdminSettingInputValue {
        &self.organization_contacts
    }
    pub(crate) const fn organization_name(&self) -> &AdminSettingInputValue {
        &self.organization_name
    }
    pub(crate) const fn primary_color(&self) -> &AdminSettingInputValue {
        &self.primary_color
    }
    pub(crate) const fn site_name(&self) -> &AdminSettingInputValue {
        &self.site_name
    }
    pub(crate) const fn support_url(&self) -> &AdminSettingInputValue {
        &self.support_url
    }
    pub(crate) const fn tab_title(&self) -> &AdminSettingInputValue {
        &self.tab_title
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
