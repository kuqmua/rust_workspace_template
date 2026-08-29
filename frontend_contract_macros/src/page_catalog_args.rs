#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
pub(crate) struct PageCatalogArgs {
    pub inventory: crate::syn_ident::SynIdent,
    pub path_ref: crate::syn_type::SynType,
    pub spec: crate::syn_type::SynType,
}

impl syn::parse::Parse for PageCatalogArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut inventory = None;
        let mut path_ref = None;
        let mut spec = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::test_fixtures::PAGE_CATALOG_INVENTORY {
                inventory = Some(crate::syn_ident::SynIdent::from(
                    input.parse::<syn::Ident>()?,
                ));
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_PATH_REF {
                path_ref = Some(crate::syn_type::SynType::from(input.parse::<syn::Type>()?));
            } else if name == constants_str::test_fixtures::PAGE_CATALOG_SPEC {
                spec = Some(crate::syn_type::SynType::from(input.parse::<syn::Type>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            inventory: inventory.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            path_ref: path_ref.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
            spec: spec.ok_or_else(|| {
                input.error(constants_str::test_fixtures::PAGE_CATALOG_REQUIRES_ATTRIBUTE)
            })?,
        })
    }
}
