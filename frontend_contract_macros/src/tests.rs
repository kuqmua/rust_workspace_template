#[test]
fn contract_struct_api_attributes_are_explicit() {
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
    let Ok(args) = super::parse_contract_struct_api_args(
        super::domain_types::SynAttributesRef::from(input.attrs.as_slice()),
    ) else {
        panic!("edc94d17");
    };
    assert!(bool::from(*args.get_new()));
    assert!(bool::from(*args.get_into_parts()));
    let syn::Data::Struct(data) = input.data else {
        panic!("eb3fcd83");
    };
    let syn::Fields::Named(fields) = data.fields else {
        panic!("55c90f04");
    };
    let parsed_result = fields
        .named
        .iter()
        .map(|field| {
            super::parse_contract_struct_api_field_args(
                super::domain_types::SynAttributesRef::from(field.attrs.as_slice()),
            )
            .map(|field_args| {
                (
                    bool::from(*field_args.get_borrow()),
                    bool::from(*field_args.get_copy()),
                    bool::from(*field_args.get_into()),
                )
            })
        })
        .collect::<syn::Result<Vec<_>>>();
    let Ok(parsed_fields) = parsed_result else {
        panic!("ceffbe6d");
    };
    assert_eq!(
        parsed_fields,
        vec![
            (true, false, false),
            (false, true, false),
            (false, false, true)
        ]
    );
}

#[test]
fn contract_struct_api_rejects_unknown_attributes() {
    let input: syn::DeriveInput = syn::parse_quote! {
        #[contract_struct_api(unknown)]
        struct Request {
            value: String,
        }
    };
    let Err(error) = super::parse_contract_struct_api_args(
        super::domain_types::SynAttributesRef::from(input.attrs.as_slice()),
    ) else {
        panic!("86b738e6");
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
fn typed_route_args_require_exactly_one_error_source() {
    [
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
        constants_str::VALUE_24AF98F3,
    ]
    .into_iter()
    .for_each(|errors| {
        let result = syn::parse_str::<super::domain_types::TypedRouteArgs>(
            typed_route_args(errors).as_str(),
        );
        let Err(error) = result else {
            panic!("f58d0a31");
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
            let Ok(_args) = syn::parse_str::<super::domain_types::TypedRouteArgs>(
                typed_route_args(errors).as_str(),
            ) else {
                panic!("470bf91c");
            };
        });
}

#[test]
fn route_registry_args_require_family_after_state() {
    let result =
        syn::parse_str::<super::domain_types::RouteRegistryArgs>(constants_str::VALUE_A19E6154);
    let Err(error) = result else {
        panic!("da287c44");
    };
    assert!(
        error
            .to_string()
            .contains(constants_str::ROUTE_REGISTRY_REQUIRES_FAMILY)
    );
}

#[test]
fn route_registry_args_parse_family_and_bindings() {
    let result =
        syn::parse_str::<super::domain_types::RouteRegistryArgs>(constants_str::VALUE_2497DABD);
    let Ok(args) = result else {
        panic!("6282e207");
    };
    assert_eq!(args.get_bindings().as_ref().len(), constants_usize::ONE);
    assert_eq!(args.get_schemas().as_ref().len(), constants_usize::ONE);
    assert_eq!(
        quote::ToTokens::to_token_stream(args.get_family().as_ref()).to_string(),
        constants_str::FAMILY_UPPER_CAMEL_CASE
    );
}
