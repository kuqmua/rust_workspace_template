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
