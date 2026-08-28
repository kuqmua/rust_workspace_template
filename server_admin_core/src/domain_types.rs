pub use crate::admin_audit_log_id::AdminAuditLogId;
pub use crate::admin_id_try_from_i64_error::AdminIdTryFromI64Error;
pub use crate::admin_non_zero_usize::AdminNonZeroUsize;
pub use crate::admin_permission_id::AdminPermissionId;
pub use crate::admin_permission_name::AdminPermissionName;
pub(crate) use crate::admin_resource_text::AdminResourceText;
pub use crate::admin_role_id::AdminRoleId;
pub use crate::admin_socket_addr::AdminSocketAddr;
pub use crate::admin_user_id::AdminUserId;
pub use crate::secrecy_admin_string::SecrecyAdminString;
pub use crate::std_admin_bool::StdAdminBool;
pub use crate::std_admin_str_ref::StdAdminStrRef;
pub use crate::std_admin_string::*;
pub use crate::uuid_admin_value::UuidAdminValue;

#[cfg(test)]
mod tests {
    #[test]
    fn administrator_secret_text_enforces_internal_bound() {
        let at_limit = constants_str::A_ALT.repeat(constants_usize::VALUE_8_192);
        let secret = super::SecrecyAdminString::try_from(at_limit.clone()).expect(
            "6673b876 administrator_secret_text_enforces_internal_bound invariant must hold",
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(&secret)
                .as_ref()
                .as_str(),
            at_limit.as_str()
        );
        assert_eq!(
            super::SecrecyAdminString::try_from("a".repeat(8_193usize)).err(),
            Some(super::StdAdminStringTryFromStringError::TooLong {
                len: 8_193usize,
                max: constants_usize::VALUE_8_192,
            })
        );
    }
    #[test]
    fn administrator_secret_text_is_redacted_and_zeroizable() {
        let raw = constants_str::NEVER_PRINT_THIS_VALUE;
        let secret = super::SecrecyAdminString::try_from(raw.to_owned()).expect(
            "67b629e2 administrator_secret_text_is_redacted_and_zeroizable invariant must hold",
        );
        assert!(!format!("{secret:?}").contains(raw));
        let mut bounded = super::StdAdminString::try_from(raw.to_owned()).expect(
            "201f3c4b administrator_secret_text_is_redacted_and_zeroizable invariant must hold",
        );
        secrecy::zeroize::Zeroize::zeroize(&mut bounded);
        assert!(bounded.as_ref().is_empty());
    }
    #[test]
    fn administrator_resource_values_are_stable() {
        let positive = server_admin_contract::domain_types::PositiveNonZeroI64::try_from(42i64)
            .expect("2570af3b administrator_resource_values_are_stable invariant must hold");
        assert_eq!(
            super::StdAdminString::from_positive_i64(positive).as_ref(),
            "42"
        );
        assert_eq!(
            super::StdAdminString::system_settings_resource().as_ref(),
            "1"
        );
        let uuid_value = uuid::Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0001u128);
        let expected = uuid_value.to_string();
        let uuid = super::UuidAdminValue::from(uuid_value);
        assert_eq!(
            super::StdAdminString::from_uuid(uuid).as_ref().as_str(),
            expected.as_str()
        );
    }
}
