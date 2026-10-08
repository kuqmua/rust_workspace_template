#[test]
fn test_string_path_adapter_preserves_canonical_segments_without_trailing_separator() {
    let segments = crate::string_syn_punct::string_syn_punct();
    let path_result = syn::parse2::<syn::Path>(quote::quote!(#segments));
    assert!(path_result.is_ok());
    assert_eq!(
        path_result.ok(),
        Some(syn::parse_quote!(std::string::String))
    );
}

#[test]
fn test_macro_text_views_preserve_borrowing_and_compile_error_literal_content() {
    [
        String::from(constants_str::EMPTY),
        String::from(constants_str::CURSOR_TEST_PAYLOAD),
        ['\u{e9}', '\0', '"', '\n'].into_iter().collect(),
    ]
    .into_iter()
    .fold((), |(), value| {
        let message = crate::compile_error_message::CompileErrorMessage::from(&value);
        let borrowed: &str = message.as_ref();
        assert_eq!(borrowed, value.as_str());
        assert!(std::ptr::eq(borrowed.as_ptr(), value.as_ptr()));
        let expected_file_content_ref =
            crate::expected_file_content_ref::ExpectedFileContentRef::from(&value);
        let content = <&str>::from(expected_file_content_ref);
        assert_eq!(content, value.as_str());
        assert!(std::ptr::eq(content.as_ptr(), value.as_ptr()));
        assert!(
            [
                crate::expected_file_content::ExpectedFileContent::new(&value),
                crate::expected_file_content::ExpectedFileContent::new(value.as_str()),
                crate::expected_file_content::ExpectedFileContent::new(expected_file_content_ref),
            ]
            .into_iter()
            .all(
                |expected_file_content| expected_file_content.as_ref() == value.as_str()
                    && std::ptr::eq(expected_file_content.as_ref().as_ptr(), value.as_ptr())
            )
        );
        let tokens = crate::macro_compile_error_tokens::macro_compile_error_tokens(message);
        assert!(
            syn::parse2::<syn::ItemMacro>(quote::quote!(#tokens)).is_ok_and(|item_macro| {
                item_macro.mac.path.is_ident(stringify!(compile_error))
                    && item_macro.semi_token.is_some()
                    && syn::parse2::<syn::LitStr>(item_macro.mac.tokens)
                        .is_ok_and(|lit_str| lit_str.value() == value)
            })
        );
    });
}

#[test]
fn test_file_assertion_path_views_preserve_borrowed_path_storage() {
    [
        std::path::PathBuf::from(constants_str::EMPTY),
        std::path::PathBuf::from(constants_str::SRC_LIB_RS),
        std::path::PathBuf::from(
            ['\u{e9}', ' ', '/', '.', '/', 'x']
                .into_iter()
                .collect::<String>(),
        ),
    ]
    .into_iter()
    .fold((), |(), path_buf| {
        let assert_file_path_ref = crate::assert_file_path_ref::AssertFilePathRef::from(&path_buf);
        let path = <&std::path::Path>::from(assert_file_path_ref);
        assert_eq!(path.as_os_str(), path_buf.as_os_str());
        assert!(std::ptr::eq(path, path_buf.as_path()));
        assert!(
            [
                crate::std_assert_file_path::StdAssertFilePath::new(&path_buf),
                crate::std_assert_file_path::StdAssertFilePath::new(path_buf.as_path()),
                crate::std_assert_file_path::StdAssertFilePath::new(assert_file_path_ref),
            ]
            .into_iter()
            .all(
                |std_assert_file_path| std_assert_file_path.as_ref().as_os_str()
                    == path_buf.as_os_str()
                    && std::ptr::eq(std_assert_file_path.as_ref(), path_buf.as_path())
            )
        );
    });
}

#[test]
fn test_location_field_factory_preserves_private_named_field_contract() {
    let field = syn::Field::from(crate::location_syn_field::location_syn_field());
    let expected: syn::Field = syn::parse_quote!(location: location_lib::location::Location);
    assert_eq!(field, expected);
}

#[test]
fn test_field_location_coordinates_preserve_bounds_and_first_values() {
    assert_eq!(
        crate::field_location_column::FieldLocationColumn::first().value(),
        1u32
    );
    assert_eq!(
        crate::field_location_line::FieldLocationLine::first().value(),
        1u32
    );
    assert!([0u32, 1u32, u32::MAX].into_iter().all(|value| {
        [
            crate::field_location_column::FieldLocationColumn::try_from(value).map(|field_location_column| field_location_column.value() == value),
            crate::field_location_line::FieldLocationLine::try_from(value).map(|field_location_line| field_location_line.value() == value),
        ]
        .into_iter()
        .all(|result| if value == 0u32 {
            result == Err(crate::field_location_coordinate_try_from_u32_error::FieldLocationCoordinateTryFromU32Error::OutOfRange)
        } else {
            result == Ok(true)
        })
    }));
}

#[test]
fn test_external_text_adapters_preserve_os_text_and_borrowed_url_storage() {
    [
        constants_str::EMPTY.to_owned(),
        constants_str::POSTGRES_USER_SECRET_LOCALHOST_TEST.to_owned(),
        ['\u{e9}', '\0', ' ', '\n'].into_iter().collect::<String>(),
    ]
    .into_iter()
    .fold((), |(), value| {
        let os_string_value = crate::os_string_value::OsStringValue::from(value.as_str());
        assert_eq!(
            os_string_value.as_os_str(),
            std::ffi::OsStr::new(value.as_str())
        );
        #[cfg(feature = "test-utils")]
        {
            let url_ref = crate::url_ref::UrlRef::from(value.as_str());
            let copied_url_ref = url_ref;
            assert_eq!(url_ref.as_str(), value.as_str());
            assert!(std::ptr::eq(url_ref.as_str().as_ptr(), value.as_ptr()));
            assert_eq!(copied_url_ref.as_str(), value.as_str());
            assert!(std::ptr::eq(
                copied_url_ref.as_str().as_ptr(),
                value.as_ptr()
            ));
        }
    });
}

#[test]
fn test_syn_variant_view_preserves_exact_borrowed_variant_storage() {
    [
        syn::parse_quote!(X),
        syn::parse_quote!(
            #[test_marker(value)]
            X = 3
        ),
        syn::parse_quote!(X(String, Option<u32>)),
        syn::parse_quote!(X {
            value: std::string::String
        }),
    ]
    .into_iter()
    .fold((), |(), variant| {
        let syn_variant_ref = crate::syn_variant_ref::SynVariantRef::from(&variant);
        let copied_syn_variant_ref = syn_variant_ref;
        assert_eq!(syn_variant_ref.variant(), &variant);
        assert!(std::ptr::eq(
            syn_variant_ref.variant(),
            std::ptr::from_ref(&variant)
        ));
        assert_eq!(copied_syn_variant_ref.variant(), &variant);
        assert!(std::ptr::eq(
            copied_syn_variant_ref.variant(),
            std::ptr::from_ref(&variant)
        ));
    });
}
