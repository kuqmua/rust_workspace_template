#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequiredHeaderValidation {
    Missing,
    PresentNonText,
    PresentText,
    PresentTextParseError,
    PresentTextParsed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequiredHeaderValueResult {
    Missing,
    Present,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequiredHeaderStringResult {
    Missing,
    Present,
    ToTextConversionFailed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RequiredHeaderParsedResult {
    Missing,
    ParseFailed,
    Parsed,
    ToTextConversionFailed,
}

#[must_use]
pub const fn get_required_header_value(
    validation: RequiredHeaderValidation,
) -> RequiredHeaderValueResult {
    match validation {
        RequiredHeaderValidation::Missing => RequiredHeaderValueResult::Missing,
        RequiredHeaderValidation::PresentNonText
        | RequiredHeaderValidation::PresentText
        | RequiredHeaderValidation::PresentTextParseError
        | RequiredHeaderValidation::PresentTextParsed => RequiredHeaderValueResult::Present,
    }
}

#[must_use]
pub const fn get_required_header_string(
    validation: RequiredHeaderValidation,
) -> RequiredHeaderStringResult {
    match validation {
        RequiredHeaderValidation::Missing => RequiredHeaderStringResult::Missing,
        RequiredHeaderValidation::PresentNonText => {
            RequiredHeaderStringResult::ToTextConversionFailed
        }
        RequiredHeaderValidation::PresentText
        | RequiredHeaderValidation::PresentTextParseError
        | RequiredHeaderValidation::PresentTextParsed => RequiredHeaderStringResult::Present,
    }
}

#[must_use]
pub const fn get_required_header_parsed(
    validation: RequiredHeaderValidation,
) -> RequiredHeaderParsedResult {
    match validation {
        RequiredHeaderValidation::Missing => RequiredHeaderParsedResult::Missing,
        RequiredHeaderValidation::PresentNonText => {
            RequiredHeaderParsedResult::ToTextConversionFailed
        }
        RequiredHeaderValidation::PresentTextParseError => RequiredHeaderParsedResult::ParseFailed,
        RequiredHeaderValidation::PresentText | RequiredHeaderValidation::PresentTextParsed => {
            RequiredHeaderParsedResult::Parsed
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_required_header_value_returns_present_when_header_exists() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_value(
            crate::hdr_val::RequiredHeaderValidation::PresentText,
        );
        if result == crate::hdr_val::RequiredHeaderValueResult::Present {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_value_returns_missing_when_header_is_absent() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_value(
            crate::hdr_val::RequiredHeaderValidation::Missing,
        );
        if result == crate::hdr_val::RequiredHeaderValueResult::Missing {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_string_returns_present_for_text_header() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_string(
            crate::hdr_val::RequiredHeaderValidation::PresentText,
        );
        if result == crate::hdr_val::RequiredHeaderStringResult::Present {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_string_returns_missing_when_header_is_absent() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_string(
            crate::hdr_val::RequiredHeaderValidation::Missing,
        );
        if result == crate::hdr_val::RequiredHeaderStringResult::Missing {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_string_rejects_non_text_header() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_string(
            crate::hdr_val::RequiredHeaderValidation::PresentNonText,
        );
        if result == crate::hdr_val::RequiredHeaderStringResult::ToTextConversionFailed {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_parsed_returns_parsed_for_valid_text_header() -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_parsed(
            crate::hdr_val::RequiredHeaderValidation::PresentTextParsed,
        );
        if result == crate::hdr_val::RequiredHeaderParsedResult::Parsed {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_parsed_returns_parse_error_for_invalid_text_header() -> Result<(), String>
    {
        let result = crate::hdr_val::get_required_header_parsed(
            crate::hdr_val::RequiredHeaderValidation::PresentTextParseError,
        );
        if result == crate::hdr_val::RequiredHeaderParsedResult::ParseFailed {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_parsed_returns_missing_without_parse_for_absent_header()
    -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_parsed(
            crate::hdr_val::RequiredHeaderValidation::Missing,
        );
        if result == crate::hdr_val::RequiredHeaderParsedResult::Missing {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }

    #[test]
    fn get_required_header_parsed_returns_to_text_error_without_parse_for_non_text_header()
    -> Result<(), String> {
        let result = crate::hdr_val::get_required_header_parsed(
            crate::hdr_val::RequiredHeaderValidation::PresentNonText,
        );
        if result == crate::hdr_val::RequiredHeaderParsedResult::ToTextConversionFailed {
            return Ok(());
        }
        Err(format!("{result:?}"))
    }
}
