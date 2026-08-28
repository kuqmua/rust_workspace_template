use super::{COLLECTION_MAX_LEN, TopLevelCommaPart};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Default)]
pub struct ProcMacro2TopLevelCommaParts(Vec<proc_macro2::TokenStream>);
impl TryFrom<Vec<proc_macro2::TokenStream>> for ProcMacro2TopLevelCommaParts {
    type Error = syn::Error;
    fn try_from(value: Vec<proc_macro2::TokenStream>) -> Result<Self, Self::Error> {
        if value.len() > COLLECTION_MAX_LEN {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                stringify!(e54c7219 too many top-level comma parts),
            ))
        } else {
            Ok(Self(value))
        }
    }
}
impl std::ops::Deref for ProcMacro2TopLevelCommaParts {
    type Target = Vec<proc_macro2::TokenStream>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ProcMacro2TopLevelCommaParts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for ProcMacro2TopLevelCommaParts {
    type IntoIter = std::vec::IntoIter<proc_macro2::TokenStream>;
    type Item = proc_macro2::TokenStream;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl syn::parse::Parse for ProcMacro2TopLevelCommaParts {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let parts =
            syn::punctuated::Punctuated::<TopLevelCommaPart, syn::Token![,]>::parse_terminated(
                input,
            )?
            .into_iter()
            .map(|part| part.0.into_inner())
            .collect::<Vec<_>>();
        Self::try_from(parts)
    }
}
