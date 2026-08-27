use super::FilterValueShape;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperation {
    AdjacentWithRange,
    Before,
    Between,
    CurrentDate,
    CurrentTime,
    CurrentTimestamp,
    Eq,
    EqToEncodedStringRepresentation,
    ExcludedUpperBound,
    FindRangesThatFullyContainTheGivenRange,
    FindRangesWithinGivenRange,
    GreaterThan,
    GreaterThanCurrentDate,
    GreaterThanCurrentTime,
    GreaterThanCurrentTimestamp,
    GreaterThanExcludedUpperBound,
    GreaterThanIncludedLowerBound,
    In,
    IncludedLowerBound,
    OverlapWithRange,
    RangeLen,
    Regex,
    StrictlyToLeftOfRange,
    StrictlyToRightOfRange,
}
impl FilterOperation {
    #[must_use]
    pub const fn value_shape(self) -> FilterValueShape {
        match self {
            Self::Between => FilterValueShape::Range,
            Self::CurrentDate
            | Self::CurrentTime
            | Self::CurrentTimestamp
            | Self::GreaterThanCurrentDate
            | Self::GreaterThanCurrentTime
            | Self::GreaterThanCurrentTimestamp => FilterValueShape::None,
            Self::EqToEncodedStringRepresentation => FilterValueShape::EncodedText,
            Self::In => FilterValueShape::List,
            Self::Regex => FilterValueShape::Regex,
            Self::AdjacentWithRange
            | Self::Before
            | Self::Eq
            | Self::ExcludedUpperBound
            | Self::FindRangesThatFullyContainTheGivenRange
            | Self::FindRangesWithinGivenRange
            | Self::GreaterThan
            | Self::GreaterThanExcludedUpperBound
            | Self::GreaterThanIncludedLowerBound
            | Self::IncludedLowerBound
            | Self::OverlapWithRange
            | Self::RangeLen
            | Self::StrictlyToLeftOfRange
            | Self::StrictlyToRightOfRange => FilterValueShape::Scalar,
        }
    }
}
