use crate::SynIdent;

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
pub(crate) struct PageCatalogArgs {
    pub inventory: SynIdent,
    pub path_ref: SynIdent,
    pub spec: SynIdent,
}

impl syn::parse::Parse for PageCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::PAGE_CATALOG_INVENTORY {
                inventory = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::PAGE_CATALOG_PATH_REF {
                path_ref = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::PAGE_CATALOG_SPEC {
                spec = Some(SynIdent::from(input.parse::<syn::Ident>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            path_ref: path_ref
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
            spec: spec
                .ok_or_else(|| input.error(constants_str::PAGE_CATALOG_REQUIRES_ATTRIBUTE))?,
        })
    }
}
