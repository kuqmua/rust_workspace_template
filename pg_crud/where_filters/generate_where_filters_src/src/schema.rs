#![allow(
    clippy::single_call_fn,
    reason = "the schema emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(super) fn schema_uses_text_value(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSpecValid {
    spec.has_text_value_shape()
}
pub(super) fn text_search_token_stream(
    spec: crate::model::FilterSpec,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    if !schema_uses_text_value(spec).get() {
        return quote::quote! {compile_error!("text search schema requires text value shape");}
            .into();
    }
    quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct TextSearchMaximumInputBytes(usize);
        impl From<TextSearchMaximumInputBytes> for usize {
            fn from(value: TextSearchMaximumInputBytes) -> Self {
                value.0
            }
        }
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
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct TextSearchPattern(String);
        impl AsRef<str> for TextSearchPattern {
            fn as_ref(&self) -> &str {
                self.0.as_str()
            }
        }
        impl From<TextSearchPattern> for String {
            fn from(value: TextSearchPattern) -> Self {
                value.0
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum TextSearchValueError {
            Empty,
            TooLong { actual_bytes: usize, maximum_bytes: usize },
        }
        impl std::fmt::Display for TextSearchValueError {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Empty => formatter.write_str("text search value must not be empty"),
                    Self::TooLong { actual_bytes, maximum_bytes } => write!(formatter, "text search value exceeds {maximum_bytes} bytes: got {actual_bytes}"),
                }
            }
        }
        impl std::error::Error for TextSearchValueError {}
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
                TextSearchMode::EndsWith | TextSearchMode::StartsWith => 1usize,
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
            operator: pg_crud_common::Operator,
        }
    }
    .into()
}
