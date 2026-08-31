#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum IsNullable {
    #[default]
    False,
    True,
}
impl IsNullable {
    #[must_use]
    pub fn maybe_optional_wrap(
        &self,
        ts: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
    {
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Option<#ts>}.into(),
        }
    }
    #[must_use]
    pub fn maybe_some_wrap(
        &self,
        ts: macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
    {
        match &self {
            Self::False => ts,
            Self::True => quote::quote! {Some(#ts)}.into(),
        }
    }
    #[must_use]
    pub fn non_null_or_nullable_str(
        &self,
    ) -> crate::non_null_or_nullable_str::NonNullOrNullableStr {
        match &self {
            Self::False => {
                crate::non_null_or_nullable_str::NonNullOrNullableStr::from(constants_str::NONNULL)
            }
            Self::True => {
                crate::non_null_or_nullable_str::NonNullOrNullableStr::from(constants_str::NULLABLE)
            }
        }
    }
    #[must_use]
    pub fn prefix_str(&self) -> crate::is_nullable_prefix_str::IsNullablePrefixStr {
        match &self {
            Self::False => {
                crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(String::default())
                    .unwrap_or_else(crate::is_nullable_prefix_str::IsNullablePrefixStr::from)
            }
            Self::True => crate::is_nullable_prefix_str::IsNullablePrefixStr::try_from(
                String::from(constants_str::STDOPTIONALOPTIONAL),
            )
            .unwrap_or_else(crate::is_nullable_prefix_str::IsNullablePrefixStr::from),
        }
    }
    #[must_use]
    pub fn rust(&self) -> &'static dyn std::fmt::Display {
        match &self {
            Self::False => &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            Self::True => &naming::domain_types::OptionalUpperCamelCase,
        }
    }
}
