#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "HTML form DTOs are deserialized in this module and consumed by the sibling action adapter"
)]

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SignInForm {
    pub(super) login: server_admin_contract::domain_types::AdminLogin,
    pub(super) password: server_admin_contract::domain_types::AdminPassword,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ChangePasswordForm {
    pub(super) current_password: server_admin_contract::domain_types::AdminPassword,
    pub(super) new_password: server_admin_contract::domain_types::AdminNewPassword,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RevokeSessionForm {
    pub(super) session_id: server_admin_contract::domain_types::AdminSessionIdentifier,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CreateUserForm {
    pub(super) display_name: server_admin_contract::domain_types::AdminDisplayName,
    pub(super) login: server_admin_contract::domain_types::AdminLogin,
    pub(super) password: server_admin_contract::domain_types::AdminNewPassword,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UpdateUserForm {
    pub(super) display_name: server_admin_contract::domain_types::AdminDisplayName,
    pub(super) login: server_admin_contract::domain_types::AdminLogin,
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UserPasswordForm {
    pub(super) password: server_admin_contract::domain_types::AdminNewPassword,
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UserBanForm {
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
    pub(super) is_banned: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UserIdForm {
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(super) struct UserRolesForm {
    pub(super) expected_role_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(super) selected: StdAdminHtmlSelected,
    pub(super) user_id: server_admin_contract::domain_types::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CreateRoleForm {
    pub(super) name: server_admin_contract::domain_types::AdminRoleName,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UpdateRoleForm {
    pub(super) name: server_admin_contract::domain_types::AdminRoleName,
    pub(super) role_id: server_admin_contract::domain_types::AdminRoleId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RoleIdForm {
    pub(super) role_id: server_admin_contract::domain_types::AdminRoleId,
    pub(super) confirmation: server_admin_contract::domain_types::AdminBool,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
pub(super) struct RolePermissionsForm {
    pub(super) expected_permission_ids: AdminHtmlFormText,
    #[serde(flatten)]
    pub(super) selected: StdAdminHtmlSelected,
    pub(super) role_id: server_admin_contract::domain_types::AdminRoleId,
}

pub(super) const ADMIN_HTML_FORM_SELECTED_MAX_ITEMS: usize = 1_000usize;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_TEXT_TOO_LONG)]
pub(super) struct AdminHtmlFormTextError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormTextError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("{message}", message = constants_str::ADMIN_HTML_FORM_KEY_TOO_LONG)]
pub(super) struct AdminHtmlFormKeyError;
impl From<bounded_types::domain_types::BoundedValueError> for AdminHtmlFormKeyError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("administrator HTML form contains too many selected fields")]
pub(super) struct StdAdminHtmlSelectedError;
impl From<bounded_types::domain_types::BoundedValueError> for StdAdminHtmlSelectedError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub(super) struct AdminHtmlFormText(
    bounded_types::domain_types::text::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormText {
    type Error = AdminHtmlFormTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::domain_types::text::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormTextError::from)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub(super) struct AdminHtmlFormKey(
    bounded_types::domain_types::text::BoundedString<0, { constants_usize::VALUE_8_192 }>,
);
impl TryFrom<String> for AdminHtmlFormKey {
    type Error = AdminHtmlFormKeyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        bounded_types::domain_types::text::BoundedString::try_from(value)
            .map(Self)
            .map_err(AdminHtmlFormKeyError::from)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    serde::Deserialize,
)]
#[serde(
    from = "bounded_types::domain_types::btree::BoundedBTreeMap<AdminHtmlFormKey, AdminHtmlFormText, ADMIN_HTML_FORM_SELECTED_MAX_ITEMS>"
)]
pub(super) struct StdAdminHtmlSelected(
    bounded_types::domain_types::btree::BoundedBTreeMap<
        AdminHtmlFormKey,
        AdminHtmlFormText,
        ADMIN_HTML_FORM_SELECTED_MAX_ITEMS,
    >,
);
impl TryFrom<std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>>
    for StdAdminHtmlSelected
{
    type Error = StdAdminHtmlSelectedError;
    fn try_from(
        value: std::collections::BTreeMap<AdminHtmlFormKey, AdminHtmlFormText>,
    ) -> Result<Self, Self::Error> {
        bounded_types::domain_types::btree::BoundedBTreeMap::try_from(value)
            .map(Self)
            .map_err(StdAdminHtmlSelectedError::from)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SettingsForm {
    pub(super) default_admin_route: server_admin_contract::domain_types::AdminDefaultRoute,
    pub(super) main_logo: AdminHtmlFormText,
    pub(super) organization_contacts: AdminHtmlFormText,
    pub(super) organization_name: AdminHtmlFormText,
    pub(super) primary_color: AdminHtmlFormText,
    pub(super) site_name: server_admin_contract::domain_types::AdminSiteName,
    pub(super) support_url: AdminHtmlFormText,
    pub(super) tab_title: AdminHtmlFormText,
}
