use super::{ImportPathStr, ImportSnakeCaseStr};

#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum Import {
    Crate,
    PgCrudCommon,
}
impl Import {
    pub(crate) fn all_enum_variants(
        self,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            match self {
                Self::Crate => quote::quote! { crate::AllEnumVariants },
                Self::PgCrudCommon => {
                    quote::quote! { pg_crud_common::domain_types::AllEnumVariants }
                }
            },
        )
    }
    pub(crate) fn all_variants_default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElement,
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElement
            }
        }
    }
    pub(crate) fn all_variants_default_some_one_element_max_page_size(
        &self,
    ) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => {
                &token_patterns::CrateAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
            Self::PgCrudCommon => {
                &token_patterns::PgCrudCommonAllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
            }
        }
    }
    pub(crate) fn default_some_one_element(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultSomeOneElement,
            Self::PgCrudCommon => &token_patterns::PgCrudCommonDefaultSomeOneElement,
        }
    }
    pub(crate) fn default_some_one_element_max_page_size(&self) -> &dyn quote::ToTokens {
        match &self {
            Self::Crate => &token_patterns::CrateDefaultSomeOneElementMaxPageSize,
            Self::PgCrudCommon => &token_patterns::PgCrudCommonDefaultSomeOneElementMaxPageSize,
        }
    }
    #[must_use]
    pub fn sc_str(&self) -> ImportSnakeCaseStr {
        match &self {
            Self::Crate => ImportSnakeCaseStr::from(constants_str::CRATE),
            Self::PgCrudCommon => {
                ImportSnakeCaseStr::from(constants_str::PG_CRUD_COMMON_DOMAIN_TYPES)
            }
        }
    }
    #[must_use]
    pub fn to_path(&self) -> ImportPathStr {
        match &self {
            Self::Crate => ImportPathStr::from(constants_str::CRATE),
            Self::PgCrudCommon => ImportPathStr::from(constants_str::PG_CRUD_COMMON_DOMAIN_TYPES),
        }
    }
}
impl quote::ToTokens for Import {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote! { crate },
            Self::PgCrudCommon => quote::quote! { pg_crud_common::domain_types },
        }
        .to_tokens(tokens);
    }
}
