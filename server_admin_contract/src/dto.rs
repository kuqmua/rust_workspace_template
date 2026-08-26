#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AuthenticatedAdmin {
    display_name: super::AdminDisplayName,
    id: super::AdminUserId,
    login: super::AdminLogin,
    permissions: super::AdminPermissionValues,
    roles: super::AdminRoleNames,
}
#[cfg(test)]
#[path = "domain_types__dto__tests.rs"]
mod tests;
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn new(
        display_name: super::AdminDisplayName,
        id: super::AdminUserId,
        login: super::AdminLogin,
        permissions: super::AdminPermissionValues,
        roles: super::AdminRoleNames,
    ) -> Self {
        Self {
            display_name,
            id,
            login,
            permissions,
            roles,
        }
    }
    #[must_use]
    pub const fn display_name(&self) -> &super::AdminDisplayName {
        &self.display_name
    }
    #[must_use]
    pub fn permissions(&self) -> &[super::AdminPermissionValue] {
        self.permissions.as_ref()
    }
    #[must_use]
    pub const fn login(&self) -> &super::AdminLogin {
        &self.login
    }
    #[must_use]
    pub const fn roles(&self) -> &[super::AdminRoleName] {
        self.roles.as_slice()
    }
    #[must_use]
    pub fn has_permission(&self, permission: super::AdminPermission) -> super::AdminBool {
        let required = permission.as_str();
        super::AdminBool::from(
            self.permissions
                .as_ref()
                .iter()
                .any(|value| value.as_ref() == required.get()),
        )
    }
    #[must_use]
    pub fn can_access(&self, page: super::AdminPage) -> super::AdminBool {
        super::AdminBool::from(match page.authentication() {
            frontend_contract::domain_types::AuthenticationRequirement::Authenticated
            | frontend_contract::domain_types::AuthenticationRequirement::Public => true,
            frontend_contract::domain_types::AuthenticationRequirement::Permission(required) => {
                self.permissions
                    .as_ref()
                    .iter()
                    .any(|value| value.as_ref() == required.as_ref())
            }
        })
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminSignInRes {
    #[contract_struct_api(borrow)]
    user: AuthenticatedAdmin,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateUserReq {
    display_name: super::AdminDisplayName,
    login: super::AdminLogin,
    password: super::AdminNewPassword,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminCreateUserRes {
    id: super::AdminUserId,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateUserReq {
    display_name: Option<super::AdminDisplayName>,
    login: Option<super::AdminLogin>,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserPasswordReq {
    #[contract_struct_api(into)]
    password: super::AdminNewPassword,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminChangeOwnPasswordReq {
    current_password: super::AdminPassword,
    new_password: super::AdminNewPassword,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserBanReq {
    #[contract_struct_api(copy)]
    is_banned: super::AdminBool,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminCreateRoleReq {
    #[contract_struct_api(into)]
    name: super::AdminRoleName,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminCreateRoleRes {
    id: super::AdminRoleId,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateRoleReq {
    #[contract_struct_api(into)]
    name: super::AdminRoleName,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetUserRolesReq {
    expected_role_ids: super::AdminRoleIds,
    role_ids: super::AdminRoleIds,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new, into_parts)]
#[serde(deny_unknown_fields)]
pub struct AdminSetRolePermissionsReq {
    expected_permission_ids: super::AdminPermissionIds,
    permission_ids: super::AdminPermissionIds,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminUserSummary {
    #[contract_struct_api(borrow)]
    display_name: super::AdminDisplayName,
    #[contract_struct_api(copy_ref)]
    id: super::AdminUserId,
    #[contract_struct_api(copy_ref)]
    is_banned: super::AdminBool,
    #[contract_struct_api(borrow)]
    login: super::AdminLogin,
    #[serde(default)]
    #[contract_struct_api(slice = super::AdminRoleId)]
    role_ids: super::AdminRoleIds,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminRoleSummary {
    #[contract_struct_api(copy_ref)]
    id: super::AdminRoleId,
    #[contract_struct_api(copy_ref)]
    is_system: super::AdminBool,
    name: super::AdminRoleName,
    #[serde(default)]
    #[contract_struct_api(slice = super::AdminPermissionId)]
    permission_ids: super::AdminPermissionIds,
}
impl AdminRoleSummary {
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &super::AdminRoleName {
        &self.name
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminPermissionSummary {
    id: super::AdminPermissionId,
    name: super::AdminPermissionValue,
}
impl AdminPermissionSummary {
    #[must_use]
    pub const fn new(id: super::AdminPermissionId, name: super::AdminPermissionValue) -> Self {
        Self { id, name }
    }
    #[must_use]
    pub const fn id(&self) -> super::AdminPermissionId {
        self.id
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &super::AdminPermissionValue {
        &self.name
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminUsersPage {
    #[contract_struct_api(into, slice = AdminUserSummary)]
    items: super::AdminUserSummaries,
    #[contract_struct_api(slice = AdminRoleSummary)]
    roles: super::AdminRoleSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: super::AdminPageTotal,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminRolesPage {
    #[contract_struct_api(into, slice = AdminRoleSummary)]
    items: super::AdminRoleSummaries,
    #[contract_struct_api(slice = AdminPermissionSummary)]
    permissions: super::AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: super::AdminPageTotal,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminPermissionsPage {
    #[contract_struct_api(into, slice = AdminPermissionSummary)]
    items: super::AdminPermissionSummaries,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: super::AdminPageTotal,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminAuditView {
    #[contract_struct_api(borrow)]
    action: super::AdminText,
    #[contract_struct_api(borrow)]
    created_at: super::AdminAuditTimestamp,
    #[contract_struct_api(option_borrow)]
    details: Option<super::SerdeJsonAdminAuditDetails>,
    #[contract_struct_api(copy_ref)]
    id: super::AdminAuditLogId,
    #[contract_struct_api(borrow)]
    resource: super::AdminText,
    #[contract_struct_api(option_borrow)]
    resource_id: Option<super::AdminText>,
    #[contract_struct_api(copy_ref)]
    succeeded: super::AdminBool,
    #[contract_struct_api(copy_ref)]
    user_id: Option<super::AdminUserId>,
    #[contract_struct_api(option_borrow)]
    user_login: Option<super::AdminLogin>,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditCursor {
    #[contract_struct_api(borrow)]
    created_at: super::AdminAuditTimestamp,
    #[contract_struct_api(copy_ref)]
    id: super::AdminAuditLogId,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditPage {
    #[contract_struct_api(slice = AdminAuditView)]
    items: super::AdminAuditViews,
    #[schema(inline)]
    #[contract_struct_api(option_borrow)]
    next_cursor: Option<AdminAuditCursor>,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: super::AdminPageTotal,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
pub struct AdminDataColumn {
    filters: AdminDataFilters,
    label: super::AdminText,
    name: super::AdminText,
    input_kind: AdminDataInputKind,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn new(
        filters: AdminDataFilters,
        input_kind: AdminDataInputKind,
        label: super::AdminText,
        name: super::AdminText,
    ) -> Self {
        Self {
            filters,
            label,
            name,
            input_kind,
        }
    }
    #[must_use]
    pub const fn filters(&self) -> &[AdminDataFilter] {
        self.filters.as_slice()
    }
    #[must_use]
    pub const fn input_kind(&self) -> AdminDataInputKind {
        self.input_kind
    }
    #[must_use]
    pub const fn label(&self) -> &super::AdminText {
        &self.label
    }
    #[must_use]
    #[allow(clippy::same_name_method)] // Utoipa 5's static schema name intentionally coexists with this domain accessor
    pub const fn name(&self) -> &super::AdminText {
        &self.name
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminDataFilter {
    operation: frontend_contract::domain_types::FilterOperation,
    value_shape: frontend_contract::domain_types::FilterValueShape,
}
impl From<frontend_contract::domain_types::FilterOperation> for AdminDataFilter {
    fn from(value: frontend_contract::domain_types::FilterOperation) -> Self {
        Self {
            operation: value,
            value_shape: value.value_shape(),
        }
    }
}
impl AdminDataFilter {
    #[must_use]
    pub const fn operation(&self) -> frontend_contract::domain_types::FilterOperation {
        self.operation
    }
    #[must_use]
    pub const fn value_shape(&self) -> frontend_contract::domain_types::FilterValueShape {
        self.value_shape
    }
    #[must_use]
    pub fn requires_value(&self) -> super::AdminBool {
        super::AdminBool::from(!matches!(
            self.value_shape,
            frontend_contract::domain_types::FilterValueShape::None
        ))
    }
    #[must_use]
    pub fn requires_end(&self) -> super::AdminBool {
        super::AdminBool::from(matches!(
            self.value_shape,
            frontend_contract::domain_types::FilterValueShape::Range
        ))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "super::collections::AdminBoundedVec<AdminDataFilter>")]
#[schema(value_type = super::collections::AdminOpenApiVec<AdminDataFilter, 100>)]
pub struct AdminDataFilters(super::collections::AdminBoundedVec<AdminDataFilter>);
impl TryFrom<Vec<AdminDataFilter>> for AdminDataFilters {
    type Error = super::AdminCollectionError;
    fn try_from(value: Vec<AdminDataFilter>) -> Result<Self, Self::Error> {
        super::collections::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataFilters {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataFilter] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminDataInputKind {
    Checkbox,
    Date,
    DateTime,
    Number,
    Text,
    Time,
    Uuid,
}
impl From<frontend_contract::domain_types::InputKind> for AdminDataInputKind {
    fn from(value: frontend_contract::domain_types::InputKind) -> Self {
        match value {
            frontend_contract::domain_types::InputKind::Checkbox => Self::Checkbox,
            frontend_contract::domain_types::InputKind::Date => Self::Date,
            frontend_contract::domain_types::InputKind::DateTime => Self::DateTime,
            frontend_contract::domain_types::InputKind::Number => Self::Number,
            frontend_contract::domain_types::InputKind::Text => Self::Text,
            frontend_contract::domain_types::InputKind::Time => Self::Time,
            frontend_contract::domain_types::InputKind::Uuid => Self::Uuid,
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "super::collections::AdminBoundedVec<AdminDataColumn>")]
#[schema(value_type = super::collections::AdminOpenApiVec<AdminDataColumn, 10_000>)]
pub struct AdminDataColumns(super::collections::AdminBoundedVec<AdminDataColumn>);
impl TryFrom<Vec<AdminDataColumn>> for AdminDataColumns {
    type Error = super::AdminCollectionError;
    fn try_from(value: Vec<AdminDataColumn>) -> Result<Self, Self::Error> {
        super::collections::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminDataColumns {
    #[must_use]
    pub const fn as_slice(&self) -> &[AdminDataColumn] {
        self.0.as_slice()
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataRow {
    #[contract_struct_api(slice = super::AdminText)]
    values: super::AdminTexts,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
#[optimal_memory_layout(skip)]
pub struct AdminDataTableView {
    #[contract_struct_api(slice = AdminDataColumn)]
    columns: AdminDataColumns,
    #[contract_struct_api(slice = AdminDataRow)]
    items: super::AdminDataRows,
    #[contract_struct_api(copy_ref)]
    table: super::AdminDataTable,
    #[schema(value_type = u64)]
    #[contract_struct_api(copy_ref)]
    total: super::AdminPageTotal,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminDataTableCatalog {
    #[contract_struct_api(slice = super::AdminDataTable)]
    items: super::AdminDataTables,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(
    max = 262_144usize,
    chars,
    serde,
    utoipa,
    description = "bounded administrator audit CSV export"
)]
pub struct AdminAuditExportCsv(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    frontend_contract::domain_types::ContractStructApi,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[contract_struct_api(new)]
pub struct AdminAuditExport {
    #[schema(value_type = String, max_length = 262_144)]
    #[contract_struct_api(borrow)]
    csv: AdminAuditExportCsv,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[serde(deny_unknown_fields)]
pub struct AdminSignInReq {
    login: super::AdminLogin,
    password: super::AdminPassword,
}
impl AdminSignInReq {
    #[must_use]
    pub const fn new(login: super::AdminLogin, password: super::AdminPassword) -> Self {
        Self { login, password }
    }
    #[must_use]
    pub const fn login(&self) -> &super::AdminLogin {
        &self.login
    }
    #[must_use]
    pub fn into_parts(self) -> (super::AdminLogin, super::AdminPassword) {
        (self.login, self.password)
    }
    #[must_use]
    pub const fn password(&self) -> &super::AdminPassword {
        &self.password
    }
}
