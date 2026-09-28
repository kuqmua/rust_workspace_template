#[test]
fn test_enum_naming_impls_preserve_const_generics_and_where_clauses() {
    let input = quote::quote! {
        enum NamingGenericFixture<const LIMIT: usize = 4usize>
        where [(); LIMIT]: Sized {
            HelloWorld,
        }
    };
    let generated = [
        crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone()),
        crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
        crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone()),
        crate::enum_with_unit_fields_to_snake_case_str(input.clone()),
        crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
        crate::enum_with_unit_fields_to_upper_snake_case_str(input),
    ];
    assert!(generated.into_iter().all(|tokens| {
        syn::parse2::<syn::File>(tokens).is_ok_and(|file| {
            file.items.len() == constants_usize::ONE && file.items.into_iter().all(|item| {
                matches!(item, syn::Item::Impl(item_impl)
                    if item_impl.generics.params.len() == constants_usize::ONE
                        && matches!(item_impl.generics.params.first(), Some(syn::GenericParam::Const(parameter)) if parameter.default.is_none())
                        && item_impl.generics.where_clause.as_ref().is_some_and(|where_clause| where_clause.predicates.len() == constants_usize::ONE)
                        && matches!(item_impl.self_ty.as_ref(), syn::Type::Path(path) if path.path.segments.last().is_some_and(|segment| matches!(&segment.arguments, syn::PathArguments::AngleBracketed(arguments) if arguments.args.len() == constants_usize::ONE))))
            })
        })
    }));
}

#[test]
fn test_enum_naming_impls_report_oversized_case_conversion() {
    let variant_identifier = syn::Ident::new(
        constants_str::HELLOWORLD_ALT.repeat(104_858usize).as_str(),
        proc_macro2::Span::call_site(),
    );
    let input = quote::quote! {
        enum OversizedCaseFixture { #variant_identifier }
    };
    assert!(
        [
            crate::as_ref_str_enum_with_unit_fields_to_snake_case_str(input.clone()),
            crate::as_ref_str_enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
            crate::as_ref_str_enum_with_unit_fields_to_upper_snake_case_str(input.clone()),
            crate::enum_with_unit_fields_to_snake_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_camel_case_str(input.clone()),
            crate::enum_with_unit_fields_to_upper_snake_case_str(input),
        ]
        .into_iter()
        .all(|tokens| tokens.to_string().contains(constants_str::VALUE_2EDAC0BF))
    );
}
