#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum Import {
    Crate,
    PgCrudCommon,
}
impl Import {
    pub(crate) fn all_enum_variants(
        self,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
    {
        macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            match self {
                Self::Crate => quote::quote! { crate::AllEnumVariants },
                Self::PgCrudCommon => {
                    quote::quote! { pg_crud_common::all_enum_variants::AllEnumVariants }
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
    pub fn sc_str(&self) -> crate::import_snake_case_str::ImportSnakeCaseStr {
        match &self {
            Self::Crate => {
                crate::import_snake_case_str::ImportSnakeCaseStr::from(constants_str::CRATE)
            }
            Self::PgCrudCommon => crate::import_snake_case_str::ImportSnakeCaseStr::from(
                constants_str::PG_CRUD_COMMON_DOMAIN_TYPES,
            ),
        }
    }
    #[must_use]
    pub fn to_path(&self) -> crate::import_path_str::ImportPathStr {
        match &self {
            Self::Crate => crate::import_path_str::ImportPathStr::from(constants_str::CRATE),
            Self::PgCrudCommon => crate::import_path_str::ImportPathStr::from(
                constants_str::PG_CRUD_COMMON_DOMAIN_TYPES,
            ),
        }
    }
}
impl quote::ToTokens for Import {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        match &self {
            Self::Crate => quote::quote! { crate },
            Self::PgCrudCommon => quote::quote! { pg_crud_common },
        }
        .to_tokens(token_stream);
    }
}
