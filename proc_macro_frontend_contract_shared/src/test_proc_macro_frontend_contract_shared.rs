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
