#![allow(
    clippy::single_call_fn,
    reason = "the schema emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(crate) fn schema_text_search_token_stream(
    spec: crate::spec::FilterSpec,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    if !crate::schema_uses_text_value::schema_uses_text_value(spec).get() {
        return quote::quote! {compile_error!("text search schema requires text value shape");}
            .into();
    }
    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom)]
        pub struct TextSearchMaximumInputBytes(usize);
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct TextSearchPolicy {
            maximum_input_bytes: TextSearchMaximumInputBytes,
        }
        impl TextSearchPolicy {
            pub const DEFAULT: Self = Self {
                maximum_input_bytes: TextSearchMaximumInputBytes(1_024usize),
            };
            pub const fn maximum_input_bytes(self) -> TextSearchMaximumInputBytes {
                self.maximum_input_bytes
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, utoipa::ToSchema)]
        #[serde(rename_all = "snake_case")]
        pub enum TextSearchMode {
            Contains,
            EndsWith,
            StartsWith,
        }
        #[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefStr, newtype::IntoInnerFrom)]
        pub struct TextSearchPattern(String);
        #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
        pub enum TextSearchValueError {
            #[error("text search value must not be empty")]
            Empty,
            #[error("text search value exceeds {maximum_bytes} bytes: got {actual_bytes}")]
            TooLong { actual_bytes: usize, maximum_bytes: usize },
        }
        pub fn build_text_search_pattern(value: &str, mode: TextSearchMode) -> Result<TextSearchPattern, TextSearchValueError> {
            if value.is_empty() {
                return Err(TextSearchValueError::Empty);
            }
            let maximum_input_bytes = usize::from(TextSearchPolicy::DEFAULT.maximum_input_bytes());
            if value.len() > maximum_input_bytes {
                return Err(TextSearchValueError::TooLong {
                    actual_bytes: value.len(),
                    maximum_bytes: maximum_input_bytes,
                });
            }
            let wildcard_count = match mode {
                TextSearchMode::Contains => 2usize,
                TextSearchMode::EndsWith | TextSearchMode::StartsWith => constants_usize::ONE,
            };
            let escaped_symbol_count = value.as_bytes().iter().copied().filter(|byte| matches!(byte, b'\\' | b'%' | b'_')).count();
            let mut pattern = String::with_capacity(value.len().saturating_add(escaped_symbol_count).saturating_add(wildcard_count));
            if matches!(mode, TextSearchMode::Contains | TextSearchMode::EndsWith) {
                pattern.push('%');
            }
            value.chars().for_each(|character| {
                if matches!(character, '\\' | '%' | '_') {
                    pattern.push('\\');
                }
                pattern.push(character);
            });
            if matches!(mode, TextSearchMode::Contains | TextSearchMode::StartsWith) {
                pattern.push('%');
            }
            Ok(TextSearchPattern(pattern))
        }
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema, utoipa::ToSchema)]
        #[serde(deny_unknown_fields)]
        pub struct PgTypeWhereTextSearch {
            value: String,
            mode: TextSearchMode,
            operator: pg_crud_common::domain_types::Operator,
        }
    }
    .into()
}
