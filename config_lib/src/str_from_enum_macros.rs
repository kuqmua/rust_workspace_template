const ALLOWED_VALUES_SEPARATOR: &str = ", ";
#[derive(Debug, Clone, Copy)]
pub struct EnumInputRef<'input_lt>(pub &'input_lt str);
#[derive(Debug)]
pub struct EnumPairsRef<'pairs_lt, T>(pub &'pairs_lt [(&'static str, T)]);
impl<T> Clone for EnumPairsRef<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for EnumPairsRef<'_, T> {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AllowedValuesCapacity(usize);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedValues(String);
impl std::fmt::Display for AllowedValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for AllowedValues {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct UnknownEnumInput(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumParseEr {
    allowed_values: AllowedValues,
    input: UnknownEnumInput,
}
impl std::fmt::Display for EnumParseEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unknown value: {}. Allowed values: {}",
            self.input.0, self.allowed_values.0
        )
    }
}
impl From<EnumParseEr> for String {
    fn from(value: EnumParseEr) -> Self {
        value.to_string()
    }
}
#[allow(clippy::single_call_fn)] // extracted for reuse by allowed-values formatter and tests
fn allowed_values_capacity<T>(vrts: EnumPairsRef<'_, T>) -> AllowedValuesCapacity {
    AllowedValuesCapacity(
        vrts.0
            .iter()
            .map(|(name, _)| name.len())
            .sum::<usize>()
            .saturating_add(
                vrts.0
                    .len()
                    .saturating_sub(1)
                    .saturating_mul(ALLOWED_VALUES_SEPARATOR.len()),
            ),
    )
}
#[allow(clippy::single_call_fn)] // extracted to keep allowed-values formatting reusable and tested
fn mk_allowed_values<T>(vrts: EnumPairsRef<'_, T>) -> AllowedValues {
    let allowed_values = vrts.0.iter().enumerate().fold(
        String::with_capacity(allowed_values_capacity(vrts).0),
        |mut acc, (idx, (name, _))| {
            if idx != 0 {
                acc.push_str(ALLOWED_VALUES_SEPARATOR);
            }
            acc.push_str(name);
            acc
        },
    );
    AllowedValues(allowed_values)
}
#[allow(clippy::single_call_fn)] // extracted lookup keeps case-insensitive enum-pair search reusable and testable
fn find_case_insensitive_pair<T>(v: EnumInputRef<'_>, vrts: EnumPairsRef<'_, T>) -> Option<T>
where
    T: Copy,
{
    vrts.0
        .iter()
        .find_map(|(str_vrt, enum_vrt)| v.0.eq_ignore_ascii_case(str_vrt).then_some(*enum_vrt))
}
pub fn impl_from_str_for_enum_helper<T, Er>(
    v: EnumInputRef<'_>,
    vrts: EnumPairsRef<'_, T>,
) -> Result<T, Er>
where
    T: Copy,
    Er: From<EnumParseEr>,
{
    find_case_insensitive_pair(v, vrts).ok_or_else(|| {
        let allowed_values = mk_allowed_values(vrts);
        EnumParseEr {
            input: UnknownEnumInput(v.0.to_owned()),
            allowed_values,
        }
        .into()
    })
}
#[cfg(test)]
mod tests {
    const PAIRS: [(&str, V); 2] = [("a", V::A), ("b", V::B)];
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum V {
        A,
        B,
    }
    #[test]
    fn helper_parses_values_case_insensitively() {
        assert_eq!(
            super::impl_from_str_for_enum_helper::<V, String>(
                super::EnumInputRef("A"),
                super::EnumPairsRef(&PAIRS)
            ),
            Ok(V::A)
        );
        assert_eq!(
            super::impl_from_str_for_enum_helper::<V, String>(
                super::EnumInputRef("b"),
                super::EnumPairsRef(&PAIRS)
            ),
            Ok(V::B)
        );
    }
    #[test]
    fn find_case_insensitive_pair_returns_none_for_unknown_value() {
        assert_eq!(
            super::find_case_insensitive_pair(
                super::EnumInputRef("x"),
                super::EnumPairsRef(&PAIRS)
            ),
            None
        );
    }
    #[test]
    fn helper_error_mentions_allowed_values() {
        let er = super::impl_from_str_for_enum_helper::<V, String>(
            super::EnumInputRef("x"),
            super::EnumPairsRef(&PAIRS),
        )
        .expect_err("4d6330e7");
        assert!(er.contains("Unknown value: x"));
        assert!(er.contains("Allowed values: a, b"));
    }
    #[test]
    fn helper_error_keeps_variant_order_in_allowed_values() {
        let pairs = [("first", V::A), ("second", V::B)];
        let er = super::impl_from_str_for_enum_helper::<V, String>(
            super::EnumInputRef("x"),
            super::EnumPairsRef(&pairs),
        )
        .expect_err("ee52fc8d");
        assert!(er.contains("Allowed values: first, second"));
    }
    #[test]
    fn helper_error_handles_empty_variants() {
        let pairs: [(&str, V); 0] = [];
        let er = super::impl_from_str_for_enum_helper::<V, String>(
            super::EnumInputRef("x"),
            super::EnumPairsRef(&pairs),
        )
        .expect_err("312f79de");
        assert_eq!(er, "Unknown value: x. Allowed values: ");
    }
    #[test]
    fn mk_allowed_values_formats_multiple_variants() {
        assert_eq!(
            super::mk_allowed_values(super::EnumPairsRef(&PAIRS)).as_ref(),
            "a, b"
        );
    }
    #[test]
    fn mk_allowed_values_returns_empty_for_no_variants() {
        let pairs: [(&str, V); 0] = [];
        assert_eq!(
            super::mk_allowed_values(super::EnumPairsRef(&pairs)).as_ref(),
            ""
        );
    }
}
