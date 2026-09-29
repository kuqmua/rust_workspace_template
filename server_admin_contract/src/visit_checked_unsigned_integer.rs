pub(crate) fn visit_checked_unsigned_integer<'de, Visitor, Integer, Error>(
    visitor: Visitor,
    integer: Integer,
) -> Result<Visitor::Value, Error>
where
    Visitor: serde::de::Visitor<'de>,
    Integer: TryInto<u64>,
    Integer::Error: std::fmt::Display,
    Error: serde::de::Error,
{
    visitor.visit_u64(integer.try_into().map_err(serde::de::Error::custom)?)
}
