use crate::domain_types::{SynExpr, SynIdentifier};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
#[derive(generate_accessor::Getters)]
pub(crate) struct WireEnumAttrs {
    error_message: SynExpr,
    ref_type: SynIdentifier,
}
impl syn::parse::Parse for WireEnumAttrs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut error_message = None;
        let mut ref_type = None;
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?;
            let _equals = input.parse::<syn::Token![=]>()?;
            if name == constants_str::WIRE_ENUM_REF_TYPE {
                ref_type = Some(SynIdentifier::from(input.parse::<syn::Ident>()?));
            } else if name == constants_str::WIRE_ENUM_ERROR_MESSAGE {
                error_message = Some(SynExpr::from(input.parse::<syn::Expr>()?));
            } else {
                return Err(syn::Error::new_spanned(
                    name,
                    constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE,
                ));
            }
            if !input.is_empty() {
                let _comma = input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self {
            error_message: error_message
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
            ref_type: ref_type
                .ok_or_else(|| input.error(constants_str::WIRE_ENUM_REQUIRES_ATTRIBUTE))?,
        })
    }
}
