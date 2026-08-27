#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminTableSortField {
    AuditAction,
    AuditCreatedAt,
    AuditResource,
    AuditSucceeded,
    AuditUserId,
    PermissionId,
    PermissionName,
    RoleId,
    RoleName,
    RoleSystem,
    UserDisplayName,
    UserId,
    UserLogin,
    UserStatus,
}

impl AdminTableSortField {
    pub const USER: [Self; 4] = [
        Self::UserLogin,
        Self::UserDisplayName,
        Self::UserId,
        Self::UserStatus,
    ];
    pub const ROLE: [Self; 3] = [Self::RoleName, Self::RoleId, Self::RoleSystem];
    pub const PERMISSION: [Self; 2] = [Self::PermissionName, Self::PermissionId];
    pub const AUDIT: [Self; 5] = [
        Self::AuditCreatedAt,
        Self::AuditUserId,
        Self::AuditAction,
        Self::AuditResource,
        Self::AuditSucceeded,
    ];
    #[must_use]
    pub fn key(self) -> frontend_contract::domain_types::ContractStr {
        self.values().key
    }
    #[must_use]
    pub fn label(self) -> frontend_contract::domain_types::ContractStr {
        self.values().label
    }
    pub fn try_from_key(
        options: &[Self],
        key: super::AdminTableSortKeyRef<'_>,
    ) -> Result<Self, super::AdminTableSortFieldTryFromKeyError> {
        options
            .iter()
            .copied()
            .find(|option| option.key().as_ref() == key.0)
            .ok_or(super::AdminTableSortFieldTryFromKeyError)
    }
    fn values(self) -> super::AdminTableSortValues {
        let (key, label) = match self {
            Self::AuditAction => (constants_str::ACTION, constants_str::SHARED_VALUES_ACTION_2),
            Self::AuditCreatedAt => (constants_str::CREATED_AT, constants_str::SHARED_VALUES_TIME),
            Self::AuditResource => (
                constants_str::RESOURCE,
                constants_str::SHARED_VALUES_RESOURCE_2,
            ),
            Self::AuditSucceeded => (constants_str::SUCCEEDED, constants_str::RESULT),
            Self::AuditUserId => (constants_str::USER_ID, constants_str::SHARED_VALUES_USER),
            Self::PermissionId | Self::RoleId | Self::UserId => {
                (constants_str::SQL_NAMES_ID, constants_str::ID)
            }
            Self::PermissionName | Self::RoleName => {
                (constants_str::NAME, constants_str::SHARED_VALUES_NAME_2)
            }
            Self::RoleSystem => (constants_str::SYSTEM, constants_str::SHARED_VALUES_SYSTEM_2),
            Self::UserDisplayName => (
                constants_str::DISPLAY_NAME,
                constants_str::SHARED_VALUES_DISPLAY_NAME_2,
            ),
            Self::UserLogin => (constants_str::LOGIN, constants_str::SHARED_VALUES_LOGIN_2),
            Self::UserStatus => (
                constants_str::STATUS_ALT,
                constants_str::SHARED_VALUES_STATUS_2,
            ),
        };
        super::AdminTableSortValues {
            key: frontend_contract::domain_types::ContractStr::from(key),
            label: frontend_contract::domain_types::ContractStr::from(label),
        }
    }
}
