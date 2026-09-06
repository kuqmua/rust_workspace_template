#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_into_inner::IntoInner,
)]
#[bounded_string(max = crate::identity::ADMIN_PASSWORD_MAX_CHARS, min = crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS, chars, serde, utoipa, write_only, validator = crate::identity::ADMIN_NEW_PASSWORD_IS_VALID, description = "new administrator password")]
pub struct AdminNewPassword(
    bounded_types::bounded_string::BoundedString<
        { crate::identity::ADMIN_NEW_PASSWORD_MIN_CHARS },
        { crate::identity::ADMIN_PASSWORD_MAX_CHARS },
        true,
    >,
);

impl TryFrom<crate::admin_password_entropy::AdminPasswordEntropy> for AdminNewPassword {
    type Error = AdminNewPasswordTryFromStringError;

    fn try_from(
        value: crate::admin_password_entropy::AdminPasswordEntropy,
    ) -> Result<Self, Self::Error> {
        let password = ['A', 'a', '1', '!']
            .into_iter()
            .chain(value.into_inner().into_iter().flat_map(|byte| {
                [byte >> 4u8, byte & 15u8].map(|nibble| {
                    char::from(if nibble < 10u8 {
                        b'0'.saturating_add(nibble)
                    } else {
                        b'a'.saturating_add(nibble.saturating_sub(10u8))
                    })
                })
            }))
            .collect::<String>();
        Self::try_from(password)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generated_passwords_satisfy_policy_for_every_byte_value() {
        assert!((u8::MIN..=u8::MAX).all(|byte| {
            crate::admin_new_password::AdminNewPassword::try_from(
                crate::admin_password_entropy::AdminPasswordEntropy::from([byte; 32]),
            )
            .is_ok_and(|password| {
                password.as_ref().len() == 68usize
                    && crate::identity::ADMIN_NEW_PASSWORD_IS_VALID(password.as_ref())
            })
        }));
    }

    #[test]
    fn test_generated_password_preserves_every_entropy_byte() {
        let baseline = crate::admin_new_password::AdminNewPassword::try_from(
            crate::admin_password_entropy::AdminPasswordEntropy::from([0u8; 32]),
        );
        assert!((0usize..32usize).all(|position| {
            let entropy = std::array::from_fn(|index| u8::from(index == position));
            let changed = crate::admin_new_password::AdminNewPassword::try_from(
                crate::admin_password_entropy::AdminPasswordEntropy::from(entropy),
            );
            matches!((&baseline, changed), (Ok(original), Ok(password)) if original != &password)
        }));
    }
}
