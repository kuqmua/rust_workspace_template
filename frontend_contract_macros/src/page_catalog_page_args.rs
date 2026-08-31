#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct PageCatalogPageArgs {
    capability: crate::contract_syn_expr::ContractSynExpr,
    metadata: crate::contract_syn_expr::ContractSynExpr,
    path: crate::contract_syn_expr::ContractSynExpr,
    route: crate::contract_syn_expr::ContractSynExpr,
    title: crate::contract_syn_expr::ContractSynExpr,
}

impl syn::parse::Parse for PageCatalogPageArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut capability = None;
        let mut metadata = None;
        let mut path = None;
        let mut route = None;
        let mut title = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            let value =
                crate::contract_syn_expr::ContractSynExpr::from(input.parse::<syn::Expr>()?);
            if name == constants_str::PAGE_CATALOG_CAPABILITY {
                capability = Some(value);
            } else if name == constants_str::PAGE_CATALOG_METADATA {
                metadata = Some(value);
            } else if name == constants_str::ROUTE_CATALOG_PATH {
                path = Some(value);
            } else if name == constants_str::PAGE_CATALOG_ROUTE {
                route = Some(value);
            } else if name == constants_str::PAGE_CATALOG_TITLE {
                title = Some(value);
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        let missing = || input.error(constants_str::PAGE_CATALOG_PAGE_REQUIRES_FIELDS);
        Ok(Self {
            capability: capability.ok_or_else(&missing)?,
            metadata: metadata.ok_or_else(&missing)?,
            path: path.ok_or_else(&missing)?,
            route: route.ok_or_else(&missing)?,
            title: title.ok_or_else(missing)?,
        })
    }
}
