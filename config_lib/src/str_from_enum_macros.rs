#[derive(Debug, Clone, Copy)]
pub struct EnumInputText<'text>(&'text str);

#[derive(Debug, Clone, Copy)]
pub struct EnumVariantNameText<'text>(&'text str);

#[derive(Debug, Clone, Copy)]
pub struct EnumVariantPairs<'pairs, EnumValue>(&'pairs [(EnumVariantNameText<'pairs>, EnumValue)]);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumAllowedValuesText(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumParseErrorMessage(String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EnumParseResult<EnumValue> {
    Found(EnumValue),
    Unknown(EnumParseErrorMessage),
}

impl<'text> From<&'text str> for EnumInputText<'text> {
    fn from(value: &'text str) -> Self {
        Self(value)
    }
}

impl<'text> From<&'text str> for EnumVariantNameText<'text> {
    fn from(value: &'text str) -> Self {
        Self(value)
    }
}

impl<'pairs, EnumValue> From<&'pairs [(EnumVariantNameText<'pairs>, EnumValue)]>
    for EnumVariantPairs<'pairs, EnumValue>
{
    fn from(value: &'pairs [(EnumVariantNameText<'pairs>, EnumValue)]) -> Self {
        Self(value)
    }
}

impl AsRef<str> for EnumAllowedValuesText {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<str> for EnumParseErrorMessage {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[must_use]
pub fn mk_allowed_values<EnumValue>(
    variant_pairs: &EnumVariantPairs<'_, EnumValue>,
) -> EnumAllowedValuesText {
    let allowed_values_capacity = variant_pairs
        .0
        .iter()
        .map(|variant_pair| variant_pair.0.0.len())
        .sum::<usize>()
        .saturating_add(variant_pairs.0.len().saturating_sub(1).saturating_mul(2));
    let mut allowed_values = String::with_capacity(allowed_values_capacity);
    variant_pairs
        .0
        .iter()
        .enumerate()
        .for_each(|(variant_pair_index, variant_pair)| {
            if variant_pair_index != 0 {
                allowed_values.push_str(", ");
            }
            allowed_values.push_str(variant_pair.0.0);
        });
    EnumAllowedValuesText(allowed_values)
}

#[must_use]
pub fn impl_from_str_for_enum_helper<EnumValue>(
    input_text: EnumInputText<'_>,
    variant_pairs: &EnumVariantPairs<'_, EnumValue>,
) -> EnumParseResult<EnumValue>
where
    EnumValue: Copy,
{
    let found_variant = variant_pairs.0.iter().find_map(|variant_pair| {
        input_text
            .0
            .eq_ignore_ascii_case(variant_pair.0.0)
            .then_some(EnumParseResult::Found(variant_pair.1))
    });
    let Some(parse_result) = found_variant else {
        let allowed_values = mk_allowed_values(variant_pairs);
        return EnumParseResult::Unknown(EnumParseErrorMessage(format!(
            "Unknown value: {}. Allowed values: {}",
            input_text.0,
            allowed_values.as_ref()
        )));
    };
    parse_result
}

#[cfg(test)]
mod tests {
    const PAIRS: [(crate::str_from_enum_macros::EnumVariantNameText<'static>, TestEnumValue); 2] = [
        (crate::str_from_enum_macros::EnumVariantNameText("a"), TestEnumValue::Alpha),
        (crate::str_from_enum_macros::EnumVariantNameText("b"), TestEnumValue::Beta),
    ];

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TestEnumValue {
        Alpha,
        Beta,
    }

    fn pairs() -> crate::str_from_enum_macros::EnumVariantPairs<'static, TestEnumValue> {
        crate::str_from_enum_macros::EnumVariantPairs::from(PAIRS.as_slice())
    }

    #[test]
    fn helper_parses_values_case_insensitively() -> Result<(), String> {
        let variant_pairs = pairs();
        let upper_result = crate::str_from_enum_macros::impl_from_str_for_enum_helper(
            crate::str_from_enum_macros::EnumInputText::from("A"),
            &variant_pairs,
        );
        let lower_result = crate::str_from_enum_macros::impl_from_str_for_enum_helper(
            crate::str_from_enum_macros::EnumInputText::from("b"),
            &variant_pairs,
        );
        if upper_result == crate::str_from_enum_macros::EnumParseResult::Found(TestEnumValue::Alpha)
            && lower_result
                == crate::str_from_enum_macros::EnumParseResult::Found(TestEnumValue::Beta)
        {
            return Ok(());
        }
        Err(format!("{upper_result:?} {lower_result:?}"))
    }

    #[test]
    fn helper_error_mentions_allowed_values() -> Result<(), String> {
        let variant_pairs = pairs();
        let crate::str_from_enum_macros::EnumParseResult::Unknown(error_message) =
            crate::str_from_enum_macros::impl_from_str_for_enum_helper(
                crate::str_from_enum_macros::EnumInputText::from("x"),
                &variant_pairs,
            )
        else {
            return Err("expected unknown enum parse result".to_owned());
        };
        if error_message.as_ref() == "Unknown value: x. Allowed values: a, b" {
            return Ok(());
        }
        Err(error_message.as_ref().to_owned())
    }

    #[test]
    fn helper_error_keeps_variant_order_in_allowed_values() -> Result<(), String> {
        let variant_pairs = crate::str_from_enum_macros::EnumVariantPairs::from(
            [
                (crate::str_from_enum_macros::EnumVariantNameText("first"), TestEnumValue::Alpha),
                (crate::str_from_enum_macros::EnumVariantNameText("second"), TestEnumValue::Beta),
            ]
            .as_slice(),
        );
        let crate::str_from_enum_macros::EnumParseResult::Unknown(error_message) =
            crate::str_from_enum_macros::impl_from_str_for_enum_helper(
                crate::str_from_enum_macros::EnumInputText::from("x"),
                &variant_pairs,
            )
        else {
            return Err("expected unknown enum parse result".to_owned());
        };
        if error_message.as_ref() == "Unknown value: x. Allowed values: first, second" {
            return Ok(());
        }
        Err(error_message.as_ref().to_owned())
    }

    #[test]
    fn helper_error_handles_empty_variants() -> Result<(), String> {
        let empty_pairs = crate::str_from_enum_macros::EnumVariantPairs::from(<&[(
            crate::str_from_enum_macros::EnumVariantNameText<'static>,
            TestEnumValue,
        )]>::default(
        ));
        let crate::str_from_enum_macros::EnumParseResult::Unknown(error_message) =
            crate::str_from_enum_macros::impl_from_str_for_enum_helper(
                crate::str_from_enum_macros::EnumInputText::from("x"),
                &empty_pairs,
            )
        else {
            return Err("expected unknown enum parse result".to_owned());
        };
        if error_message.as_ref() == "Unknown value: x. Allowed values: " {
            return Ok(());
        }
        Err(error_message.as_ref().to_owned())
    }

    #[test]
    fn mk_allowed_values_returns_empty_for_no_variants() -> Result<(), String> {
        let empty_pairs = crate::str_from_enum_macros::EnumVariantPairs::from(<&[(
            crate::str_from_enum_macros::EnumVariantNameText<'static>,
            TestEnumValue,
        )]>::default(
        ));
        let allowed_values = crate::str_from_enum_macros::mk_allowed_values(&empty_pairs);
        if allowed_values.as_ref().is_empty() {
            return Ok(());
        }
        Err(allowed_values.as_ref().to_owned())
    }
}
