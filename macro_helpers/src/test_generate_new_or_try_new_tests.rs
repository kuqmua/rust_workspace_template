fn cmpct(str: &str) -> String {
    str.split_whitespace().collect::<String>()
}
fn empty_token_stream() -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}
#[test]
fn test_generate_impl_new_for_identifier_token_stream_generates_non_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_48495BE4);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_DB75B4FB);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_7AD6DD07);
    let tokens = crate::generate_impl_new_for_identifier_token_stream_impl::generate_impl_new_for_identifier_token_stream_impl(
        &identifier,
        &empty_token_stream(),
        &parameters,
        &body,
    );
    assert_eq!(
        cmpct(&tokens.to_string()),
        cmpct(constants_str::VALUE_87685B6B)
    );
}
#[test]
fn test_generate_impl_const_new_for_identifier_token_stream_generates_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_7795AF9B);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_28CCDFC4);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_46FB1C80);
    let tokens = crate::generate_impl_const_new_for_identifier_token_stream_impl::generate_impl_const_new_for_identifier_token_stream_impl(
        &identifier,
        &empty_token_stream(),
        &parameters,
        &body,
    );
    assert_eq!(
        cmpct(&tokens.to_string()),
        cmpct(constants_str::VALUE_C3851857)
    );
}
#[test]
fn test_generate_impl_pub_const_new_for_identifier_token_stream_generates_pub_const_new() {
    let identifier: proc_macro2::TokenStream = constants_str::CFG
        .parse()
        .expect(constants_str::DIAGNOSTIC_4AFBE04B);
    let attr: proc_macro2::TokenStream = constants_str::INLINE
        .parse()
        .expect(constants_str::DIAGNOSTIC_5CFDE4DD);
    let parameters: proc_macro2::TokenStream = constants_str::V_USIZE
        .parse()
        .expect(constants_str::DIAGNOSTIC_4304AB24);
    let body: proc_macro2::TokenStream = constants_str::SELF_V
        .parse()
        .expect(constants_str::DIAGNOSTIC_29AC89D5);
    let tokens = crate::generate_impl_pub_const_new_for_identifier_token_stream_impl::generate_impl_pub_const_new_for_identifier_token_stream_impl(
        &identifier,
        &attr,
        &parameters,
        &body,
    );
    assert_eq!(
        cmpct(&tokens.to_string()),
        cmpct(constants_str::VALUE_BA9AA4C0)
    );
}

#[test]
fn test_constructor_impl_adapters_preserve_modifiers_attributes_and_bodies() {
    let identifier = quote::quote!(Value);
    let attributes = quote::quote!(#[must_use] #[inline]);
    let parameters = quote::quote!(value: Inner);
    let error_type = quote::quote!(ValidationError);
    let body = quote::quote!(Ok(Self(value)));
    [
        (crate::generate_impl_pub_const_try_new_for_identifier_token_stream_impl::generate_impl_pub_const_try_new_for_identifier_token_stream_impl(&attributes, &identifier, &parameters, &error_type, &body), quote::quote! {
            impl Value {
                #[must_use] #[inline]
                pub const fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
            }
        }),
        (crate::generate_impl_try_new_for_identifier_token_stream_impl::generate_impl_try_new_for_identifier_token_stream_impl(&attributes, &identifier, &parameters, &error_type, &body), quote::quote! {
            impl Value {
                #[must_use] #[inline]
                fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
            }
        }),
        (crate::generate_impl_pub_new_for_identifier_token_stream_impl::generate_impl_pub_new_for_identifier_token_stream_impl(&identifier, &attributes, &parameters, &quote::quote!(Self(value))), quote::quote! {
            impl Value {
                #[must_use] #[inline]
                pub fn new(value: Inner) -> Self { Self(value) }
            }
        }),
    ].into_iter().fold((), |(), (generated, expected_tokens)| {
        let observed_result = syn::parse2::<syn::ItemImpl>(generated.as_ref().clone());
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::ItemImpl>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_fallible_constructor_method_adapters_preserve_const_visibility_and_errors() {
    let attributes = quote::quote!(#[must_use] #[inline]);
    let parameters = quote::quote!(value: Inner);
    let error_type = quote::quote!(ValidationError);
    let body = quote::quote!(Ok(Self(value)));
    [
        (crate::generate_const_try_new_token_stream_impl::generate_const_try_new_token_stream_impl(&attributes, &parameters, &error_type, &body), quote::quote! {
            #[must_use] #[inline]
            const fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
        }),
        (crate::generate_pub_const_try_new_token_stream_impl::generate_pub_const_try_new_token_stream_impl(&attributes, &parameters, &error_type, &body), quote::quote! {
            #[must_use] #[inline]
            pub const fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
        }),
    ].into_iter().fold((), |(), (generated, expected_tokens)| {
        let observed_result = syn::parse2::<syn::ImplItemFn>(generated.as_ref().clone());
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::ImplItemFn>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_remaining_constructor_adapters_preserve_method_visibility_and_return_types() {
    let attributes = quote::quote!(#[must_use] #[inline]);
    let parameters = quote::quote!(value: Inner);
    let body = quote::quote!(Self(value));
    [
        (crate::generate_const_new_token_stream_impl::generate_const_new_token_stream_impl(&attributes, &parameters, &body), quote::quote! {
            #[must_use] #[inline]
            const fn new(value: Inner) -> Self { Self(value) }
        }),
        (crate::generate_pub_new_token_stream_impl::generate_pub_new_token_stream_impl(&attributes, &parameters, &body), quote::quote! {
            #[must_use] #[inline]
            pub fn new(value: Inner) -> Self { Self(value) }
        }),
        (crate::generate_pub_const_new_token_stream_impl::generate_pub_const_new_token_stream_impl(&attributes, &parameters, &body), quote::quote! {
            #[must_use] #[inline]
            pub const fn new(value: Inner) -> Self { Self(value) }
        }),
        (crate::generate_pub_try_new_token_stream_impl::generate_pub_try_new_token_stream_impl(&attributes, &parameters, &quote::quote!(ValidationError), &quote::quote!(Ok(Self(value)))), quote::quote! {
            #[must_use] #[inline]
            pub fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
        }),
    ].into_iter().fold((), |(), (generated, expected_tokens)| {
        let observed_result = syn::parse2::<syn::ImplItemFn>(generated.as_ref().clone());
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::ImplItemFn>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
    let generated = crate::generate_impl_pub_try_new_for_identifier_token_stream_impl::generate_impl_pub_try_new_for_identifier_token_stream_impl(&attributes, &quote::quote!(Value), &parameters, &quote::quote!(ValidationError), &quote::quote!(Ok(Self(value))));
    let expected: syn::ItemImpl = syn::parse_quote! {
        impl Value {
            #[must_use] #[inline]
            pub fn try_new(value: Inner) -> Result<Self, ValidationError> { Ok(Self(value)) }
        }
    };
    let observed_result = syn::parse2::<syn::ItemImpl>(generated.as_ref().clone());
    assert!(observed_result.is_ok());
    assert_eq!(observed_result.ok(), Some(expected));
}
