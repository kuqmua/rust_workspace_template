#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct PageCatalogPageArgs {
    pub capability: crate::syn_expr::SynExpr,
    pub metadata: crate::syn_expr::SynExpr,
    pub path: crate::syn_expr::SynExpr,
    pub route: crate::syn_expr::SynExpr,
    pub title: crate::syn_expr::SynExpr,
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
            let value = crate::syn_expr::SynExpr::from(input.parse::<syn::Expr>()?);
            if name == constants_str::test_fixtures::PAGE_CATALOG_CAPABILITY {
                capability = Some(value);
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_METADATA {
                metadata = Some(value);
            } else if name == constants_str::test_fixtures::ROUTE_CATALOG_PATH {
                path = Some(value);
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_ROUTE {
                route = Some(value);
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_TITLE {
                title = Some(value);
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::test_fixtures::PAGE_CATALOG_PAGE_REQUIRES_FIELDS,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        let missing =
            || input.error(constants_str::test_fixtures::PAGE_CATALOG_PAGE_REQUIRES_FIELDS);
        Ok(Self {
            capability: capability.ok_or_else(&missing)?,
            metadata: metadata.ok_or_else(&missing)?,
            path: path.ok_or_else(&missing)?,
            route: route.ok_or_else(&missing)?,
            title: title.ok_or_else(missing)?,
        })
    }
}
