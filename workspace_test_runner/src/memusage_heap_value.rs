pub(crate) fn memusage_heap_value(
    clean_ansi_text: &crate::clean_ansi_text::CleanAnsiText,
    memusage_key: crate::memusage_key::MemusageKey,
) -> crate::memusage_value_ref::MemusageValueRef<'_> {
    clean_ansi_text
        .as_ref()
        .lines()
        .find_map(|line| {
            line.split_once(memusage_key.get())
                .map(|(_, tail)| tail.trim())
        })
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or_else(
            || crate::memusage_value_ref::MemusageValueRef::from(constants_str::UNAVAILABLE),
            crate::memusage_value_ref::MemusageValueRef::from,
        )
}
