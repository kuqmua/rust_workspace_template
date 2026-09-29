#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub struct ToolAnsiChars<'text> {
    std_str_chars: crate::std_str_chars::StdStrChars<'text>,
    tool_ansi_escape_state: crate::tool_ansi_escape_state::ToolAnsiEscapeState,
}

impl<'text> From<crate::tool_ansi_text_ref::ToolAnsiTextRef<'text>> for ToolAnsiChars<'text> {
    fn from(value: crate::tool_ansi_text_ref::ToolAnsiTextRef<'text>) -> Self {
        Self {
            std_str_chars: crate::std_str_chars::StdStrChars::from(value.get().chars()),
            tool_ansi_escape_state: crate::tool_ansi_escape_state::ToolAnsiEscapeState::Text,
        }
    }
}

impl From<ToolAnsiChars<'_>> for String {
    fn from(value: ToolAnsiChars<'_>) -> Self {
        let mut output = Self::with_capacity(value.std_str_chars.as_str().len());
        output.extend(value);
        output
    }
}

impl Iterator for ToolAnsiChars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.std_str_chars.find(
            |character| {
                match (self.tool_ansi_escape_state, *character) {
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::Text, '\u{1b}') => {
                        self.tool_ansi_escape_state =
                            crate::tool_ansi_escape_state::ToolAnsiEscapeState::Escape;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::Text, _) => true,
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::Escape, '[') => {
                        self.tool_ansi_escape_state =
                            crate::tool_ansi_escape_state::ToolAnsiEscapeState::ControlSequence;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::Escape, ']') => {
                        self.tool_ansi_escape_state = crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommand;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::ControlSequence, final_byte)
                        if ('@'..='~').contains(&final_byte) =>
                    {
                        self.tool_ansi_escape_state =
                            crate::tool_ansi_escape_state::ToolAnsiEscapeState::Text;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommand, '\u{1b}') => {
                        self.tool_ansi_escape_state = crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommandEscape;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::Escape, _)
                    | (crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommand, '\u{7}')
                    | (crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommandEscape, '\\') => {
                        self.tool_ansi_escape_state =
                            crate::tool_ansi_escape_state::ToolAnsiEscapeState::Text;
                        false
                    }
                    (crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommandEscape, _) => {
                        self.tool_ansi_escape_state = crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommand;
                        false
                    }
                    (
                        crate::tool_ansi_escape_state::ToolAnsiEscapeState::ControlSequence
                        | crate::tool_ansi_escape_state::ToolAnsiEscapeState::OperatingSystemCommand,
                        _,
                    ) => false,
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shared_ansi_filter_preserves_plain_unicode_and_escape_semantics() {
        assert!(
            [
                (constants_str::VALUE_22233BC3, constants_str::VALUE_14D8B1DB),
                (constants_str::VALUE_A116C9ED, constants_str::VALUE_A116C9ED),
                (constants_str::VALUE_EC39432A, constants_str::VALUE_4E9A9107),
                (
                    constants_str::MEMORY_USAGE_SUMMARY,
                    constants_str::MEMORY_USAGE_SUMMARY
                ),
                (constants_str::EMPTY, constants_str::EMPTY),
                (constants_str::NON_ASCII_U_E9, constants_str::NON_ASCII_U_E9),
            ]
            .into_iter()
            .all(|(input, expected)| {
                let tool_ansi_text_ref = crate::tool_ansi_text_ref::ToolAnsiTextRef::from(input);
                let output = String::from(super::ToolAnsiChars::from(tool_ansi_text_ref));
                output == expected && output.capacity() >= tool_ansi_text_ref.get().len()
            })
        );
    }

    #[test]
    fn test_ansi_filter_preserves_text_after_csi_and_osc_sequences() {
        let cases = [
            (
                constants_str::TOOL_ANSI_CSI_ERASE_INPUT,
                constants_str::TOOL_ANSI_CSI_ERASE_OUTPUT,
            ),
            (
                constants_str::TOOL_ANSI_OSC_BEL_INPUT,
                constants_str::TOOL_ANSI_OSC_BEL_OUTPUT,
            ),
            (
                constants_str::TOOL_ANSI_OSC_ST_INPUT,
                constants_str::TOOL_ANSI_OSC_ST_OUTPUT,
            ),
        ];
        assert!(cases.into_iter().all(|(input, expected)| {
            String::from(super::ToolAnsiChars::from(
                crate::tool_ansi_text_ref::ToolAnsiTextRef::from(input),
            )) == expected
        }));
    }
}
