pub(crate) fn memusage_heap_value(
    clean_ansi_text: &crate::clean_ansi_text::CleanAnsiText,
    memusage_key: crate::memusage_key::MemusageKey,
) -> crate::memusage_value_ref::MemusageValueRef<'_> {
    crate::memusage_summary_text::memusage_summary_text(clean_ansi_text)
        .and_then(|summary| {
            summary.get().lines().find_map(|line| {
                line.split_once(memusage_key.get())
                    .map(|(_, tail)| tail.trim())
            })
        })
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or_else(
            || crate::memusage_value_ref::MemusageValueRef::from(constants_str::UNAVAILABLE),
            crate::memusage_value_ref::MemusageValueRef::from,
        )
}
