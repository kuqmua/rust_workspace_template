pub(crate) const ADMIN_COLLECTION_MAX_ITEMS: usize = 10_000usize;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminCollectionError {
    #[error(
        "{}",
        str_constants::ADMINISTRATOR_COLLECTION_EXCEEDS_MAXIMUM_ITEM_COUNT
    )]
    TooLong,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(from = "bounded_types::BoundedVec<T, 0, { ADMIN_COLLECTION_MAX_ITEMS }>")]
pub(crate) struct AdminBoundedVec<T>(
    bounded_types::BoundedVec<T, 0, { ADMIN_COLLECTION_MAX_ITEMS }>,
);
impl<T> AdminBoundedVec<T> {
    pub(crate) const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> From<[T; 0]> for AdminBoundedVec<T> {
    fn from(_value: [T; 0]) -> Self {
        Self(bounded_types::BoundedVec::from([]))
    }
}
impl<T> TryFrom<Vec<T>> for AdminBoundedVec<T> {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|_error| AdminCollectionError::TooLong)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct StdPhantomDataAdminOpenApiVec<T>(std::marker::PhantomData<T>);
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(dead_code)] // schema-only generic carries its item type without runtime construction
pub(crate) struct AdminOpenApiVec<T, const MAX: usize> {
    marker: StdPhantomDataAdminOpenApiVec<T>,
}
impl<T: utoipa::PartialSchema, const MAX: usize> utoipa::__dev::ComposeSchema
    for AdminOpenApiVec<T, MAX>
{
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ArrayBuilder::new()
            .items(<T as utoipa::PartialSchema>::schema())
            .max_items(Some(MAX))
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema, const MAX: usize> utoipa::ToSchema for AdminOpenApiVec<T, MAX> {
    fn schemas(
        schemas: &mut Vec<(
            String,
            utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
        )>,
    ) {
        schemas.push((
            T::name().into_owned(),
            <T as utoipa::PartialSchema>::schema(),
        ));
        T::schemas(schemas);
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminPermissionValue>")]
#[schema(value_type = AdminOpenApiVec<super::AdminPermissionValue, 10_000>)]
pub struct AdminPermissionValues(AdminBoundedVec<super::AdminPermissionValue>);
impl TryFrom<Vec<super::AdminPermissionValue>> for AdminPermissionValues {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminPermissionValue>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminRoleName>")]
#[schema(value_type = AdminOpenApiVec<super::AdminRoleName, 10_000>)]
pub struct AdminRoleNames(AdminBoundedVec<super::AdminRoleName>);
impl TryFrom<Vec<super::AdminRoleName>> for AdminRoleNames {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminRoleName>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleNames {
    pub(crate) const fn as_slice(&self) -> &[super::AdminRoleName] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminRoleId>")]
#[schema(value_type = AdminOpenApiVec<super::AdminRoleId, 10_000>)]
pub struct AdminRoleIds(AdminBoundedVec<super::AdminRoleId>);
impl TryFrom<Vec<super::AdminRoleId>> for AdminRoleIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminRoleId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleIds {
    pub(crate) const fn as_slice(&self) -> &[super::AdminRoleId] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminPermissionId>")]
#[schema(value_type = AdminOpenApiVec<super::AdminPermissionId, 10_000>)]
pub struct AdminPermissionIds(AdminBoundedVec<super::AdminPermissionId>);
impl TryFrom<Vec<super::AdminPermissionId>> for AdminPermissionIds {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminPermissionId>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminPermissionIds {
    pub(crate) const fn as_slice(&self) -> &[super::AdminPermissionId] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminUserSummary>")]
#[schema(value_type = AdminOpenApiVec<super::AdminUserSummary, 10_000>)]
pub struct AdminUserSummaries(AdminBoundedVec<super::AdminUserSummary>);
impl TryFrom<Vec<super::AdminUserSummary>> for AdminUserSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminUserSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminUserSummaries {
    pub(crate) const fn as_slice(&self) -> &[super::AdminUserSummary] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminRoleSummary>")]
#[schema(value_type = AdminOpenApiVec<super::AdminRoleSummary, 10_000>)]
pub struct AdminRoleSummaries(AdminBoundedVec<super::AdminRoleSummary>);
impl TryFrom<Vec<super::AdminRoleSummary>> for AdminRoleSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminRoleSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRoleSummaries {
    pub(crate) const fn as_slice(&self) -> &[super::AdminRoleSummary] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminPermissionSummary>")]
#[schema(value_type = AdminOpenApiVec<super::AdminPermissionSummary, 10_000>)]
pub struct AdminPermissionSummaries(AdminBoundedVec<super::AdminPermissionSummary>);
impl TryFrom<Vec<super::AdminPermissionSummary>> for AdminPermissionSummaries {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminPermissionSummary>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminPermissionSummaries {
    pub(crate) const fn as_slice(&self) -> &[super::AdminPermissionSummary] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminAuditView>")]
#[schema(value_type = AdminOpenApiVec<super::AdminAuditView, 10_000>)]
pub struct AdminAuditViews(AdminBoundedVec<super::AdminAuditView>);
impl TryFrom<Vec<super::AdminAuditView>> for AdminAuditViews {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminAuditView>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminAuditViews {
    pub(crate) const fn as_slice(&self) -> &[super::AdminAuditView] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminText>")]
#[schema(value_type = AdminOpenApiVec<super::AdminText, 10_000>)]
pub struct AdminTexts(AdminBoundedVec<super::AdminText>);
impl TryFrom<Vec<super::AdminText>> for AdminTexts {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminText>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminTexts {
    pub(crate) const fn as_slice(&self) -> &[super::AdminText] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminDataRow>")]
#[schema(value_type = AdminOpenApiVec<super::AdminDataRow, 10_000>)]
pub struct AdminDataRows(AdminBoundedVec<super::AdminDataRow>);
impl TryFrom<Vec<super::AdminDataRow>> for AdminDataRows {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminDataRow>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataRows {
    pub(crate) const fn as_slice(&self) -> &[super::AdminDataRow] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminDataTable>")]
#[schema(value_type = AdminOpenApiVec<super::AdminDataTable, 10_000>)]
pub struct AdminDataTables(AdminBoundedVec<super::AdminDataTable>);
impl TryFrom<Vec<super::AdminDataTable>> for AdminDataTables {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminDataTable>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataTables {
    pub(crate) const fn as_slice(&self) -> &[super::AdminDataTable] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminOptionalSetting>")]
#[schema(value_type = AdminOpenApiVec<super::AdminOptionalSetting, 10_000>)]
pub struct AdminOptionalSettings(AdminBoundedVec<super::AdminOptionalSetting>);
impl TryFrom<Vec<super::AdminOptionalSetting>> for AdminOptionalSettings {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminOptionalSetting>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
#[serde(from = "AdminBoundedVec<super::AdminSessionView>")]
#[schema(value_type = AdminOpenApiVec<super::AdminSessionView, 10_000>)]
pub struct AdminSessionViews(AdminBoundedVec<super::AdminSessionView>);
impl TryFrom<Vec<super::AdminSessionView>> for AdminSessionViews {
    type Error = AdminCollectionError;
    fn try_from(value: Vec<super::AdminSessionView>) -> Result<Self, Self::Error> {
        AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminSessionViews {
    pub(crate) const fn as_slice(&self) -> &[super::AdminSessionView] {
        self.0.as_slice()
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminRoleIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminPermissionIds {
    fn default() -> Self {
        Self::from(AdminEmptyCollection)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminEmptyCollection;
impl From<AdminEmptyCollection> for AdminRoleIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
impl From<AdminEmptyCollection> for AdminPermissionIds {
    fn from(_value: AdminEmptyCollection) -> Self {
        Self(AdminBoundedVec::from([]))
    }
}
