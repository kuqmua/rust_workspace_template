#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogArgs {
    body_limit: crate::contract_syn_expr::ContractSynExpr,
    family: crate::contract_syn_ident::ContractSynIdent,
}

impl syn::parse::Parse for RouteCatalogArgs {
    fn parse(parse_stream: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut body_limit = None;
        let mut family = None;
        while !parse_stream.is_empty() {
            let name = parse_stream.parse::<syn::Ident>()?;
            let _equals = parse_stream.parse::<syn::Token![=]>()?;
            if name == constants_str::ROUTE_CATALOG_FAMILY {
                family = Some(crate::contract_syn_ident::ContractSynIdent::from(
                    parse_stream.parse::<syn::Ident>()?,
                ));
            } else if name == constants_str::ROUTE_CATALOG_BODY_LIMIT {
                body_limit = Some(crate::contract_syn_expr::ContractSynExpr::from(
                    parse_stream.parse::<syn::Expr>()?,
                ));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::UNSUPPORTED_TYPED_ROUTE_FIELD,
                ));
            }
            if !parse_stream.is_empty() {
                let _comma = parse_stream.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            body_limit: body_limit.ok_or_else(|| {
                parse_stream.error(constants_str::ROUTE_CATALOG_REQUIRES_BODY_LIMIT)
            })?,
            family: family
                .ok_or_else(|| parse_stream.error(constants_str::ROUTE_CATALOG_REQUIRES_FAMILY))?,
        })
    }
}
