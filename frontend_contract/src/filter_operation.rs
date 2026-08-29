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
    pub const fn value_shape(self) -> crate::filter_value_shape::FilterValueShape {
        match self {
            Self::Between => crate::filter_value_shape::FilterValueShape::Range,
            Self::CurrentDate
            | Self::CurrentTime
            | Self::CurrentTimestamp
            | Self::GreaterThanCurrentDate
            | Self::GreaterThanCurrentTime
            | Self::GreaterThanCurrentTimestamp => {
                crate::filter_value_shape::FilterValueShape::None
            }
            Self::EqToEncodedStringRepresentation => {
                crate::filter_value_shape::FilterValueShape::EncodedText
            }
            Self::In => crate::filter_value_shape::FilterValueShape::List,
            Self::Regex => crate::filter_value_shape::FilterValueShape::Regex,
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
            | Self::StrictlyToRightOfRange => crate::filter_value_shape::FilterValueShape::Scalar,
        }
    }
}
