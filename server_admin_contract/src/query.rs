#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(from = "bool")]
pub struct AdminBool(bool);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct AdminPageOffset(u32);
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminPageOffsetVisitor;
impl serde::de::Visitor<'_> for AdminPageOffsetVisitor {
    type Value = AdminPageOffset;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(str_constants::ADMIN_PAGE_OFFSET_EXPECTING)
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        v.parse::<u32>()
            .map(AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        u32::try_from(v)
            .map(AdminPageOffset::from)
            .map_err(serde::de::Error::custom)
    }
}
#[cfg(test)]
mod tests;
impl<'de> serde::Deserialize<'de> for AdminPageOffset {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageOffsetVisitor)?;
        Ok(Self::from(u32::from(value)))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminPageLimit(u16);
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminPageLimitVisitor;
impl serde::de::Visitor<'_> for AdminPageLimitVisitor {
    type Value = AdminPageLimit;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "an administrator page limit from {} through {}",
            AdminPageLimit::MIN,
            AdminPageLimit::MAX
        )
    }
    fn visit_str<Error>(self, v: &str) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = v.parse::<u16>().map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
    fn visit_u64<Error>(self, v: u64) -> Result<Self::Value, Error>
    where
        Error: serde::de::Error,
    {
        let parsed = u16::try_from(v).map_err(serde::de::Error::custom)?;
        AdminPageLimit::try_from(parsed).map_err(serde::de::Error::custom)
    }
}
impl<'de> serde::Deserialize<'de> for AdminPageLimit {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageLimitVisitor)?;
        Self::try_from(u16::from(value)).map_err(serde::de::Error::custom)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminDefaultPageLimit;
impl From<AdminDefaultPageLimit> for AdminPageLimit {
    fn from(_value: AdminDefaultPageLimit) -> Self {
        Self(Self::DEFAULT)
    }
}
impl Default for AdminPageLimit {
    fn default() -> Self {
        Self::from(AdminDefaultPageLimit)
    }
}
impl TryFrom<u16> for AdminPageLimit {
    type Error = AdminPageLimitError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminPageLimitError)
        }
    }
}
impl AdminPageLimit {
    pub const DEFAULT: u16 = 20u16;
    pub const MAX: u16 = 100u16;
    pub const MIN: u16 = 1u16;
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq, thiserror::Error,
)]
#[error(
    "administrator page limit must be between {min} and {max}",
    min = AdminPageLimit::MIN,
    max = AdminPageLimit::MAX
)]
pub struct AdminPageLimitError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
#[serde(from = "u64")]
pub struct AdminPageTotal(u64);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 128usize,
    chars,
    serde,
    utoipa,
    description = "administrator table search"
)]
pub struct AdminTableSearch(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    newtype::BoundedString,
    newtype::AsRefStr,
)]
#[bounded_string(
    max = 32usize,
    chars,
    serde,
    utoipa,
    description = "administrator table sort key"
)]
pub struct AdminTableSortKey(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminSortDirection {
    #[default]
    Asc,
    Desc,
}
impl AsRef<str> for AdminSortDirection {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asc => str_constants::ASC_ALT,
            Self::Desc => str_constants::DESC_ALT,
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminTableQuery {
    #[serde(default)]
    #[param(value_type = String, max_length = 128)]
    search: AdminTableSearch,
    #[serde(default)]
    #[param(value_type = String, max_length = 32)]
    sort: AdminTableSortKey,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: AdminPageLimit,
    #[serde(default)]
    #[param(inline)]
    direction: AdminSortDirection,
}
impl AdminTableQuery {
    #[must_use]
    pub fn pagination(limit: AdminPageLimit, offset: AdminPageOffset) -> Self {
        Self {
            offset,
            limit,
            ..Self::default()
        }
    }
    #[must_use]
    pub const fn limit(&self) -> AdminPageLimit {
        self.limit
    }
    #[must_use]
    pub const fn offset(&self) -> AdminPageOffset {
        self.offset
    }
    #[must_use]
    pub const fn search(&self) -> &AdminTableSearch {
        &self.search
    }
    #[must_use]
    pub const fn sort(&self) -> &AdminTableSortKey {
        &self.sort
    }
    #[must_use]
    pub const fn direction(&self) -> AdminSortDirection {
        self.direction
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 63usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter field"
)]
pub struct AdminFilterField(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(
    max = 4096usize,
    chars,
    serde,
    utoipa,
    description = "administrator filter value"
)]
pub struct AdminFilterValue(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    newtype::AsRefStr,
    newtype::BoundedString,
    newtype::Display,
)]
#[bounded_string(max = 63usize)]
pub struct AdminFilterOperationKey(String);
impl From<frontend_contract::FilterOperation> for AdminFilterOperationKey {
    fn from(value: frontend_contract::FilterOperation) -> Self {
        let formatted = format!("{value:?}");
        let mut key = String::with_capacity(formatted.len().saturating_mul(2usize));
        formatted
            .chars()
            .enumerate()
            .for_each(|(index, character)| {
                if character.is_uppercase() && index > 0usize {
                    key.push('_');
                }
                key.extend(character.to_lowercase());
            });
        Self::try_from(key).unwrap_or_default()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminDataTableFilterQuery {
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 63)]
    filter_field: Option<AdminFilterField>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_value: Option<AdminFilterValue>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_end: Option<AdminFilterValue>,
    #[serde(default)]
    #[param(inline)]
    filter_operation: Option<frontend_contract::FilterOperation>,
}
impl AdminDataTableFilterQuery {
    #[must_use]
    pub const fn new(
        filter_field: Option<AdminFilterField>,
        filter_operation: Option<frontend_contract::FilterOperation>,
        filter_value: Option<AdminFilterValue>,
        filter_end: Option<AdminFilterValue>,
    ) -> Self {
        Self {
            filter_field,
            filter_value,
            filter_end,
            filter_operation,
        }
    }
    #[must_use]
    pub const fn field(&self) -> Option<&AdminFilterField> {
        self.filter_field.as_ref()
    }
    #[must_use]
    pub const fn operation(&self) -> Option<frontend_contract::FilterOperation> {
        self.filter_operation
    }
    #[must_use]
    pub const fn value(&self) -> Option<&AdminFilterValue> {
        self.filter_value.as_ref()
    }
    #[must_use]
    pub const fn end(&self) -> Option<&AdminFilterValue> {
        self.filter_end.as_ref()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminDataTableQuery {
    #[serde(flatten)]
    filter: AdminDataTableFilterQuery,
    #[serde(flatten)]
    page: AdminTableQuery,
}
impl utoipa::IntoParams for AdminDataTableQuery {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        let parameter_in = parameter_in_provider();
        let mut parameters =
            <AdminDataTableFilterQuery as utoipa::IntoParams>::into_params(|| parameter_in.clone());
        parameters.extend(<AdminTableQuery as utoipa::IntoParams>::into_params(|| {
            parameter_in.clone()
        }));
        parameters
    }
}
impl AdminDataTableQuery {
    #[must_use]
    pub const fn new(filter: AdminDataTableFilterQuery, page: AdminTableQuery) -> Self {
        Self { filter, page }
    }
    #[must_use]
    pub const fn filter(&self) -> &AdminDataTableFilterQuery {
        &self.filter
    }
    #[must_use]
    pub const fn page(&self) -> &AdminTableQuery {
        &self.page
    }
}
