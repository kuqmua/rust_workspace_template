use super::{CleanAnsiText, MemusageKey, MemusageValueRef};

pub(crate) fn memusage_heap_value(text: &CleanAnsiText, key: MemusageKey) -> MemusageValueRef<'_> {
    text.0
        .as_str()
        .lines()
        .find_map(|line| line.split_once(key.get()).map(|(_, tail)| tail.trim()))
        .and_then(|tail| tail.split([',', ' ']).find(|part| !part.is_empty()))
        .map_or_else(
            || MemusageValueRef::from(constants_str::UNAVAILABLE),
            MemusageValueRef,
        )
}
