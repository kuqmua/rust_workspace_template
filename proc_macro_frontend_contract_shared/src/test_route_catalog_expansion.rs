#[test]
fn test_route_operation_preserves_function_tokens() {
    let input = quote::quote! { pub async fn endpoint(request: Request) -> Response { response } };
    assert_eq!(
        crate::route_operation(quote::quote!(), input.clone()).to_string(),
        input.to_string()
    );
}

#[test]
fn test_route_operation_rejects_arguments_and_nonfunctions() {
    assert!(
        crate::route_operation(
            quote::quote!(Value),
            quote::quote!(
                fn endpoint() {
                    response
                }
            )
        )
        .to_string()
        .contains(constants_str::ROUTE_OPERATION_ACCEPTS_NO_ARGUMENTS)
    );
    assert!(
        crate::route_operation(
            quote::quote!(),
            quote::quote!(
                struct Request;
            )
        )
        .to_string()
        .contains(quote::quote!(compile_error).to_string().as_str())
    );
}

#[test]
fn test_route_catalog_requires_one_catalog_attribute_and_enum() {
    assert!(
        [
            (
                quote::quote!(
                    enum Catalog {
                        First,
                    }
                ),
                constants_str::ROUTE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                quote::quote!(
                    #[route_catalog(body_limit = Limit, family = Family)]
                    struct Catalog;
                ),
                constants_str::ROUTE_CATALOG_REQUIRES_ATTRIBUTE
            ),
            (
                quote::quote!(
                    #[route_catalog(body_limit = Limit, family = Family)]
                    #[route_catalog(body_limit = Limit, family = Family)]
                    enum Catalog {
                        First,
                    }
                ),
                constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD
            ),
        ]
        .into_iter()
        .all(|(input, expected)| crate::derive_route_catalog(input)
            .to_string()
            .contains(expected))
    );
}

#[test]
fn test_route_catalog_requires_one_route_attribute_per_variant() {
    assert!(
        [
            (
                quote::quote!(First),
                constants_str::ROUTE_CATALOG_VARIANT_REQUIRES_ROUTE
            ),
            (
                quote::quote!(
                    #[route_catalog_route(Route)]
                    #[route_catalog_route(Route)]
                    First
                ),
                constants_str::DUPLICATE_FRONTEND_CONTRACT_FIELD
            ),
        ]
        .into_iter()
        .all(
            |(variant, expected)| crate::derive_route_catalog(quote::quote! {
                #[route_catalog(body_limit = Limit, family = Family)] enum Catalog { #variant }
            })
            .to_string()
            .contains(expected)
        )
    );
}

#[test]
fn test_route_catalog_rejects_unsupported_typed_variant_fields() {
    assert!([quote::quote!(First(Value, Value)), quote::quote!(First { value: Value })].into_iter().all(|variant| crate::derive_route_catalog(quote::quote! {
        #[route_catalog(body_limit = Limit, family = Family)] enum Catalog { #[route_catalog_route(Route)] #variant }
    }).to_string().contains(constants_str::ROUTE_CATALOG_ROUTE_SUPPORTS_UNIT_OR_SINGLE_FIELD_VARIANTS)));
}

#[test]
fn test_route_catalog_custom_routes_require_unit_and_family_exclusion() {
    assert!([
        (quote::quote!(First(Value)), constants_str::ROUTE_CATALOG_CUSTOM_ROUTE_MUST_BE_UNIT),
        (quote::quote!(First { value: Value }), constants_str::ROUTE_CATALOG_CUSTOM_ROUTE_MUST_BE_UNIT),
        (quote::quote!(First), constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES),
    ].into_iter().all(|(variant, expected)| crate::derive_route_catalog(quote::quote! {
        #[route_catalog(body_limit = Limit, family = Family)] enum Catalog { #[route_catalog_route(contract = Contract, path = Path)] #variant }
    }).to_string().contains(expected)));
}

#[test]
fn test_route_catalog_unit_variants_emit_ordered_all_and_family_membership() {
    let output = crate::derive_route_catalog(quote::quote! {
        #[route_catalog(body_limit = Limit, family = Family)]
        pub enum Catalog {
            #[route_catalog_route(FirstRoute)] First,
            #[route_catalog_route(SecondRoute)] Second,
        }
    });
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(
                pub const ALL: [Self; 2usize] = [Self::First, Self::Second];
            )
            .to_string()
            .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(
                const ROUTE_COUNT: usize = 2usize;
            )
            .to_string()
            .as_str()
        )
    );
    assert!([quote::quote!(FirstRoute), quote::quote!(SecondRoute)].into_iter().all(|route| text.contains(quote::quote!(impl frontend_contract::route_in_family::RouteInFamily<Family> for #route {}).to_string().as_str())));
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 5usize));
}

#[test]
fn test_route_catalog_parameterized_variant_forwards_value_and_omits_all() {
    let output = crate::derive_route_catalog(quote::quote! {
        #[route_catalog(body_limit = Limit, family = Family)]
        enum Catalog { #[route_catalog_route(Route)] First(Parameter) }
    });
    let text = output.to_string();
    assert!(text.contains(quote::quote!(Self::First(value) => frontend_contract::typed_parameterized_route_path::typed_parameterized_route_path::<Route>(&value)).to_string().as_str()));
    assert!(!text.contains(quote::quote!(const ALL).to_string().as_str()));
    assert!(
        text.contains(
            quote::quote!(
                const ROUTE_COUNT: usize = 1usize;
            )
            .to_string()
            .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 4usize));
}

#[test]
fn test_route_catalog_custom_route_generates_snake_case_functions_without_family_member() {
    let output = crate::derive_route_catalog(quote::quote! {
        #[route_catalog(body_limit = Limit, family = Family)]
        pub enum Catalog {
            #[route_catalog_route(contract = Contract, path = Path, exclude_from_family)] CustomRoute,
        }
    });
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(pub fn custom_route_route())
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(pub async fn custom_route_client<Transport>)
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(client.send_contract(Contract, custom_route_route()).await)
                .to_string()
                .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(
                const ROUTE_COUNT: usize = 0usize;
            )
            .to_string()
            .as_str()
        )
    );
    assert!(
        text.contains(
            quote::quote!(
                pub const ALL: [Self; 1usize] = [Self::CustomRoute];
            )
            .to_string()
            .as_str()
        )
    );
    assert!(!text.contains(quote::quote!(RouteInFamily).to_string().as_str()));
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 5usize));
}

#[test]
fn test_unit_enum_generators_reject_non_enum_and_non_unit_inputs() {
    assert!(
        [
            (
                quote::quote!(
                    struct Catalog;
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_ENUMS
            ),
            (
                quote::quote!(
                    enum Catalog {
                        First(Value),
                    }
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS
            ),
            (
                quote::quote!(
                    enum Catalog {
                        First { value: Value },
                    }
                ),
                constants_str::ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS
            ),
        ]
        .into_iter()
        .all(|(input, expected)| [
            crate::derive_unit_enum_catalog(input.clone()),
            crate::derive_unit_enum_index(input)
        ]
        .into_iter()
        .all(|output| output.to_string().contains(expected)))
    );
}

#[test]
fn test_unit_enum_catalog_preserves_variant_order_and_discriminants() {
    let input = quote::quote!(
        enum Catalog {
            Second = 9,
            First = 3,
        }
    );
    assert_eq!(crate::derive_unit_enum_catalog(input.clone()).to_string(), quote::quote!(impl Catalog { pub const ALL: [Self; 2usize] = [Self::Second, Self::First]; }).to_string());
    assert_eq!(crate::derive_unit_enum_index(input).to_string(), quote::quote! {
        impl Catalog {
            pub const COUNT: usize = 2usize;
            #[must_use]
            pub const fn index(self) -> usize { match self { Self::Second => 0usize, Self::First => 1usize } }
        }
    }.to_string());
}

#[test]
fn test_unit_enum_generators_report_malformed_tokens() {
    assert!(
        [
            crate::derive_unit_enum_catalog(quote::quote!(1)),
            crate::derive_unit_enum_index(quote::quote!(1))
        ]
        .into_iter()
        .all(|output| output
            .to_string()
            .contains(quote::quote!(compile_error).to_string().as_str()))
    );
}

#[test]
fn test_page_catalog_rejects_missing_attribute_and_unsupported_shapes() {
    assert!(
        crate::derive_page_catalog(quote::quote!(
            enum Catalog {
                First,
            }
        ))
        .to_string()
        .contains(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
    );
    assert!(
        [
            quote::quote!(
                struct Catalog;
            ),
            quote::quote!(
                enum Catalog {
                    First(Value),
                }
            ),
            quote::quote!(
                enum Catalog {
                    First { value: Value },
                }
            )
        ]
        .into_iter()
        .all(|item| crate::derive_page_catalog(quote::quote! {
            #[page_catalog(spec = Spec, path_ref = PathRef, inventory = Inventory)] #item
        })
        .to_string()
        .contains(constants_str::PAGE_CATALOG_SUPPORTS_UNIT_VARIANTS))
    );
}

#[test]
fn test_page_catalog_requires_page_attributes_and_complete_fields() {
    assert!([
        (quote::quote!(First), constants_str::PAGE_CATALOG_VARIANT_REQUIRES_PAGE),
        (quote::quote!(#[page_catalog_page(capability = Capability)] First), constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS),
    ].into_iter().all(|(variant, expected)| crate::derive_page_catalog(quote::quote! {
        #[page_catalog(spec = Spec, path_ref = PathRef, inventory = Inventory)] enum Catalog { #variant }
    }).to_string().contains(expected)));
}

#[test]
fn test_page_catalog_inventory_preserves_fields_order_and_page_indexes() {
    let output = crate::derive_page_catalog(quote::quote! {
        #[page_catalog(spec = Spec, path_ref = PathRef, inventory = Inventory)]
        enum Catalog {
            #[page_catalog_page(capability = FirstCapability, metadata = FirstMetadata, path = FirstPath, route = FirstRoute, title = FirstTitle)] Second,
            #[page_catalog_page(capability = SecondCapability, metadata = SecondMetadata, path = SecondPath, route = SecondRoute, title = SecondTitle)] First,
        }
    });
    let text = output.to_string();
    assert!(text.contains(quote::quote! {
        const Inventory: [Spec; 2usize] = [
            Spec::new(FirstCapability, FirstMetadata, Catalog::Second, FirstPath, FirstRoute, FirstTitle,),
            Spec::new(SecondCapability, SecondMetadata, Catalog::First, SecondPath, SecondRoute, SecondTitle,)
        ];
    }.to_string().as_str()));
    assert!(text.contains(quote::quote! { match self { Self::Second => Inventory[0], Self::First => Inventory[1] } }.to_string().as_str()));
    assert!(
        text.contains(
            quote::quote!(.find(|spec| spec.path().as_ref() == path.get()))
                .to_string()
                .as_str()
        )
    );
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 2usize));
}

#[test]
fn test_route_family_requires_attribute_and_nonempty_routes() {
    assert!(
        crate::derive_route_family(quote::quote!(
            struct Family;
        ))
        .to_string()
        .contains(constants_str::ROUTE_FAMILY_DERIVE_REQUIRES_ATTRIBUTE)
    );
    assert!(
        crate::derive_route_family(quote::quote!(
            #[route_family()]
            struct Family;
        ))
        .to_string()
        .contains(constants_str::ROUTE_FAMILY_REQUIRES_ROUTE)
    );
}

#[test]
fn test_route_family_generates_body_limit_only_when_supplied() {
    assert!([false, true].into_iter().all(|has_limit| {
        let attribute = if has_limit {
            quote::quote!(#[route_family_body_limit(Limit)])
        } else {
            quote::quote!()
        };
        let output = crate::derive_route_family(
            quote::quote! { #[route_family(FirstRoute, SecondRoute)] #attribute struct Family; },
        );
        let text = output.to_string();
        text.contains(
            quote::quote!(
                const ROUTE_COUNT: usize = 2usize;
            )
            .to_string()
            .as_str(),
        ) && text.contains(quote::quote!(fn body_limit()).to_string().as_str()) == has_limit
            && (!has_limit
                || text.contains(
                    quote::quote!(Some(
                        frontend_contract::route_body_limit::RouteBodyLimit::from(Limit)
                    ))
                    .to_string()
                    .as_str(),
                ))
            && syn::parse2::<syn::File>(output).is_ok_and(|file| file.items.len() == 3usize)
    }));
}

#[test]
fn test_route_family_rejects_invalid_route_and_body_limit_tokens() {
    assert!(
        [
            quote::quote!(
                #[route_family(1)]
                struct Family;
            ),
            quote::quote!(
                #[route_family(Route)]
                #[route_family_body_limit()]
                struct Family;
            ),
        ]
        .into_iter()
        .all(|input| crate::derive_route_family(input)
            .to_string()
            .contains(quote::quote!(compile_error).to_string().as_str()))
    );
}

#[test]
fn test_route_registry_requires_list_form_openapi_metadata() {
    assert!([quote::quote!(), quote::quote!(#[openapi]), quote::quote!(#[openapi = Metadata])].into_iter().all(|attribute| crate::route_registry(quote::quote! {
        #attribute pub; state = State, family = Family; (Authentication, Csrf); schemas(Schema); (Route, endpoint)
    }).to_string().contains(constants_str::ROUTE_REGISTRY_REQUIRES_OPENAPI_ATTRIBUTE)));
}

#[test]
fn test_route_registry_expands_qualified_endpoints_schema_and_route_membership() {
    let output = crate::route_registry(quote::quote! {
        #[openapi(tags())] pub; state = State, family = Family; (Authentication, Csrf);
        schemas(FirstSchema, SecondSchema); (FirstRoute, first::endpoint), (SecondRoute, second::endpoint)
    });
    let text = output.to_string();
    assert!(
        text.contains(
            quote::quote!(#[openapi(paths(first::endpoint, second::endpoint), tags())])
                .to_string()
                .as_str()
        )
    );
    assert!(text.contains(quote::quote!([(); 2usize]).to_string().as_str()));
    assert!(
        [
            quote::quote!(first::__path_endpoint),
            quote::quote!(second::__path_endpoint)
        ]
        .into_iter()
        .all(|path| text.contains(
            quote::quote!(<#path as utoipa::Path>::operation())
                .to_string()
                .as_str()
        ))
    );
    assert!(
        [quote::quote!(FirstRoute), quote::quote!(SecondRoute)]
            .into_iter()
            .all(|route| text.contains(
                quote::quote!(assert_route_family_membership::<#route>();)
                    .to_string()
                    .as_str()
            ))
    );
    assert!([quote::quote!(FirstSchema), quote::quote!(SecondSchema)].into_iter().all(|schema| text.contains(quote::quote!(frontend_contract::register_openapi_schema::register_openapi_schema::<#schema>).to_string().as_str())));
    assert!(syn::parse2::<syn::File>(output).is_ok_and(|file| {
        let functions = file.items.iter().filter_map(|item| if let syn::Item::Fn(function) = item { Some(function) } else { None }).collect::<Vec<_>>();
        matches!(functions.as_slice(), [openapi, router, limited]
            if openapi.sig.ident == stringify!(open_api)
                && router.sig.ident == stringify!(router)
                && limited.sig.ident == stringify!(router_with_body_limit)
                && functions.iter().all(|function| matches!(function.vis, syn::Visibility::Public(_))))
    }));
}

#[test]
fn test_typed_route_error_response_schema_handles_absent_unit_and_typed_choices() {
    assert!([
        (quote::quote!(), false, false),
        (quote::quote!(error_response = (),), true, false),
        (quote::quote!(error_response = ErrorResponse,), true, true),
    ].into_iter().all(|(optional, has_method, has_schema)| {
        let output = crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy, #optional
                method = Get, openapi_operation_id = Operation, path = Path, request = Request,
                response = Response, success_status = Status, transport = Transport)] struct Route;
        });
        let text = output.to_string();
        text.contains(stringify!(openapi_error_response_schema)) == has_method
            && text.contains(quote::quote!(frontend_contract::register_openapi_schema::register_openapi_schema::<ErrorResponse>(components);).to_string().as_str()) == has_schema
            && (!has_schema || text.contains(quote::quote!(<ErrorResponse as utoipa::PartialSchema>::schema()).to_string().as_str()))
            && syn::parse2::<syn::File>(output).is_ok_and(|file| !file.items.is_empty())
    }));
}

#[test]
fn test_typed_route_vector_response_omits_duplicate_schema_registration() {
    assert!([quote::quote!(Response), quote::quote!(Vec<Response>)].into_iter().all(|response| {
        let output = crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy,
                method = Get, openapi_operation_id = Operation, path = Path, request = Request,
                response = #response, success_status = Status, transport = Transport)] struct Route;
        });
        let text = output.to_string();
        text.contains(quote::quote!(<#response as utoipa::PartialSchema>::schema()).to_string().as_str())
            && text.contains(quote::quote!(frontend_contract::register_openapi_schema::register_openapi_schema::<#response>(components);).to_string().as_str()) == (response.to_string() == quote::quote!(Response).to_string())
    }));
}

#[test]
fn test_typed_route_preserves_explicit_policy_body_mutation_and_obligations() {
    assert!([false, true].into_iter().all(|explicit| {
        let options = if explicit { quote::quote!(error_statuses = Statuses, mutation = Mutation, request_body = Body, obligations = Obligations,) } else { quote::quote!(error_policy = Policy,) };
        let output = crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, #options method = Get, openapi_operation_id = Operation,
                path = Path, request = Request, response = Response, success_status = Status, transport = Transport)] struct Route;
        }).to_string();
        if explicit {
            output.contains(quote::quote! { fn request_body() -> frontend_contract::route_request_body::RouteRequestBody { Body } }.to_string().as_str())
                && output.contains(quote::quote!(Authentication, Statuses, frontend_contract::route_method::RouteMethod::Get, Mutation,).to_string().as_str())
                && output.contains(stringify!(Obligations))
                && !output.contains(quote::quote!((Policy).statuses).to_string().as_str())
        } else {
            output.contains(quote::quote!((Policy).statuses(Authentication, frontend_contract::route_mutation::RouteMutation::ReadOnly)).to_string().as_str())
                && output.contains(quote::quote! { fn request_body() -> frontend_contract::route_request_body::RouteRequestBody { frontend_contract::route_request_body::RouteRequestBody::Absent } }.to_string().as_str())
        }
    }));
}

#[test]
fn test_typed_route_named_functions_use_literal_operation_or_struct_name() {
    let literal = syn::LitStr::new(stringify!(custom_operation), proc_macro2::Span::call_site());
    assert!([
        (quote::quote!(#literal), quote::quote!(custom_operation_route), quote::quote!(custom_operation_client)),
        (quote::quote!(Operation), quote::quote!(account_list_route), quote::quote!(account_list_client)),
        (quote::quote!(1), quote::quote!(account_list_route), quote::quote!(account_list_client)),
    ].into_iter().all(|(operation, route, client)| {
        let output = crate::derive_typed_route(quote::quote! {
            #[typed_route(authentication = Authentication, error_policy = Policy, method = Get, openapi_operation_id = #operation,
                path = Path, request = Request, response = Response, success_status = Status, transport = Transport)] pub struct AccountListRoute;
        }).to_string();
        output.contains(quote::quote!(pub fn #route()).to_string().as_str())
            && output.contains(quote::quote!(pub async fn #client<Transport>).to_string().as_str())
    }));
}

#[test]
fn test_contract_macro_entrypoints_preserve_malformed_input_diagnostics() {
    let input = quote::quote!(1);
    let expected = syn::parse2::<syn::DeriveInput>(input.clone())
        .err()
        .map(|error| error.to_compile_error().to_string());
    assert!(expected.is_some());
    assert!(
        [
            crate::derive_contract_struct_api(input.clone()),
            crate::derive_typed_route(input.clone()),
            crate::derive_route_catalog(input.clone()),
            crate::derive_page_catalog(input.clone()),
            crate::derive_route_family(input),
        ]
        .into_iter()
        .all(|output| expected
            .as_deref()
            .is_some_and(|diagnostic| output.to_string() == diagnostic))
    );
}

#[test]
fn test_openapi_macro_preserves_function_and_metadata_parse_errors() {
    let invalid_function = quote::quote!(1);
    let invalid_metadata = quote::quote!(,);
    let expected_function = syn::parse2::<syn::ItemFn>(invalid_function.clone())
        .err()
        .map(|error| error.to_compile_error().to_string());
    let expected_metadata = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        invalid_metadata.clone(),
    )
    .err()
    .map(|error| error.to_compile_error().to_string());
    assert!(expected_function.is_some());
    assert!(expected_metadata.is_some());
    assert!(expected_function.is_some_and(|diagnostic| {
        crate::route_openapi(quote::quote!(), invalid_function).to_string() == diagnostic
    }));
    assert!(expected_metadata.is_some_and(|diagnostic| {
        crate::route_openapi(
            invalid_metadata,
            quote::quote!(
                async fn endpoint() {}
            ),
        )
        .to_string()
            == diagnostic
    }));
}

#[test]
fn test_registry_entrypoints_preserve_visibility_and_separator_diagnostics() {
    let invalid_visibility = quote::quote!(pub(in));
    let expected_visibility = syn::parse2::<syn::Visibility>(invalid_visibility.clone())
        .err()
        .map(|error| error.to_compile_error().to_string());
    let expected_separator = syn::parse2::<syn::Token![;]>(quote::quote!(,))
        .err()
        .map(|error| error.to_compile_error().to_string());
    assert!(expected_visibility.is_some());
    assert!(expected_separator.is_some());
    assert!(
        [
            (
                crate::endpoint_registry(invalid_visibility.clone()),
                &expected_visibility
            ),
            (
                crate::route_registry(quote::quote!(#[openapi()] #invalid_visibility)),
                &expected_visibility
            ),
            (
                crate::endpoint_registry(quote::quote!(pub,)),
                &expected_separator
            ),
            (
                crate::route_registry(quote::quote!(#[openapi()] pub ,)),
                &expected_separator
            ),
        ]
        .into_iter()
        .all(|(output, expected)| expected
            .as_deref()
            .is_some_and(|diagnostic| output.to_string() == diagnostic))
    );
}

#[test]
fn test_registry_entrypoints_preserve_inner_argument_diagnostics() {
    let invalid_arguments = quote::quote!(,);
    let expected_endpoint = syn::parse2::<crate::endpoint_registry_args::EndpointRegistryArgs>(
        invalid_arguments.clone(),
    )
    .err()
    .map(|error| error.to_compile_error().to_string());
    let expected_route =
        syn::parse2::<crate::route_registry_args::RouteRegistryArgs>(invalid_arguments.clone())
            .err()
            .map(|error| error.to_compile_error().to_string());
    assert!(expected_endpoint.is_some());
    assert!(expected_route.is_some());
    assert!(expected_endpoint.is_some_and(|diagnostic| {
        crate::endpoint_registry(quote::quote!(pub; #invalid_arguments)).to_string() == diagnostic
    }));
    assert!(expected_route.is_some_and(|diagnostic| {
        crate::route_registry(quote::quote!(#[openapi()] pub; #invalid_arguments)).to_string()
            == diagnostic
    }));
}

#[test]
fn test_catalog_entrypoints_preserve_nested_attribute_diagnostics() {
    let invalid_arguments = quote::quote!(,);
    assert!(
        [
            (
                crate::derive_typed_route(quote::quote!(
                    #[typed_route(#invalid_arguments)]
                    struct Route;
                )),
                syn::parse2::<crate::typed_route_args::TypedRouteArgs>(invalid_arguments.clone())
                    .map(|_args| ())
            ),
            (
                crate::derive_route_catalog(quote::quote!(
                    #[route_catalog(#invalid_arguments)]
                    enum Catalog {
                        First,
                    }
                )),
                syn::parse2::<crate::route_catalog_args::RouteCatalogArgs>(
                    invalid_arguments.clone()
                )
                .map(|_args| ())
            ),
            (
                crate::derive_route_catalog(quote::quote!(
                    #[route_catalog(body_limit = Limit, family = Family)]
                    enum Catalog {
                        #[route_catalog_route(#invalid_arguments)]
                        First,
                    }
                )),
                syn::parse2::<crate::route_catalog_route_args::RouteCatalogRouteArgs>(
                    invalid_arguments.clone()
                )
                .map(|_args| ())
            ),
            (
                crate::derive_page_catalog(quote::quote!(
                    #[page_catalog(#invalid_arguments)]
                    enum Catalog {
                        First,
                    }
                )),
                syn::parse2::<crate::page_catalog_args::PageCatalogArgs>(invalid_arguments)
                    .map(|_args| ())
            ),
        ]
        .into_iter()
        .all(|(output, expected)| expected
            .is_err_and(|error| output.to_string() == error.to_compile_error().to_string()))
    );
}

#[test]
fn test_openapi_delegate_preserves_ordered_metadata() {
    let metadata = quote::quote!(responses(Response), security(Auth));
    let output = crate::route_openapi(
        quote::quote!(
            responses(Response),
            delegate = implementation,
            security(Auth)
        ),
        quote::quote!(
            async fn endpoint() -> Result<Response, Error> {}
        ),
    );
    assert!(syn::parse2::<syn::ItemFn>(output).is_ok_and(|function| {
        function
            .attrs
            .first()
            .is_some_and(|attribute| match &attribute.meta {
                syn::Meta::List(list) => {
                    let expected_path = syn::LitStr::new(
                        ['/', '_', '_']
                            .into_iter()
                            .chain(constants_str::TYPED_ROUTE.chars())
                            .chain(std::iter::once('_'))
                            .chain(function.sig.ident.to_string().chars())
                            .collect::<String>()
                            .as_str(),
                        proc_macro2::Span::call_site(),
                    );
                    list.path == syn::parse_quote!(utoipa::path)
                        && list.tokens.to_string()
                            == quote::quote!(get, path = #expected_path, #metadata).to_string()
                }
                syn::Meta::Path(_) | syn::Meta::NameValue(_) => false,
            })
            && quote::ToTokens::to_token_stream(&function.block).to_string()
                == quote::quote!({ implementation().await.map_err(Error::from) }).to_string()
    }));
}

#[test]
fn test_contract_slice_accessor_preserves_value_and_type_syntax_errors() {
    let expected_equals = syn::parse2::<syn::Token![=]>(quote::quote!(,))
        .err()
        .map(|error| error.to_compile_error().to_string());
    let expected_type = syn::parse2::<syn::Type>(quote::quote!(,))
        .err()
        .map(|error| error.to_compile_error().to_string());
    assert!(expected_equals.is_some());
    assert!(expected_type.is_some());
    assert!(
        [
            (
                quote::quote!(
                    struct Request {
                        #[contract_struct_api(slice)]
                        value: Values,
                    }
                ),
                &expected_equals
            ),
            (
                quote::quote!(
                    struct Request {
                        #[contract_struct_api(slice = ,)]
                        value: Values,
                    }
                ),
                &expected_type
            ),
        ]
        .into_iter()
        .all(
            |(input, expected)| expected.as_deref().is_some_and(|diagnostic| {
                crate::derive_contract_struct_api(input).to_string() == diagnostic
            })
        )
    );
}

#[test]
fn test_route_registry_preserves_malformed_outer_attribute_diagnostic() {
    let input = quote::quote!(#[openapi = ]);
    let expected = syn::parse::Parser::parse2(syn::Attribute::parse_outer, input.clone())
        .err()
        .map(|error| error.to_compile_error().to_string());
    assert!(expected.is_some());
    assert!(
        expected.is_some_and(|diagnostic| crate::route_registry(input).to_string() == diagnostic)
    );
}

#[test]
fn test_openapi_without_delegate_preserves_original_function_and_metadata() {
    let input = quote::quote!(
        #[allow(dead_code)]
        pub async fn endpoint(value: Value) -> Response {
            response(value)
        }
    );
    let output = crate::route_openapi(
        quote::quote!(responses(Response), security(Auth)),
        input.clone(),
    );
    assert!(
        syn::parse2::<syn::ItemFn>(output).is_ok_and(|mut function| {
            let attribute = function.attrs.remove(0usize);
            attribute.path() == &syn::parse_quote!(utoipa::path)
                && attribute
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .is_ok_and(|arguments| {
                        arguments
                            .into_iter()
                            .skip(2usize)
                            .map(|argument| quote::quote!(#argument).to_string())
                            .eq([
                                quote::quote!(responses(Response)).to_string(),
                                quote::quote!(security(Auth)).to_string(),
                            ])
                    })
                && quote::quote!(#function).to_string() == input.to_string()
        })
    );
}
