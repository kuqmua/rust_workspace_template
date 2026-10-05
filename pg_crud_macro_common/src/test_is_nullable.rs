#[test]
fn test_nullability_token_wrapping_preserves_nonnullable_and_wraps_nullable_tokens() {
    assert!([
        quote::quote! {},
        quote::quote! { std::collections::BTreeMap<i64, String> },
        quote::quote! { value.method() },
    ].into_iter().all(|tokens| {
        let generated = macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(tokens.clone());
        let unchanged_type = crate::is_nullable::IsNullable::False.maybe_optional_wrap(generated.clone());
        let unchanged_value = crate::is_nullable::IsNullable::False.maybe_some_wrap(generated.clone());
        let optional = crate::is_nullable::IsNullable::True.maybe_optional_wrap(generated.clone());
        let some = crate::is_nullable::IsNullable::True.maybe_some_wrap(generated);
        unchanged_type.to_string() == tokens.to_string()
            && unchanged_value.to_string() == tokens.to_string()
            && optional.to_string() == quote::quote! { Option<#tokens> }.to_string()
            && some.to_string() == quote::quote! { Some(#tokens) }.to_string()
    }));
}

#[test]
fn test_nullability_names_and_prefixes_match_generated_type_conventions() {
    assert_eq!(
        crate::is_nullable::IsNullable::default(),
        crate::is_nullable::IsNullable::False
    );
    assert!(
        [
            (
                crate::is_nullable::IsNullable::False,
                constants_str::NONNULL,
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
            ),
            (
                crate::is_nullable::IsNullable::True,
                constants_str::NULLABLE,
                constants_str::STDOPTIONALOPTIONAL,
                constants_str::OPTIONAL
            ),
        ]
        .into_iter()
        .all(|(nullable, name, prefix, rust_name)| {
            nullable.non_null_or_nullable_str().to_string() == name
                && nullable.prefix_str().to_string() == prefix
                && nullable.rust().to_string() == rust_name
        })
    );
}

#[test]
fn test_nullable_prefix_wrapper_preserves_empty_and_maximum_ascii_byte_lengths() {
    assert!(
        [
            0usize,
            1usize,
            crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN
        ]
        .into_iter()
        .all(|length| {
            crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(
                constants_str::SLASH.repeat(length),
            )
            .is_ok_and(|prefix| {
                let text = prefix.to_string();
                text.len() == length && text.bytes().all(|byte| byte == b'/')
            })
        })
    );
    assert!(
        crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(
            constants_str::SLASH
                .repeat(crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN + 1usize),
        )
        .is_err_and(|error| error
            .to_string()
            .contains(&crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN.to_string()))
    );
}

#[test]
fn test_nullable_prefix_wrapper_counts_utf8_bytes_and_preserves_allowed_null_characters() {
    let character_count = 262_144usize;
    assert_eq!(
        character_count * char::MAX.len_utf8(),
        crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN
    );
    assert!(
        crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(
            char::MAX.to_string().repeat(character_count)
        )
        .is_ok_and(|prefix| {
            let text = prefix.to_string();
            text.len() == crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN
                && text.chars().count() == character_count
                && text.chars().all(|character| character == char::MAX)
        })
    );
    assert!(
        crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(
            char::MAX.to_string().repeat(character_count + 1usize)
        )
        .is_err_and(|error| error
            .to_string()
            .contains(&crate::is_nl_prefix_str_max_len::IS_NL_PREFIX_STR_MAX_LEN.to_string()))
    );
    assert!(
        crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(char::from(0u8).to_string())
            .is_ok_and(|prefix| prefix
                .to_string()
                .chars()
                .eq(std::iter::once(char::from(0u8))))
    );
}
