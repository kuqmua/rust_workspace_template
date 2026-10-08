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

#[test]
fn test_contract_struct_api_rejects_duplicate_slice_type() {
    let output = crate::derive_contract_struct_api(quote::quote! {
        #[contract_struct_api(new)]
        struct Request {
            #[contract_struct_api(slice = u8, slice = u16)]
            value: Vec<u8>,
        }
    });
    assert!(
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    );
}

#[test]
fn test_route_openapi_rejects_duplicate_delegate() {
    let output = crate::route_openapi(
        quote::quote! { delegate = first, delegate = second },
        quote::quote! { async fn endpoint() -> Result<(), Error> {} },
    );
    assert!(
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    );
}

#[test]
fn test_route_family_rejects_duplicate_attributes() {
    let outputs = [
        crate::derive_route_family(quote::quote! {
            #[route_family(First)]
            #[route_family(Second)]
            struct Family;
        }),
        crate::derive_route_family(quote::quote! {
            #[route_family(Route)]
            #[route_family_body_limit(1)]
            #[route_family_body_limit(2)]
            struct Family;
        }),
    ];
    assert!(outputs.into_iter().all(|output| {
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    }));
}

#[test]
fn test_contract_derives_reject_duplicate_top_level_attributes() {
    let outputs = [
        crate::derive_typed_route(quote::quote! {
            #[typed_route()]
            #[typed_route()]
            struct Route;
        }),
        crate::derive_route_catalog(quote::quote! {
            #[route_catalog()]
            #[route_catalog()]
            enum Catalog {}
        }),
        crate::derive_page_catalog(quote::quote! {
            #[page_catalog()]
            #[page_catalog()]
            enum Catalog {}
        }),
    ];
    assert!(outputs.into_iter().all(|output| {
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    }));
}

#[test]
fn test_contract_catalogs_reject_duplicate_variant_attributes() {
    let outputs = [
        crate::derive_route_catalog(quote::quote! {
            #[route_catalog(family = Family, body_limit = 1)]
            enum Catalog {
                #[route_catalog_route()]
                #[route_catalog_route()]
                Route,
            }
        }),
        crate::derive_page_catalog(quote::quote! {
            #[page_catalog(spec = Spec, path_ref = PathRef, inventory = INVENTORY)]
            enum Catalog {
                #[page_catalog_page()]
                #[page_catalog_page()]
                Page,
            }
        }),
    ];
    assert!(outputs.into_iter().all(|output| {
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    }));
}

#[test]
fn test_route_registry_rejects_duplicate_openapi_attributes() {
    let output = crate::route_registry(quote::quote! {
        #[openapi()]
        #[openapi()]
        pub;
        state = State, family = Family;
        (authenticated, csrf);
        schemas(Schema);
        (Route, endpoint)
    });
    assert!(
        output
            .to_string()
            .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
    );
}

#[test]
fn test_route_registry_rejects_unsupported_outer_attribute() {
    let output = crate::route_registry(quote::quote! {
        #[openapi()]
        #[unsupported]
        pub;
        state = State, family = Family;
        (authenticated, csrf);
        schemas(Schema);
        (Route, endpoint)
    });
    assert!(
        output
            .to_string()
            .contains(constants_str::ROUTE_REGISTRY_UNSUPPORTED_ATTRIBUTE)
    );
}

fn typed_route_args(str: &str) -> String {
    format!(
        "authentication = Authentication, {str} method = Method, openapi_operation_id = \"operation\", path = \"/path\", request = Request, response = Response, success_status = Status, transport = Transport"
    )
}

#[test]
#[allow(
    clippy::needless_for_each,
    reason = "test tests uses iterator traversal to comply with the workspace no-for-loop policy"
)]
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
fn test_duplicate_contract_attribute_fields_are_rejected() {
    let results = [
        syn::parse2::<crate::typed_route_args::TypedRouteArgs>(quote::quote! {
            authentication = Authentication, error_policy = Policy,
            method = First, method = Second, openapi_operation_id = Operation,
            path = Path, request = Request, response = Response,
            success_status = Status, transport = Transport
        })
        .map(|_args| ()),
        syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote! {
            spec = First, spec = Second, path_ref = Path, inventory = Inventory
        })
        .map(|_args| ()),
        syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote! {
            capability = First, capability = Second, metadata = Metadata,
            path = Path, route = Route, title = Title
        })
        .map(|_args| ()),
        syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote! {
            family = First, family = Second, body_limit = Limit
        })
        .map(|_args| ()),
        syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(quote::quote! {
            contract = First, contract = Second, path = Path
        })
        .map(|_args| ()),
        syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(quote::quote! {
            contract = Contract, path = Path, exclude_from_family, exclude_from_family
        })
        .map(|_args| ()),
    ];
    assert!(results.into_iter().all(|result| {
        result.is_err_and(|error| {
            error
                .to_string()
                .contains(constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD)
        })
    }));
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

#[test]
fn test_endpoint_registry_rejects_missing_state_and_empty_bindings() {
    let cases = [
        (
            quote::quote! { pub; other = State; (Contract, endpoint) },
            constants_str::ENDPOINT_REGISTRY_REQUIRES_STATE,
        ),
        (
            quote::quote! { pub; state = State; },
            constants_str::ENDPOINT_REGISTRY_REQUIRES_BINDING,
        ),
    ];
    assert!(cases.into_iter().all(|(input, expected)| {
        crate::endpoint_registry(input)
            .to_string()
            .contains(expected)
    }));
}

#[test]
fn test_route_openapi_rejects_nonpath_delegate_forms() {
    let attributes = [
        quote::quote! { delegate },
        quote::quote! { delegate(first) },
        quote::quote! { delegate = 1 },
    ];
    assert!(attributes.into_iter().all(|attribute| {
        crate::route_openapi(
            attribute,
            quote::quote! { fn endpoint() -> Result<(), Error> { Ok(()) } },
        )
        .to_string()
        .contains(constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_PATH)
    }));
}

#[test]
fn test_endpoint_registry_generates_router_for_each_binding() {
    let output = crate::endpoint_registry(quote::quote! {
        pub; state = State; (First, first), (Second, second)
    });
    assert!(syn::parse2::<syn::ItemFn>(output).is_ok_and(|function| {
        matches!(function.vis, syn::Visibility::Public(_))
            && function.sig.inputs.is_empty()
            && function.block.stmts.len() == 4usize
            && function.block.stmts.iter().skip(1usize).take(2usize).zip([
                (quote::quote!(First), quote::quote!(first)),
                (quote::quote!(Second), quote::quote!(second)),
            ]).all(|(statement, (contract, endpoint))| {
                let text = quote::quote!(#statement).to_string();
                text.contains(quote::quote!(
                    frontend_contract::route_registration_contract::RouteRegistrationContract::registration_path(#contract)
                ).to_string().as_str())
                    && text.contains(quote::quote!(
                        frontend_contract::route_method_router::route_method_router(
                            frontend_contract::route_registration_contract::RouteRegistrationContract::registration_method(#contract),
                            #endpoint,
                        )
                    ).to_string().as_str())
            })
    }));
}

#[test]
fn test_route_openapi_delegate_rejects_nonempty_body() {
    let output = crate::route_openapi(
        quote::quote! { delegate = implementation },
        quote::quote! { fn endpoint() -> Result<(), Error> { Ok(()) } },
    );
    assert!(
        output
            .to_string()
            .contains(constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_EMPTY_BODY)
    );
}

#[test]
fn test_route_openapi_delegate_rejects_nonidentifier_parameters() {
    let inputs = [
        quote::quote! { fn endpoint(&self) -> Result<(), Error> {} },
        quote::quote! { fn endpoint((first, second): Pair) -> Result<(), Error> {} },
        quote::quote! { fn endpoint(value @ Some(_): Option<Value>) -> Result<(), Error> {} },
    ];
    assert!(inputs.into_iter().all(|input| {
        crate::route_openapi(quote::quote! { delegate = implementation }, input)
            .to_string()
            .contains(constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_IDENT_PARAMETERS)
    }));
}

#[test]
fn test_route_openapi_delegate_requires_result_with_error_type() {
    let inputs = [
        quote::quote! { fn endpoint() {} },
        quote::quote! { fn endpoint() -> () {} },
        quote::quote! { fn endpoint() -> Response {} },
        quote::quote! { fn endpoint() -> Result {} },
        quote::quote! { fn endpoint() -> Result<Response> {} },
        quote::quote! { fn endpoint() -> Result<Response, 1> {} },
    ];
    assert!(inputs.into_iter().all(|input| {
        crate::route_openapi(quote::quote! { delegate = implementation }, input)
            .to_string()
            .contains(constants_str::ROUTE_OPENAPI_DELEGATE_REQUIRES_RESULT)
    }));
}

#[test]
fn test_route_openapi_delegate_forwards_arguments_and_converts_error() {
    let output = crate::route_openapi(
        quote::quote! { delegate = implementation },
        quote::quote! { pub async fn endpoint(first: First, second: Second) -> Result<Response, Error> {} },
    );
    assert!(syn::parse2::<syn::ItemFn>(output).is_ok_and(|function| {
        let block = function.block;
        function.sig.asyncness.is_some()
            && function.sig.inputs.len() == 2usize
            && function.attrs.len() == 2usize
            && quote::quote!(#block).to_string()
                == quote::quote!({ implementation(first, second).await.map_err(Error::from) })
                    .to_string()
    }));
}

#[test]
fn test_contract_struct_api_rejects_non_named_struct_inputs() {
    let inputs = [
        quote::quote! { enum Request { Value } },
        quote::quote! { struct Request; },
        quote::quote! { struct Request(Value); },
    ];
    assert!(inputs.into_iter().all(|input| {
        crate::derive_contract_struct_api(input)
            .to_string()
            .contains(constants_str::CONTRACT_STRUCT_API_REQUIRES_NAMED_STRUCT)
    }));
}

#[test]
fn test_contract_struct_api_expansion_rejects_unknown_struct_and_field_attributes() {
    let inputs = [
        quote::quote! { #[contract_struct_api(unknown)] struct Request { value: Value } },
        quote::quote! { struct Request { #[contract_struct_api(unknown)] value: Value } },
    ];
    assert!(inputs.into_iter().all(|input| {
        crate::derive_contract_struct_api(input)
            .to_string()
            .contains(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE)
    }));
}

#[test]
fn test_contract_struct_api_option_borrow_rejects_missing_inner_type() {
    let inputs = [
        quote::quote! { struct Request { #[contract_struct_api(option_borrow)] value: () } },
        quote::quote! { struct Request { #[contract_struct_api(option_borrow)] value: Option } },
        quote::quote! { struct Request { #[contract_struct_api(option_borrow)] value: Option<1> } },
    ];
    assert!(inputs.into_iter().all(|input| {
        crate::derive_contract_struct_api(input)
            .to_string()
            .contains(constants_str::CONTRACT_STRUCT_API_UNSUPPORTED_ATTRIBUTE)
    }));
}

#[test]
fn test_contract_struct_api_generates_all_accessor_forms_and_preserves_generics() {
    let output = crate::derive_contract_struct_api(quote::quote! {
        #[contract_struct_api(new, into_parts)]
        struct Request<Value> where Value: Copy {
            #[contract_struct_api(borrow)] borrowed: Value,
            #[contract_struct_api(copy)] copied: Value,
            #[contract_struct_api(copy_ref)] copied_ref: Value,
            #[contract_struct_api(into)] consumed: Value,
            #[contract_struct_api(option_borrow)] optional: Option<Value>,
            #[contract_struct_api(slice = Value)] values: Values<Value>,
        }
    });
    let expected = quote::quote! {
        impl<Value> Request<Value> where Value: Copy {
            #[must_use]
            pub const fn new(borrowed: Value, copied: Value, copied_ref: Value, consumed: Value, optional: Option<Value>, values: Values<Value>) -> Self {
                Self { borrowed, copied, copied_ref, consumed, optional, values }
            }
            #[must_use]
            pub fn into_parts(self) -> (Value, Value, Value, Value, Option<Value>, Values<Value>,) {
                (self.borrowed, self.copied, self.copied_ref, self.consumed, self.optional, self.values,)
            }
            #[must_use]
            pub const fn borrowed(&self) -> &Value { &self.borrowed }
            #[must_use]
            pub const fn copied(self) -> Value { self.copied }
            #[must_use]
            pub const fn copied_ref(&self) -> Value { self.copied_ref }
            #[must_use]
            pub fn into_consumed(self) -> Value { self.consumed }
            #[must_use]
            pub const fn optional(&self) -> Option<&Value> { self.optional.as_ref() }
            #[must_use]
            pub const fn values(&self) -> &[Value] { self.values.as_slice() }
        }
    };
    assert_eq!(output.to_string(), expected.to_string());
}

#[test]
fn test_typed_route_expansion_requires_attribute() {
    let output = crate::derive_typed_route(quote::quote! { struct Route; });
    assert!(
        output
            .to_string()
            .contains(constants_str::TYPED_ROUTE_DERIVE_REQUIRES_ATTRIBUTE)
    );
}

#[test]
fn test_typed_route_expansion_rejects_unknown_and_nonpath_methods() {
    let methods = [quote::quote!(Unknown), quote::quote!(1)];
    assert!(methods.into_iter().all(|method| {
        crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy,
                method = #method, openapi_operation_id = Operation, path = Path,
                request = Request, response = Response, success_status = Status, transport = Transport)]
            struct Route;
        }).to_string().contains(constants_str::TYPED_ROUTE_METHOD_MUST_BE_STANDARD_HTTP_METHOD)
    }));
}

#[test]
fn test_typed_route_expands_every_standard_method_case_insensitively() {
    let cases = [
        (
            quote::quote!(connect),
            quote::quote!(frontend_contract::route_method::RouteMethod::Connect),
        ),
        (
            quote::quote!(CONNECT),
            quote::quote!(frontend_contract::route_method::RouteMethod::Connect),
        ),
        (
            quote::quote!(delete),
            quote::quote!(frontend_contract::route_method::RouteMethod::Delete),
        ),
        (
            quote::quote!(DELETE),
            quote::quote!(frontend_contract::route_method::RouteMethod::Delete),
        ),
        (
            quote::quote!(get),
            quote::quote!(frontend_contract::route_method::RouteMethod::Get),
        ),
        (
            quote::quote!(GET),
            quote::quote!(frontend_contract::route_method::RouteMethod::Get),
        ),
        (
            quote::quote!(head),
            quote::quote!(frontend_contract::route_method::RouteMethod::Head),
        ),
        (
            quote::quote!(HEAD),
            quote::quote!(frontend_contract::route_method::RouteMethod::Head),
        ),
        (
            quote::quote!(options),
            quote::quote!(frontend_contract::route_method::RouteMethod::Options),
        ),
        (
            quote::quote!(OPTIONS),
            quote::quote!(frontend_contract::route_method::RouteMethod::Options),
        ),
        (
            quote::quote!(patch),
            quote::quote!(frontend_contract::route_method::RouteMethod::Patch),
        ),
        (
            quote::quote!(PATCH),
            quote::quote!(frontend_contract::route_method::RouteMethod::Patch),
        ),
        (
            quote::quote!(post),
            quote::quote!(frontend_contract::route_method::RouteMethod::Post),
        ),
        (
            quote::quote!(POST),
            quote::quote!(frontend_contract::route_method::RouteMethod::Post),
        ),
        (
            quote::quote!(put),
            quote::quote!(frontend_contract::route_method::RouteMethod::Put),
        ),
        (
            quote::quote!(PUT),
            quote::quote!(frontend_contract::route_method::RouteMethod::Put),
        ),
        (
            quote::quote!(trace),
            quote::quote!(frontend_contract::route_method::RouteMethod::Trace),
        ),
        (
            quote::quote!(TRACE),
            quote::quote!(frontend_contract::route_method::RouteMethod::Trace),
        ),
    ];
    assert!(cases.into_iter().all(|(method, expected)| {
        let output = crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy,
                method = #method, openapi_operation_id = Operation, path = Path,
                request = Request, response = Response, success_status = Status, transport = Transport)]
            struct Route;
        });
        output.to_string().contains(expected.to_string().as_str()) && syn::parse2::<syn::File>(output).is_ok()
    }));
}

#[test]
fn test_typed_route_parameterized_path_reports_each_invalid_shape() {
    let missing = syn::LitStr::new(constants_str::X, proc_macro2::Span::call_site());
    let unclosed = syn::LitStr::new(
        format!("{}{{{}", constants_str::X, constants_str::X).as_str(),
        proc_macro2::Span::call_site(),
    );
    let empty = syn::LitStr::new(
        format!("{}{{}}", constants_str::X).as_str(),
        proc_macro2::Span::call_site(),
    );
    let multiple = syn::LitStr::new(
        format!(
            "{}{{{}}}{{{}}}",
            constants_str::X,
            constants_str::X,
            constants_str::X
        )
        .as_str(),
        proc_macro2::Span::call_site(),
    );
    let cases = [
        (
            quote::quote!(Path),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
        ),
        (
            quote::quote!(1),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_MUST_BE_STRING_LITERAL,
        ),
        (
            quote::quote!(#missing),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_PLACEHOLDER,
        ),
        (
            quote::quote!(#unclosed),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_REQUIRES_CLOSED_PLACEHOLDER,
        ),
        (
            quote::quote!(#empty),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_SUPPORTS_ONE_PLACEHOLDER,
        ),
        (
            quote::quote!(#multiple),
            constants_str::TYPED_ROUTE_PARAMETER_PATH_SUPPORTS_ONE_PLACEHOLDER,
        ),
    ];
    assert!(cases.into_iter().all(|(path, expected)| {
        crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy,
                method = Get, openapi_operation_id = Operation, path = #path, path_parameter = Parameter,
                request = Request, response = Response, success_status = Status, transport = Transport)]
            struct Route;
        }).to_string().contains(expected)
    }));
}

#[test]
fn test_typed_parameterized_route_preserves_path_parts_and_client_parameter() {
    let prefix = syn::LitStr::new(constants_str::TEST_FIRST, proc_macro2::Span::call_site());
    let suffix = syn::LitStr::new(constants_str::TEST_LAST, proc_macro2::Span::call_site());
    let parameter_name = syn::LitStr::new(constants_str::ID, proc_macro2::Span::call_site());
    let path = syn::LitStr::new(
        format!(
            "{}{{{}}}{}",
            prefix.value(),
            parameter_name.value(),
            suffix.value()
        )
        .as_str(),
        proc_macro2::Span::call_site(),
    );
    let output = crate::derive_typed_route(quote::quote! {
        #[typed_route(authentication = Authentication, error_policy = Policy,
            method = Get, openapi_operation_id = Operation, path = #path, path_parameter = Parameter,
            request = Request, response = Response, success_status = Status, transport = Transport)]
        pub struct Route;
    });
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(format!("{}{}{}", #prefix, parameter, #suffix))
                .to_string()
                .as_str()
        )
    );
    assert!(text.contains(quote::quote!(.name(#parameter_name)).to_string().as_str()));
    assert!(
        text.contains(
            quote::quote!(client.send_parameterized::<Route>(parameter, request).await)
                .to_string()
                .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| {
        let functions = file.items.iter().filter_map(|item| if let syn::Item::Fn(function) = item { Some(function) } else { None }).collect::<Vec<_>>();
        matches!(functions.as_slice(), [route, client] if route.sig.inputs.len() == 1usize && client.sig.inputs.len() == 3usize && client.sig.asyncness.is_some())
    }));
}

#[test]
fn test_api_operation_error_requires_exactly_one_error_identifier() {
    assert!(
        [
            (
                quote::quote!(),
                constants_str::API_OPERATION_ERROR_REQUIRES_ERROR_TYPE
            ),
            (
                quote::quote!(First, Second),
                constants_str::API_OPERATION_ERROR_ACCEPTS_ONE_ERROR_TYPE
            ),
        ]
        .into_iter()
        .all(|(input, expected)| crate::api_operation_error(input)
            .to_string()
            .contains(expected))
    );
    assert!(
        crate::api_operation_error(quote::quote!(1))
            .to_string()
            .contains(quote::quote!(compile_error).to_string().as_str())
    );
}

#[test]
fn test_api_operation_error_expansion_preserves_error_identity_and_implementations() {
    assert!(syn::parse2::<syn::File>(crate::api_operation_error(quote::quote!(OperationError))).is_ok_and(|file| {
        matches!(file.items.as_slice(), [syn::Item::Enum(error), syn::Item::Impl(conversion), syn::Item::Impl(response)]
            if error.ident == quote::quote!(OperationError).to_string()
                && matches!(error.vis, syn::Visibility::Restricted(_))
                && error.variants.len() == 17usize
                && conversion.self_ty == syn::parse_quote!(OperationError)
                && response.self_ty == syn::parse_quote!(OperationError)
                && conversion.trait_.as_ref().is_some_and(|(path, _)| path.segments.last().is_some_and(|segment| segment.ident == quote::quote!(From).to_string()))
                && response.trait_.as_ref().is_some_and(|(path, _)| path.segments.last().is_some_and(|segment| segment.ident == quote::quote!(IntoResponse).to_string())))
    }));
}

#[test]
fn test_route_error_rejects_missing_async_return_and_receiver() {
    assert!(
        [
            (
                quote::quote!(
                    fn endpoint() -> Response {
                        response
                    }
                ),
                constants_str::ROUTE_ERROR_REQUIRES_ASYNC_FUNCTION
            ),
            (
                quote::quote!(
                    async fn endpoint() {
                        response
                    }
                ),
                constants_str::ROUTE_ERROR_REQUIRES_EXPLICIT_RETURN_TYPE
            ),
            (
                quote::quote!(
                    async fn endpoint(&self) -> Response {
                        response
                    }
                ),
                constants_str::ROUTE_ERROR_REQUIRES_TYPED_PARAMETERS
            ),
        ]
        .into_iter()
        .all(
            |(input, expected)| crate::route_error(quote::quote!(OperationError), input)
                .to_string()
                .contains(expected)
        )
    );
}

#[test]
fn test_route_error_rejects_nonforwardable_parameter_patterns() {
    assert!(
        [
            quote::quote!(mut value: Value),
            quote::quote!(ref value: Value),
            quote::quote!(value @ Some(_): Value),
            quote::quote!((first, second): Pair),
            quote::quote!(State(mut value): State<Value>),
            quote::quote!(State(_): State<Value>),
            quote::quote!(State((first, second)): State<Pair>),
        ]
        .into_iter()
        .all(|parameter| crate::route_error(
            quote::quote!(OperationError),
            quote::quote!(async fn endpoint(#parameter) -> Response { response })
        )
        .to_string()
        .contains(constants_str::ROUTE_ERROR_UNSUPPORTED_PARAMETER_PATTERN))
    );
}

#[test]
fn test_route_error_reports_invalid_attribute_and_function_tokens() {
    assert!(
        [
            (
                quote::quote!(1),
                quote::quote!(
                    async fn endpoint() -> Response {
                        response
                    }
                )
            ),
            (
                quote::quote!(OperationError),
                quote::quote!(
                    struct Request;
                )
            ),
        ]
        .into_iter()
        .all(|(attribute, input)| crate::route_error(attribute, input)
            .to_string()
            .contains(quote::quote!(compile_error).to_string().as_str()))
    );
}

#[test]
fn test_route_error_forwards_identifier_and_tuple_struct_parameters() {
    let output = crate::route_error(
        quote::quote!(OperationError),
        quote::quote! {
            pub async fn endpoint(State(state): State<Value>, request: Request,) -> Response { response }
        },
    );
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(Ok(endpoint_route_impl(State(state), request).await))
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(
                async fn endpoint_route_impl(
                    State(state): State<Value>,
                    request: Request,
                ) -> Response {
                    response
                }
            )
            .to_string()
            .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| {
        matches!(file.items.as_slice(), [syn::Item::Enum(error), syn::Item::Impl(_), syn::Item::Fn(function)]
            if error.ident == quote::quote!(OperationError).to_string()
                && matches!(error.vis, syn::Visibility::Public(_))
                && function.sig.output == syn::parse_quote!(-> Result<Response, OperationError>))
    }));
}

#[test]
fn test_typed_route_missing_required_fields_preserve_exact_diagnostics() {
    let fields = [
        (
            quote::quote!(authentication),
            quote::quote!(Authentication),
            constants_str::TYPED_ROUTE_REQUIRES_AUTHENTICATION,
        ),
        (
            quote::quote!(method),
            quote::quote!(Method),
            constants_str::TYPED_ROUTE_REQUIRES_METHOD,
        ),
        (
            quote::quote!(openapi_operation_id),
            quote::quote!(Operation),
            constants_str::TYPED_ROUTE_REQUIRES_OPERATION_ID,
        ),
        (
            quote::quote!(path),
            quote::quote!(Path),
            constants_str::TYPED_ROUTE_REQUIRES_PATH,
        ),
        (
            quote::quote!(request),
            quote::quote!(Request),
            constants_str::TYPED_ROUTE_REQUIRES_REQUEST,
        ),
        (
            quote::quote!(response),
            quote::quote!(Response),
            constants_str::TYPED_ROUTE_REQUIRES_RESPONSE,
        ),
        (
            quote::quote!(success_status),
            quote::quote!(Status),
            constants_str::TYPED_ROUTE_REQUIRES_SUCCESS_STATUS,
        ),
        (
            quote::quote!(transport),
            quote::quote!(Transport),
            constants_str::TYPED_ROUTE_REQUIRES_TRANSPORT,
        ),
    ];
    assert!(
        fields
            .iter()
            .enumerate()
            .all(|(missing_index, (_, _, expected))| {
                let mut tokens = quote::quote!(error_policy = Policy,);
                tokens.extend(
                    fields
                        .iter()
                        .enumerate()
                        .filter(|(index, _field)| *index != missing_index)
                        .map(|(_index, (name, value, _diagnostic))| quote::quote!(#name = #value,)),
                );
                syn::parse2::<crate::typed_route_args::TypedRouteArgs>(tokens).is_err_and(|error| {
                    error
                        .to_string()
                        .rsplit_once(',')
                        .is_some_and(|(_context, diagnostic)| diagnostic.trim() == *expected)
                })
            })
    );
}

#[test]
fn test_typed_route_invalid_field_values_preserve_syn_diagnostics() {
    let expected_expression = syn::parse2::<syn::Expr>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let expected_type = syn::parse2::<syn::Type>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    assert!(expected_expression.is_some());
    assert!(expected_type.is_some());
    assert!(
        [
            (quote::quote!(authentication), false),
            (quote::quote!(error_statuses), false),
            (quote::quote!(error_policy), false),
            (quote::quote!(method), false),
            (quote::quote!(openapi_operation_id), false),
            (quote::quote!(mutation), false),
            (quote::quote!(obligations), false),
            (quote::quote!(path), false),
            (quote::quote!(request_body), false),
            (quote::quote!(success_status), false),
            (quote::quote!(error_response), true),
            (quote::quote!(path_parameter), true),
            (quote::quote!(request), true),
            (quote::quote!(response), true),
            (quote::quote!(transport), true),
        ]
        .into_iter()
        .all(|(field, is_type)| {
            let expected = if is_type {
                &expected_type
            } else {
                &expected_expression
            };
            syn::parse2::<crate::typed_route_args::TypedRouteArgs>(quote::quote!(#field = ,))
                .is_err_and(|error| {
                    expected
                        .as_deref()
                        .is_some_and(|diagnostic| error.to_string() == diagnostic)
                })
        })
    );
}

#[test]
fn test_typed_route_unknown_fields_and_missing_syntax_retain_diagnostics() {
    assert!(
        syn::parse2::<crate::typed_route_args::TypedRouteArgs>(quote::quote!(unknown = Value))
            .is_err_and(|error| error.to_string() == constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD)
    );
    let cases = [
        (
            quote::quote!(0u8 = Value),
            syn::parse2::<syn::Ident>(quote::quote!(0u8))
                .err()
                .map(|error| error.to_string()),
        ),
        (
            quote::quote!(authentication Authentication),
            syn::parse2::<syn::Token![=]>(quote::quote!(Authentication))
                .err()
                .map(|error| error.to_string()),
        ),
        (
            quote::quote!(authentication = Authentication method = Method),
            syn::parse2::<syn::Token![,]>(quote::quote!(method))
                .err()
                .map(|error| error.to_string()),
        ),
    ];
    assert!(cases.into_iter().all(|(tokens, expected)| {
        assert!(expected.is_some());
        syn::parse2::<crate::typed_route_args::TypedRouteArgs>(tokens).is_err_and(|error| {
            expected
                .as_deref()
                .is_some_and(|diagnostic| error.to_string() == diagnostic)
        })
    }));
}

#[test]
fn test_catalog_parsers_missing_required_fields_preserve_domain_diagnostics() {
    assert!(
        [
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote!(
                    family = Family
                ))
                .map(|_args| ()),
                constants_str::ROUTE_CATALOG_REQUIRES_BODY_LIMIT
            ),
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote!(
                    body_limit = Limit
                ))
                .map(|_args| ()),
                constants_str::ROUTE_CATALOG_REQUIRES_FAMILY
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(contract = Contract)
                )
                .map(|_args| ()),
                constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(path = Path)
                )
                .map(|_args| ()),
                constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(
                    path_ref = PathRef,
                    spec = Spec
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(
                    inventory = Inventory,
                    spec = Spec
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(
                    inventory = Inventory,
                    path_ref = PathRef
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    metadata = Metadata,
                    path = Path,
                    route = Route,
                    title = Title
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    capability = Capability,
                    path = Path,
                    route = Route,
                    title = Title
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    capability = Capability,
                    metadata = Metadata,
                    route = Route,
                    title = Title
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    capability = Capability,
                    metadata = Metadata,
                    path = Path,
                    title = Title
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    capability = Capability,
                    metadata = Metadata,
                    path = Path,
                    route = Route
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
        ]
        .into_iter()
        .all(|(result, expected)| result.is_err_and(|error| error
            .to_string()
            .split_once(',')
            .is_some_and(|(_context, diagnostic)| diagnostic.trim() == expected)))
    );
}

#[test]
fn test_catalog_parsers_unknown_fields_preserve_owned_diagnostics() {
    assert!(
        [
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote!(
                    unknown = Value
                ))
                .map(|_args| ()),
                constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(unknown = Value)
                )
                .map(|_args| ()),
                constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(
                    unknown = Value
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(
                    unknown = Value
                ))
                .map(|_args| ()),
                constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS
            ),
        ]
        .into_iter()
        .all(|(result, expected)| result.is_err_and(|error| error.to_string() == expected))
    );
}

#[test]
fn test_catalog_parsers_syntax_failures_preserve_syn_diagnostics() {
    let expected_identifier = syn::parse2::<syn::Ident>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let expected_equals = syn::parse2::<syn::Token![=]>(quote::quote!(Value))
        .err()
        .map(|error| error.to_string());
    let expected_comma = syn::parse2::<syn::Token![,]>(quote::quote!(field))
        .err()
        .map(|error| error.to_string());
    let expected_expression = syn::parse2::<syn::Expr>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let expected_type = syn::parse2::<syn::Type>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    assert!(
        [
            &expected_identifier,
            &expected_equals,
            &expected_comma,
            &expected_expression,
            &expected_type
        ]
        .into_iter()
        .all(Option::is_some)
    );
    assert!(
        [
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote!(,))
                    .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(
                    quote::quote!(family Family)
                )
                .map(|_args| ()),
                &expected_equals
            ),
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(
                    quote::quote!(family = Family body_limit = Limit)
                )
                .map(|_args| ()),
                &expected_comma
            ),
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(
                    quote::quote!(family = ,)
                )
                .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(
                    quote::quote!(body_limit = ,)
                )
                .map(|_args| ()),
                &expected_expression
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(,)
                )
                .map(|_args| ()),
                &expected_type
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(contract = Contract, ,)
                )
                .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(contract = Contract, path Path)
                )
                .map(|_args| ()),
                &expected_equals
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(contract = Contract path = Path)
                )
                .map(|_args| ()),
                &expected_comma
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(contract = ,)
                )
                .map(|_args| ()),
                &expected_expression
            ),
            (
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    quote::quote!(path = ,)
                )
                .map(|_args| ()),
                &expected_expression
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(,))
                    .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(
                    quote::quote!(inventory Inventory)
                )
                .map(|_args| ()),
                &expected_equals
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(
                    quote::quote!(inventory = Inventory path_ref = PathRef)
                )
                .map(|_args| ()),
                &expected_comma
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(
                    quote::quote!(inventory = ,)
                )
                .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(
                    quote::quote!(path_ref = ,)
                )
                .map(|_args| ()),
                &expected_type
            ),
            (
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote!(spec = ,))
                    .map(|_args| ()),
                &expected_type
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote!(,))
                    .map(|_args| ()),
                &expected_identifier
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(
                    quote::quote!(capability Capability)
                )
                .map(|_args| ()),
                &expected_equals
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(
                    quote::quote!(capability = Capability metadata = Metadata)
                )
                .map(|_args| ()),
                &expected_comma
            ),
            (
                syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(
                    quote::quote!(capability = ,)
                )
                .map(|_args| ()),
                &expected_expression
            ),
        ]
        .into_iter()
        .all(|(result, expected)| result.is_err_and(|error| expected
            .as_deref()
            .is_some_and(|diagnostic| error.to_string() == diagnostic)))
    );
}

#[test]
fn test_route_registry_required_labels_and_bindings_preserve_domain_diagnostics() {
    assert!(
        [
            (
                quote::quote!(unknown = State),
                constants_str::ROUTE_REGISTRY_REQUIRES_STATE
            ),
            (
                quote::quote!(state = State, unknown = Family),
                constants_str::ROUTE_REGISTRY_REQUIRES_FAMILY
            ),
            (
                quote::quote!(state = State, family = Family; (Auth, Csrf); unknown()),
                constants_str::ROUTE_REGISTRY_REQUIRES_SCHEMAS
            ),
            (
                quote::quote!(state = State, family = Family; (Auth, Csrf); schemas();),
                constants_str::ROUTE_REGISTRY_REQUIRES_BINDING
            ),
        ]
        .into_iter()
        .all(|(tokens, expected)| syn::parse2::<
            crate::route_registry_args::RouteRegistryArgs,
        >(tokens)
        .is_err_and(|error| {
            let diagnostic = error.to_string();
            diagnostic == expected
                || diagnostic
                    .split_once(',')
                    .is_some_and(|(_context, message)| message.trim() == expected)
        }))
    );
}

#[test]
fn test_route_registry_syntax_failures_preserve_syn_diagnostics() {
    let identifier = syn::parse2::<syn::Ident>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let equals = syn::parse2::<syn::Token![=]>(quote::quote!(Value))
        .err()
        .map(|error| error.to_string());
    let comma = syn::parse2::<syn::Token![,]>(quote::quote!(field))
        .err()
        .map(|error| error.to_string());
    let semicolon = syn::parse2::<syn::Token![;]>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let expression = syn::parse2::<syn::Expr>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let schema_type = syn::parse2::<syn::Type>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    let endpoint = syn::parse2::<syn::Path>(quote::quote!(,))
        .err()
        .map(|error| error.to_string());
    assert!(
        [
            &identifier,
            &equals,
            &comma,
            &semicolon,
            &expression,
            &schema_type,
            &endpoint
        ]
        .into_iter()
        .all(Option::is_some)
    );
    assert!([
        (quote::quote!(,), &identifier),
        (quote::quote!(state State), &equals),
        (quote::quote!(state = ,), &schema_type),
        (quote::quote!(state = State family = Family), &comma),
        (quote::quote!(state = State, ,), &identifier),
        (quote::quote!(state = State, family Family), &equals),
        (quote::quote!(state = State, family = ,), &schema_type),
        (quote::quote!(state = State, family = Family,), &semicolon),
        (quote::quote!(state = State, family = Family; (, Csrf)), &expression),
        (quote::quote!(state = State, family = Family; (Auth Csrf)), &comma),
        (quote::quote!(state = State, family = Family; (Auth, ,)), &expression),
        (quote::quote!(state = State, family = Family; (Auth, Csrf),), &semicolon),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); ,), &identifier),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); schemas(,)), &schema_type),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); schemas(),), &semicolon),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); schemas(); (, Endpoint)), &schema_type),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); schemas(); (Route Endpoint)), &comma),
        (quote::quote!(state = State, family = Family; (Auth, Csrf); schemas(); (Route, ,)), &endpoint),
    ].into_iter().all(|(tokens, expected)| syn::parse2::<crate::route_registry_args::RouteRegistryArgs>(tokens).is_err_and(|error| expected.as_deref().is_some_and(|diagnostic| error.to_string() == diagnostic))));
}

#[test]
fn test_endpoint_registry_parser_preserves_complex_state_and_ordered_bindings() {
    let parsed =
        syn::parse2::<crate::endpoint_registry_args::EndpointRegistryArgs>(quote::quote! {
            state = crate::State<Inner>;
            (contract::first(Inner::VALUE), handlers::first::<Inner>),
            (contract::second(), handlers::second),
        });
    assert!(parsed.is_ok_and(|args| {
        quote::ToTokens::to_token_stream(args.get_state().as_ref()).to_string()
            == quote::quote!(crate::State<Inner>).to_string()
            && args.get_bindings().as_ref().len() == 2usize
            && args
                .get_bindings()
                .as_ref()
                .iter()
                .zip([
                    (
                        quote::quote!(contract::first(Inner::VALUE)),
                        quote::quote!(handlers::first::<Inner>),
                    ),
                    (
                        quote::quote!(contract::second()),
                        quote::quote!(handlers::second),
                    ),
                ])
                .all(|(binding, (contract, endpoint))| {
                    quote::ToTokens::to_token_stream(binding.get_contract().as_ref()).to_string()
                        == contract.to_string()
                        && quote::ToTokens::to_token_stream(binding.get_endpoint().as_ref())
                            .to_string()
                            == endpoint.to_string()
                })
    }));
}

#[test]
fn test_route_registry_parser_preserves_security_schemas_and_complex_bindings() {
    let parsed = syn::parse2::<crate::route_registry_args::RouteRegistryArgs>(quote::quote! {
        state = crate::State<Inner>, family = crate::Family<Inner>;
        (security::authenticated(Inner::VALUE), security::csrf());
        schemas(Option<Inner>, [Inner; 2]);
        (crate::First<Inner>, handlers::first::<Inner>),
        (crate::Second, handlers::second),
    });
    assert!(parsed.is_ok_and(|args| {
        quote::ToTokens::to_token_stream(args.get_state().as_ref()).to_string()
            == quote::quote!(crate::State<Inner>).to_string()
            && quote::ToTokens::to_token_stream(args.get_family().as_ref()).to_string()
                == quote::quote!(crate::Family<Inner>).to_string()
            && quote::ToTokens::to_token_stream(args.get_authenticated_security().as_ref())
                .to_string()
                == quote::quote!(security::authenticated(Inner::VALUE)).to_string()
            && quote::ToTokens::to_token_stream(args.get_csrf_security().as_ref()).to_string()
                == quote::quote!(security::csrf()).to_string()
            && args.get_schemas().as_ref().len() == 2usize
            && args
                .get_schemas()
                .as_ref()
                .iter()
                .zip([quote::quote!(Option<Inner>), quote::quote!([Inner; 2])])
                .all(|(schema, expected)| {
                    quote::ToTokens::to_token_stream(schema).to_string() == expected.to_string()
                })
            && args.get_bindings().as_ref().len() == 2usize
            && args
                .get_bindings()
                .as_ref()
                .iter()
                .zip([
                    (
                        quote::quote!(crate::First<Inner>),
                        quote::quote!(handlers::first::<Inner>),
                    ),
                    (
                        quote::quote!(crate::Second),
                        quote::quote!(handlers::second),
                    ),
                ])
                .all(|(binding, (route, endpoint))| {
                    quote::ToTokens::to_token_stream(binding.get_route().as_ref()).to_string()
                        == route.to_string()
                        && quote::ToTokens::to_token_stream(binding.get_endpoint().as_ref())
                            .to_string()
                            == endpoint.to_string()
                })
    }));
}

#[test]
fn test_typed_route_parser_preserves_required_optional_and_distinct_error_fields() {
    assert!(
        [
            (
                quote::quote!(error_policy = policy::VALUE,),
                true,
                quote::quote!(policy::VALUE)
            ),
            (
                quote::quote!(error_statuses = [statuses::BAD_REQUEST, statuses::CONFLICT],),
                false,
                quote::quote!([statuses::BAD_REQUEST, statuses::CONFLICT])
            ),
        ]
        .into_iter()
        .all(|(errors, policy, expected_errors)| {
            [false, true].into_iter().all(|include_optional| {
                let optional_fields = if include_optional {
                    quote::quote! {
                            error_response = crate::ErrorResponse<Inner>,
                            mutation = mutation::VALUE,
                            obligations = obligations::required(),
                            path_parameter = crate::Parameter<Inner>,
                            request_body = body::schema::<Inner>()
                        ,
                    }
                } else {
                    proc_macro2::TokenStream::new()
                };
                let parsed =
                    syn::parse2::<crate::typed_route_args::TypedRouteArgs>(quote::quote! {
                        #optional_fields #errors
                            authentication = authentication::required(),
                            method = methods::GET,
                            openapi_operation_id = stringify!(endpoint),
                            path = paths::VALUE,
                            request = crate::Request<Inner>,
                            response = crate::Response<Inner>,
                            success_status = 200 + 1,
                            transport = crate::Transport<Inner>
                        ,
                    });
                parsed.is_ok_and(|args| {
                    let errors_preserved = match args.get_errors() {
                        crate::syn_typed_route_errors::SynTypedRouteErrors::Policy(value) => {
                            policy
                                && quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                    == expected_errors.to_string()
                        }
                        crate::syn_typed_route_errors::SynTypedRouteErrors::Statuses(value) => {
                            !policy
                                && quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                    == expected_errors.to_string()
                        }
                    };
                    errors_preserved
                        && [
                            (
                                quote::ToTokens::to_token_stream(
                                    args.get_authentication().as_ref(),
                                )
                                .to_string(),
                                quote::quote!(authentication::required()).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(args.get_method().as_ref())
                                    .to_string(),
                                quote::quote!(methods::GET).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(
                                    args.get_openapi_operation_id().as_ref(),
                                )
                                .to_string(),
                                quote::quote!(stringify!(endpoint)).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(args.get_path().as_ref())
                                    .to_string(),
                                quote::quote!(paths::VALUE).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(args.get_request().as_ref())
                                    .to_string(),
                                quote::quote!(crate::Request<Inner>).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(args.get_response().as_ref())
                                    .to_string(),
                                quote::quote!(crate::Response<Inner>).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(
                                    args.get_success_status().as_ref(),
                                )
                                .to_string(),
                                quote::quote!(200 + 1).to_string(),
                            ),
                            (
                                quote::ToTokens::to_token_stream(args.get_transport().as_ref())
                                    .to_string(),
                                quote::quote!(crate::Transport<Inner>).to_string(),
                            ),
                        ]
                        .into_iter()
                        .all(|(actual, expected)| actual == expected)
                        && [
                            (
                                args.get_error_response().map(|value| {
                                    quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                }),
                                quote::quote!(crate::ErrorResponse<Inner>).to_string(),
                            ),
                            (
                                args.get_mutation().map(|value| {
                                    quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                }),
                                quote::quote!(mutation::VALUE).to_string(),
                            ),
                            (
                                args.get_obligations().map(|value| {
                                    quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                }),
                                quote::quote!(obligations::required()).to_string(),
                            ),
                            (
                                args.get_path_parameter().map(|value| {
                                    quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                }),
                                quote::quote!(crate::Parameter<Inner>).to_string(),
                            ),
                            (
                                args.get_request_body().map(|value| {
                                    quote::ToTokens::to_token_stream(value.as_ref()).to_string()
                                }),
                                quote::quote!(body::schema::<Inner>()).to_string(),
                            ),
                        ]
                        .into_iter()
                        .all(|(actual, expected)| actual == include_optional.then_some(expected))
                })
            })
        })
    );
}

#[test]
fn test_page_catalog_parsers_preserve_reordered_types_and_page_expressions() {
    let catalog = syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(quote::quote! {
        spec = crate::Spec<Inner>, path_ref = &'static crate::Path, inventory = Inventory,
    });
    assert!(catalog.is_ok_and(|args| {
        [
            (
                quote::ToTokens::to_token_stream(args.get_spec().as_ref()).to_string(),
                quote::quote!(crate::Spec<Inner>).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_path_ref().as_ref()).to_string(),
                quote::quote!(&'static crate::Path).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_inventory().as_ref()).to_string(),
                quote::quote!(Inventory).to_string(),
            ),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == expected)
    }));
    let page = syn::parse2::<crate::page_catalog_page_args::PageCatalogPageArgs>(quote::quote! {
        title = stringify!(Page), route = routes::VALUE, path = paths::build(),
        metadata = metadata::build::<Inner>(), capability = capability::VALUE,
    });
    assert!(page.is_ok_and(|args| {
        [
            (
                quote::ToTokens::to_token_stream(args.get_title().as_ref()).to_string(),
                quote::quote!(stringify!(Page)).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_route().as_ref()).to_string(),
                quote::quote!(routes::VALUE).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_path().as_ref()).to_string(),
                quote::quote!(paths::build()).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_metadata().as_ref()).to_string(),
                quote::quote!(metadata::build::<Inner>()).to_string(),
            ),
            (
                quote::ToTokens::to_token_stream(args.get_capability().as_ref()).to_string(),
                quote::quote!(capability::VALUE).to_string(),
            ),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == expected)
    }));
}

#[test]
fn test_route_catalog_parsers_preserve_family_limit_and_both_route_forms() {
    let catalog = syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(quote::quote! {
        body_limit = 128 * 2, family = Family,
    });
    assert!(catalog.is_ok_and(|args| {
        quote::ToTokens::to_token_stream(args.get_family().as_ref()).to_string()
            == quote::quote!(Family).to_string()
            && quote::ToTokens::to_token_stream(args.get_body_limit().as_ref()).to_string()
                == quote::quote!(128 * 2).to_string()
    }));
    let typed = syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
        quote::quote!(crate::Route<Inner>),
    );
    assert!(typed.is_ok_and(|args| {
        args.get_contract().is_none()
            && args.get_path().is_none()
            && !args.get_exclude_from_family().get()
            && args.get_route().is_some_and(|route| {
                quote::ToTokens::to_token_stream(route.as_ref()).to_string()
                    == quote::quote!(crate::Route<Inner>).to_string()
            })
    }));
    assert!(
        [
            (
                quote::quote!(contract = contract::build(), path = paths::VALUE),
                false
            ),
            (
                quote::quote!(
                    path = paths::VALUE,
                    exclude_from_family,
                    contract = contract::build(),
                ),
                true
            ),
        ]
        .into_iter()
        .all(|(tokens, excluded)| {
            syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(tokens).is_ok_and(
                |args| {
                    args.get_route().is_none()
                        && args.get_exclude_from_family().get() == excluded
                        && args.get_contract().is_some_and(|contract| {
                            quote::ToTokens::to_token_stream(contract.as_ref()).to_string()
                                == quote::quote!(contract::build()).to_string()
                        })
                        && args.get_path().is_some_and(|path| {
                            quote::ToTokens::to_token_stream(path.as_ref()).to_string()
                                == quote::quote!(paths::VALUE).to_string()
                        })
                },
            )
        })
    );
}
