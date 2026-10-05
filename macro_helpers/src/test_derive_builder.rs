#[test]
fn test_derive_builder_defaults_and_conditional_selections_preserve_visibility_and_order() {
    let private = crate::derive_token_stream_builder::DeriveTokenStreamBuilder::new()
        .make_pub_if(crate::derive_token_stream_builder::MakePub::False)
        .derive_debug_if(crate::derive_token_stream_builder::DeriveDebug::False);
    let selected = crate::derive_token_stream_builder::DeriveTokenStreamBuilder::default()
        .derive_clone_if(crate::derive_token_stream_builder::DeriveClone::True)
        .derive_debug_if(crate::derive_token_stream_builder::DeriveDebug::True)
        .derive_debug_if(crate::derive_token_stream_builder::DeriveDebug::False)
        .derive_clone()
        .make_pub_if(crate::derive_token_stream_builder::MakePub::True)
        .make_pub_if(crate::derive_token_stream_builder::MakePub::False);
    [
        (
            private.build_struct(
                &quote::quote!(#[repr(C)]),
                &quote::quote!(Value),
                &quote::quote!(<T>),
                &quote::quote!({ value: Inner<T> }),
            ),
            quote::quote! {
                #[derive()] #[repr(C)] struct Value<T> { value: Inner<T> }
            },
        ),
        (
            selected.build_enum(
                &quote::quote!(#[repr(C)]),
                &quote::quote!(Value),
                &quote::quote!(<T>),
                &quote::quote!({ First(Inner<T>), Second }),
            ),
            quote::quote! {
                #[derive(Debug, Clone)] #[repr(C)] pub enum Value<T> { First(Inner<T>), Second }
            },
        ),
    ]
    .into_iter()
    .fold((), |(), (generated, expected_tokens)| {
        let observed_result = syn::parse2::<syn::Item>(generated);
        assert!(observed_result.is_ok());
        let expected_result = syn::parse2::<syn::Item>(expected_tokens);
        assert!(expected_result.is_ok());
        assert_eq!(observed_result.ok(), expected_result.ok());
    });
}

#[test]
fn test_derive_builder_all_traits_preserve_catalog_order_and_original_declaration() {
    let builder = crate::derive_token_stream_builder::DeriveTokenStreamBuilder::new()
        .make_pub()
        .derive_proc_macro_location_derive_location_location()
        .derive_thiserror_error()
        .derive_schemars_json_schema()
        .derive_utoipa_to_schema()
        .derive_serde_deserialize()
        .derive_serde_serialize()
        .derive_ord()
        .derive_partial_ord()
        .derive_std_hash_hash()
        .derive_eq()
        .derive_partial_eq()
        .derive_copy()
        .derive_clone()
        .derive_default()
        .derive_debug();
    let output = builder.build_struct(
        &quote::quote!(#[repr(C)]),
        &quote::quote!(Value),
        &quote::quote!(<T>),
        &quote::quote!({ value: Inner<T> }),
    );
    let expected: syn::ItemStruct = syn::parse_quote! {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, std::hash::Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema, thiserror::Error, proc_macro_location_derive_location::Location)]
        #[repr(C)]
        pub struct Value<T> { value: Inner<T> }
    };
    let observed_result = syn::parse2::<syn::ItemStruct>(output);
    assert!(observed_result.is_ok());
    assert_eq!(observed_result.ok(), Some(expected));
}
