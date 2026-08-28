pub(crate) fn deserialize_bounded_map<'de, Map, Key, Value, Values, Insert, const MAX: usize>(
    mut map: Map,
    mut values: Values,
    mut insert: Insert,
) -> Result<Values, Map::Error>
where
    Map: serde::de::MapAccess<'de>,
    Key: serde::Deserialize<'de>,
    Value: serde::Deserialize<'de>,
    Insert: FnMut(&mut Values, Key, Value) -> Result<(), crate::domain_types::BoundedValueError>,
{
    let mut entry_count = constants_usize::ZERO;
    loop {
        if entry_count == MAX {
            return map.next_key::<serde::de::IgnoredAny>()?.map_or_else(
                || Ok(values),
                |_ignored| {
                    Err(serde::de::Error::custom(
                        crate::domain_types::BoundedValueError::AboveMax {
                            actual: crate::domain_types::BoundedLen::from(
                                MAX.saturating_add(constants_usize::ONE),
                            ),
                            max: crate::domain_types::BoundedLen::from(MAX),
                        },
                    ))
                },
            );
        }
        let Some(key) = map.next_key()? else {
            return Ok(values);
        };
        let value = map.next_value()?;
        insert(&mut values, key, value).map_err(serde::de::Error::custom)?;
        entry_count = entry_count.saturating_add(constants_usize::ONE);
    }
}
