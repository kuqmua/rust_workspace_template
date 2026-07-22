#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, optml::Optml)]
pub enum PgTypeFilter {
    Eq {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThan {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    Between {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    In {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    Regex,
    Before {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    CurrentDate,
    GreaterThanCurrentDate,
    CurrentTimestamp,
    GreaterThanCurrentTimestamp,
    CurrentTime,
    GreaterThanCurrentTime,
    EqToEncodedStringRepresentation,
    FindRangesWithinGivenRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    FindRangesThatFullyContainTheGivenRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    StrictlyToLeftOfRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    StrictlyToRightOfRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    IncludedLowerBound {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    ExcludedUpperBound {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThanIncludedLowerBound {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThanExcludedUpperBound {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    OverlapWithRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    AdjacentWithRange {
        identifier: macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream,
    },
    RangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFilter for PgTypeFilter {
    fn maybe_generic(
        &self,
    ) -> Option<macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream> {
        match &self {
            Self::Eq { identifier }
            | Self::GreaterThan { identifier }
            | Self::Between { identifier }
            | Self::In { identifier }
            | Self::Before { identifier }
            | Self::FindRangesWithinGivenRange { identifier }
            | Self::FindRangesThatFullyContainTheGivenRange { identifier }
            | Self::StrictlyToLeftOfRange { identifier }
            | Self::StrictlyToRightOfRange { identifier }
            | Self::IncludedLowerBound { identifier }
            | Self::ExcludedUpperBound { identifier }
            | Self::GreaterThanIncludedLowerBound { identifier }
            | Self::GreaterThanExcludedUpperBound { identifier }
            | Self::OverlapWithRange { identifier }
            | Self::AdjacentWithRange { identifier } => Some(identifier.clone()),
            Self::Regex
            | Self::CurrentDate
            | Self::GreaterThanCurrentDate
            | Self::CurrentTimestamp
            | Self::GreaterThanCurrentTimestamp
            | Self::CurrentTime
            | Self::GreaterThanCurrentTime
            | Self::EqToEncodedStringRepresentation
            | Self::RangeLen => None,
        }
    }
    fn prefix_where_self_upper_camel_case(
        &self,
    ) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
        let v = naming::parameter::PgTypeWhereSelfUpperCamelCase::from_display(&self.ucc());
        macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream::from(
            quote::quote! {#v},
        )
    }
    fn ucc(&self) -> &'static dyn naming::DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &naming::EqUpperCamelCase,
            Self::GreaterThan { .. } => &naming::GreaterThanUpperCamelCase,
            Self::Between { .. } => &naming::BetweenUpperCamelCase,
            Self::In { .. } => &naming::InUpperCamelCase,
            Self::Regex => &naming::RegexUpperCamelCase,
            Self::Before { .. } => &naming::BeforeUpperCamelCase,
            Self::CurrentDate => &naming::CurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => &naming::GreaterThanCurrentDateUpperCamelCase,
            Self::CurrentTimestamp => &naming::CurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => &naming::GreaterThanCurrentTimestampUpperCamelCase,
            Self::CurrentTime => &naming::CurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => &naming::GreaterThanCurrentTimeUpperCamelCase,
            Self::EqToEncodedStringRepresentation => {
                &naming::EqToEncodedStringRepresentationUpperCamelCase
            }
            Self::FindRangesWithinGivenRange { .. } => {
                &naming::FindRangesWithinGivenRangeUpperCamelCase
            }
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &naming::FindRangesThatFullyContainTheGivenRangeUpperCamelCase
            }
            Self::StrictlyToLeftOfRange { .. } => &naming::StrictlyToLeftOfRangeUpperCamelCase,
            Self::StrictlyToRightOfRange { .. } => &naming::StrictlyToRightOfRangeUpperCamelCase,
            Self::IncludedLowerBound { .. } => &naming::IncludedLowerBoundUpperCamelCase,
            Self::ExcludedUpperBound { .. } => &naming::ExcludedUpperBoundUpperCamelCase,
            Self::GreaterThanIncludedLowerBound { .. } => {
                &naming::GreaterThanIncludedLowerBoundUpperCamelCase
            }
            Self::GreaterThanExcludedUpperBound { .. } => {
                &naming::GreaterThanExcludedUpperBoundUpperCamelCase
            }
            Self::OverlapWithRange { .. } => &naming::OverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange { .. } => &naming::AdjacentWithRangeUpperCamelCase,
            Self::RangeLen => &naming::RangeLenUpperCamelCase,
        }
    }
}
pub trait PgFilter {
    fn maybe_generic(
        &self,
    ) -> Option<macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream>;
    fn prefix_where_self_upper_camel_case(
        &self,
    ) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream;
    fn ucc(&self) -> &'static dyn naming::DisplayPlusToTokens;
}
