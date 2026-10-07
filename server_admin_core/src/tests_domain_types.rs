#[cfg(test)]
mod tests {
    #[test]
    fn test_administrator_secret_text_enforces_internal_bound() {
        let at_limit = constants_str::A_ALT.repeat(constants_usize::VALUE_8_192);
        let secret = crate::secrecy_admin_string::SecrecyAdminString::try_from(at_limit.clone())
            .expect(constants_str::DIAGNOSTIC_6673B876);
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(&secret)
                .as_ref()
                .as_str(),
            at_limit.as_str()
        );
        assert_eq!(
            crate::secrecy_admin_string::SecrecyAdminString::try_from(
                constants_str::A_ALT.repeat(8_193usize)
            )
            .err(),
            Some(
                crate::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                    len: 8_193usize,
                    max: constants_usize::VALUE_8_192,
                }
            )
        );
    }
    #[test]
    fn test_administrator_secret_text_is_redacted_and_zeroizable() {
        let raw = constants_str::NEVER_PRINT_THIS_VALUE;
        let secret = crate::secrecy_admin_string::SecrecyAdminString::try_from(raw.to_owned())
            .expect(constants_str::DIAGNOSTIC_67B629E2);
        assert!(!format!("{secret:?}").contains(raw));
        let mut bounded = crate::std_admin_string::StdAdminString::try_from(raw.to_owned())
            .expect(constants_str::DIAGNOSTIC_201F3C4B);
        secrecy::zeroize::Zeroize::zeroize(&mut bounded);
        assert!(bounded.as_ref().is_empty());
    }
    #[test]
    fn test_administrator_resource_values_are_stable() {
        let positive =
            server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64::try_from(42i64)
                .expect(constants_str::DIAGNOSTIC_2570AF3B);
        assert_eq!(
            crate::std_admin_string::StdAdminString::from_positive_i64(positive).as_ref(),
            constants_str::VALUE_42
        );
        assert_eq!(
            crate::std_admin_string::StdAdminString::system_settings_resource().as_ref(),
            constants_str::VALUE_1
        );
        let uuid_value = uuid::Uuid::from_u128(0x0000_0000_0000_4000_8000_0000_0000_0001u128);
        let expected = uuid_value.to_string();
        let uuid = crate::uuid_admin_value::UuidAdminValue::from(uuid_value);
        assert_eq!(
            crate::std_admin_string::StdAdminString::from_uuid(uuid)
                .as_ref()
                .as_str(),
            expected.as_str()
        );
    }

    #[test]
    fn test_core_record_identifiers_enforce_positive_integer_bounds() {
        [i64::MIN, -1i64, 0i64, 1i64, 42i64, i64::MAX].into_iter().fold((), |(), value| {
            let expected = if value > 0i64 { Ok(value) } else { Err(crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid) };
            assert_eq!(crate::admin_user_record_id::AdminUserRecordId::try_from(value).map(|identifier| {
                assert_eq!(identifier.value().get(), value);
                assert_eq!(identifier.to_string(), value.to_string());
                identifier.get()
            }), expected);
            assert_eq!(crate::admin_role_record_id::AdminRoleRecordId::try_from(value).map(|identifier| {
                assert_eq!(identifier.value().get(), value);
                identifier.get()
            }), expected);
        });
    }

    #[test]
    fn test_core_record_identifier_deserialization_preserves_validation_and_native_type_errors() {
        fn assert_core_record_identifier_native_wire_errors<Factory, Deserializer>(factory: Factory)
        where
            Factory: Fn() -> Deserializer,
            Deserializer: serde::Deserializer<'static, Error = serde::de::value::Error>,
        {
            let expected = <i64 as serde::Deserialize>::deserialize(factory())
                .map_err(|error| error.to_string());
            assert!(expected.as_ref().is_err_and(|error| !error.is_empty()));
            assert_eq!(<crate::admin_user_record_id::AdminUserRecordId as serde::Deserialize>::deserialize(factory()).map(crate::admin_user_record_id::AdminUserRecordId::get).map_err(|error| error.to_string()), expected);
            assert_eq!(<crate::admin_role_record_id::AdminRoleRecordId as serde::Deserialize>::deserialize(factory()).map(crate::admin_role_record_id::AdminRoleRecordId::get).map_err(|error| error.to_string()), expected);
        }
        [i64::MIN, -1i64, 0i64, 1i64, 42i64, i64::MAX].into_iter().fold((), |(), value| {
            let deserialize_value = || serde::de::value::I64Deserializer::<serde::de::value::Error>::new(value);
            let expected = if value > 0i64 { Ok(value) } else { Err(crate::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error::Invalid.to_string()) };
            assert_eq!(<crate::admin_user_record_id::AdminUserRecordId as serde::Deserialize>::deserialize(deserialize_value()).map(crate::admin_user_record_id::AdminUserRecordId::get).map_err(|error| error.to_string()), expected);
            assert_eq!(<crate::admin_role_record_id::AdminRoleRecordId as serde::Deserialize>::deserialize(deserialize_value()).map(crate::admin_role_record_id::AdminRoleRecordId::get).map_err(|error| error.to_string()), expected);
        });
        assert_core_record_identifier_native_wire_errors(|| {
            serde::de::value::BoolDeserializer::new(false)
        });
        assert_core_record_identifier_native_wire_errors(|| {
            serde::de::value::U64Deserializer::new(u64::MAX)
        });
        assert_core_record_identifier_native_wire_errors(|| {
            serde::de::value::F64Deserializer::new(1.0f64)
        });
        assert_core_record_identifier_native_wire_errors(|| {
            serde::de::value::StrDeserializer::new(constants_str::VALUE_1)
        });
        assert_core_record_identifier_native_wire_errors(|| {
            serde::de::value::UnitDeserializer::new()
        });
    }
}
