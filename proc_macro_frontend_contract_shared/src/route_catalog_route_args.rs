#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters)]
#[getters(get_mut)]
#[derive(proc_macro_new::New)]
pub(crate) struct RouteCatalogRouteArgs {
    contract: Option<crate::contract_syn_expr::ContractSynExpr>,
    path: Option<crate::contract_syn_expr::ContractSynExpr>,
    route: Option<crate::contract_syn_type::ContractSynType>,
    exclude_from_family: crate::std_bool::StdBool,
}
impl syn::parse::Parse for RouteCatalogRouteArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if parse_stream.peek(syn::Ident) && parse_stream.peek2(syn::Token![=]) {
            let mut contract = None;
            let mut exclude_from_family = crate::std_bool::StdBool::from(false);
            let mut path = None;
            while !parse_stream.is_empty() {
                let name = parse_stream.parse::<syn::Ident>()?;
                if name == constants_str::ROUTE_CATALOG_EXCLUDE_FROM_FAMILY {
                    exclude_from_family = crate::std_bool::StdBool::from(true);
                } else {
                    let _equals = parse_stream.parse::<syn::Token![=]>()?;
                    if name == constants_str::ROUTE_CATALOG_CONTRACT {
                        contract = Some(crate::contract_syn_expr::ContractSynExpr::from(
                            parse_stream.parse::<syn::Expr>()?,
                        ));
                    } else if name == constants_str::ROUTE_CATALOG_PATH {
                        path = Some(crate::contract_syn_expr::ContractSynExpr::from(
                            parse_stream.parse::<syn::Expr>()?,
                        ));
                    } else {
                        return Err(syn::Error::new_spanned(
                            name,
                            constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                        ));
                    }
                }
                if !parse_stream.is_empty() {
                    let _comma = parse_stream.parse::<syn::Token![,]>()?;
                }
            }
            if contract.is_none() || path.is_none() {
                return Err(parse_stream
                    .error(constants_str::ROUTE_CATALOG_ROUTE_REQUIRES_TYPE_OR_CUSTOM_VALUES));
            }
            Ok(Self::new(contract, path, None, exclude_from_family))
        } else {
            Ok(Self::new(
                None,
                None,
                Some(crate::contract_syn_type::ContractSynType::from(
                    parse_stream.parse::<syn::Type>()?,
                )),
                crate::std_bool::StdBool::from(false),
            ))
        }
    }
}
