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
    let Ok(args) = super::parse_contract_struct_api_args(super::SynAttributesRef::from(
        input.attrs.as_slice(),
    )) else {
        panic!("edc94d17");
    };
    assert!(bool::from(args.new));
    assert!(bool::from(args.into_parts));
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
            super::parse_contract_struct_api_field_args(super::SynAttributesRef::from(
                field.attrs.as_slice(),
            ))
            .map(|field_args| {
                (
                    bool::from(field_args.borrow),
                    bool::from(field_args.copy),
                    bool::from(field_args.into),
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
    let Err(error) = super::parse_contract_struct_api_args(super::SynAttributesRef::from(
        input.attrs.as_slice(),
    )) else {
        panic!("86b738e6");
    };
    assert!(
        error
            .to_string()
            .contains(str_constants::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE)
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
    ["", "error_policy = Policy, error_statuses = Statuses,"]
        .into_iter()
        .for_each(|errors| {
            let result = syn::parse_str::<super::TypedRouteArgs>(typed_route_args(errors).as_str());
            let Err(error) = result else {
                panic!("f58d0a31");
            };
            assert!(
                error
                    .to_string()
                    .contains(str_constants::TYPED_ROUTE_REQUIRES_ERROR_POLICY_OR_STATUSES)
            );
        });
    ["error_policy = Policy,", "error_statuses = Statuses,"]
        .into_iter()
        .for_each(|errors| {
            let Ok(_args) =
                syn::parse_str::<super::TypedRouteArgs>(typed_route_args(errors).as_str())
            else {
                panic!("470bf91c");
            };
        });
}

#[test]
fn route_registry_args_require_family_after_state() {
    let result = syn::parse_str::<super::RouteRegistryArgs>(
        "state = (), wrong = Family; (\"authenticated\", \"csrf\"); schemas(); (Route, handler),",
    );
    let Err(error) = result else {
        panic!("da287c44");
    };
    assert!(
        error
            .to_string()
            .contains(str_constants::ROUTE_REGISTRY_REQUIRES_FAMILY)
    );
}

#[test]
fn route_registry_args_parse_family_and_bindings() {
    let result = syn::parse_str::<super::RouteRegistryArgs>(
        "state = (), family = Family; (\"authenticated\", \"csrf\"); schemas(Schema); (Route, handler),",
    );
    let Ok(args) = result else {
        panic!("6282e207");
    };
    assert_eq!(args.bindings.0.len(), 1usize);
    assert_eq!(args.schemas.0.len(), 1usize);
    assert_eq!(
        quote::ToTokens::to_token_stream(&args.family.0).to_string(),
        str_constants::FAMILY_UPPER_CAMEL_CASE
    );
}
