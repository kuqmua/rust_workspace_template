#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq,
)]
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
    pub fn key(self) -> frontend_contract::contract_str::ContractStr {
        self.values().into_parts().0
    }
    #[must_use]
    pub fn label(self) -> frontend_contract::contract_str::ContractStr {
        self.values().into_parts().1
    }
    pub fn try_from_key(
        options: &[Self],
        admin_table_sort_key_ref: crate::admin_table_sort_key_ref::AdminTableSortKeyRef<'_>,
    ) -> Result<
        Self,
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError,
    > {
        options
            .iter()
            .copied()
            .find(|option| option.key().as_ref() == admin_table_sort_key_ref.get())
            .ok_or(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown)
    }
    fn values(self) -> crate::admin_table_sort_values::AdminTableSortValues {
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
        crate::admin_table_sort_values::AdminTableSortValues::new(
            frontend_contract::contract_str::ContractStr::from(key),
            frontend_contract::contract_str::ContractStr::from(label),
        )
    }
}
