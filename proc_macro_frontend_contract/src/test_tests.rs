#[test]
fn test_contract_struct_api_attributes_are_explicit() {
    let input: syn::DeriveInput = syn::parse_quote! {
        #[contract_struct_api(new, into_parts)]
        struct Request {
            #[contract_struct_api(borrow)]
            name: String,
            #[contract_struct_api(copy)]
            enabled: bool,
            #[contract_struct_api(into)]
            value: String,
        }
    };
    let Ok(args) = crate::parse_contract_struct_api_args(
        crate::syn_attributes_ref::SynAttributesRef::from(input.attrs.as_slice()),
    ) else {
        std::panic::panic_any(constants_str::PANIC_EDC94D17);
    };
    assert!(bool::from(*args.get_generate_constructor()));
    assert!(bool::from(*args.get_into_parts()));
    let syn::Data::Struct(data) = input.data else {
        std::panic::panic_any(constants_str::PANIC_EB3FCD83);
    };
    let syn::Fields::Named(fields) = data.fields else {
        std::panic::panic_any(constants_str::PANIC_55C90F04);
    };
    let parsed_result = fields
        .named
        .iter()
        .map(|field| {
            let attributes =
                crate::syn_attributes_ref::SynAttributesRef::from(field.attrs.as_slice());
            let mut field_args =
                crate::contract_struct_api_field_args::ContractStructApiFieldArgs::default();
            attributes
                .get()
                .iter()
                .filter(|attribute| {
                    attribute
                        .path()
                        .is_ident(constants_str::CONTRACT_STRUCT_API)
                })
                .try_for_each(|attribute| {
                    attribute.parse_nested_meta(|metadata| {
                        if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_BORROW)
                        {
                            *field_args.get_borrow_mut() = crate::std_bool::StdBool::from(true);
                            Ok(())
                        } else if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_COPY)
                        {
                            *field_args.get_copy_mut() = crate::std_bool::StdBool::from(true);
                            Ok(())
                        } else if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_COPY_REF)
                        {
                            *field_args.get_copy_ref_mut() = crate::std_bool::StdBool::from(true);
                            Ok(())
                        } else if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_INTO)
                        {
                            *field_args.get_into_mut() = crate::std_bool::StdBool::from(true);
                            Ok(())
                        } else if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_OPTION_BORROW)
                        {
                            *field_args.get_option_borrow_mut() =
                                crate::std_bool::StdBool::from(true);
                            Ok(())
                        } else if metadata
                            .path
                            .is_ident(constants_str::CONTRACT_STRUCT_API_SLICE)
                        {
                            *field_args.get_slice_mut() =
                                Some(crate::contract_syn_type::ContractSynType::from(
                                    metadata.value()?.parse::<syn::Type>()?,
                                ));
                            Ok(())
                        } else {
                            Err(metadata
                                .error(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE))
                        }
                    })
                })
                .map(|()| {
                    (
                        bool::from(*field_args.get_borrow()),
                        bool::from(*field_args.get_copy()),
                        bool::from(*field_args.get_into()),
                    )
                })
        })
        .collect::<syn::Result<Vec<_>>>();
    let Ok(parsed_fields) = parsed_result else {
        std::panic::panic_any(constants_str::PANIC_CEFFBE6D);
    };
    assert_eq!(
        parsed_fields,
        [
            (true, false, false),
            (false, true, false),
            (false, false, true)
        ]
    );
}

#[test]
fn test_contract_struct_api_rejects_unknown_attributes() {
    let input: syn::DeriveInput = syn::parse_quote! {
        #[contract_struct_api(unknown)]
        struct Request {
            value: String,
        }
    };
    let Err(error) = crate::parse_contract_struct_api_args(
        crate::syn_attributes_ref::SynAttributesRef::from(input.attrs.as_slice()),
    ) else {
        std::panic::panic_any(constants_str::PANIC_86B738E6);
    };
    assert!(
        error
            .to_string()
            .contains(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE)
    );
}

fn typed_route_args(errors: &str) -> String {
    format!(
        "authentication = Authentication, {errors} method = Method, openapi_operation_id = \"operation\", path = \"/path\", request = Request, response = Response, success_status = Status, transport = Transport"
    )
}

#[test]
#[allow(clippy::needless_for_each)] // iterator form follows the workspace no-for-loop policy
fn test_typed_route_args_require_exactly_one_error_source() {
    [
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
        constants_str::VALUE_24AF98F3,
    ]
    .into_iter()
    .for_each(|errors| {
        let result = syn::parse_str::<crate::typed_route_args::TypedRouteArgs>(
            typed_route_args(errors).as_str(),
        );
        let Err(error) = result else {
            std::panic::panic_any(constants_str::PANIC_F58D0A31);
        };
        assert!(
            error
                .to_string()
                .contains(constants_str::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES)
        );
    });
    [constants_str::VALUE_5D5703CD, constants_str::VALUE_240525BC]
        .into_iter()
        .for_each(|errors| {
            let Ok(_args) = syn::parse_str::<crate::typed_route_args::TypedRouteArgs>(
                typed_route_args(errors).as_str(),
            ) else {
                std::panic::panic_any(constants_str::PANIC_470BF91C);
            };
        });
}

#[test]
fn test_route_registry_args_require_family_after_state() {
    let result = syn::parse_str::<crate::route_registry_args::RouteRegistryArgs>(
        constants_str::VALUE_A19E6154,
    );
    let Err(error) = result else {
        std::panic::panic_any(constants_str::PANIC_DA287C44);
    };
    assert!(
        error
            .to_string()
            .contains(constants_str::ROUTE_REGISTRY_REQUIRES_FAMILY)
    );
}

#[test]
fn test_route_registry_args_parse_family_and_bindings() {
    let result = syn::parse_str::<crate::route_registry_args::RouteRegistryArgs>(
        constants_str::VALUE_2497DABD,
    );
    let Ok(args) = result else {
        std::panic::panic_any(constants_str::PANIC_6282E207);
    };
    assert_eq!(args.get_bindings().as_ref().len(), constants_usize::ONE);
    assert_eq!(args.get_schemas().as_ref().len(), constants_usize::ONE);
    assert_eq!(
        quote::ToTokens::to_token_stream(args.get_family().as_ref()).to_string(),
        constants_str::FAMILY_UPPER_CAMEL_CASE
    );
}
