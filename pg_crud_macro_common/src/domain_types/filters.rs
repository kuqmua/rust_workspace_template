#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Clone,
    strum_macros::Display,
    strum_macros::EnumIter,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PgTypeFilter {
    Eq {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThan {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    Between {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    In {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    Regex,
    Before {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    CurrentDate,
    GreaterThanCurrentDate,
    CurrentTimestamp,
    GreaterThanCurrentTimestamp,
    CurrentTime,
    GreaterThanCurrentTime,
    EqToEncodedStringRepresentation,
    FindRangesWithinGivenRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    FindRangesThatFullyContainTheGivenRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    StrictlyToLeftOfRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    StrictlyToRightOfRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    IncludedLowerBound {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    ExcludedUpperBound {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThanIncludedLowerBound {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    GreaterThanExcludedUpperBound {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    OverlapWithRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    AdjacentWithRange {
        identifier:
            macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream,
    },
    RangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFilter for PgTypeFilter {
    fn maybe_generic(
        &self,
    ) -> Option<macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream>
    {
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
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
        let v = naming::domain_types::parameter::PgTypeWhereSelfUpperCamelCase::from_display(
            &self.ucc(),
        );
        macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
            quote::quote! {#v},
        )
    }
    fn ucc(&self) -> &'static dyn naming::domain_types::DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &naming::domain_types::EqUpperCamelCase,
            Self::GreaterThan { .. } => &naming::domain_types::GreaterThanUpperCamelCase,
            Self::Between { .. } => &naming::domain_types::BetweenUpperCamelCase,
            Self::In { .. } => &naming::domain_types::InUpperCamelCase,
            Self::Regex => &naming::domain_types::RegexUpperCamelCase,
            Self::Before { .. } => &naming::domain_types::BeforeUpperCamelCase,
            Self::CurrentDate => &naming::domain_types::CurrentDateUpperCamelCase,
            Self::GreaterThanCurrentDate => {
                &naming::domain_types::GreaterThanCurrentDateUpperCamelCase
            }
            Self::CurrentTimestamp => &naming::domain_types::CurrentTimestampUpperCamelCase,
            Self::GreaterThanCurrentTimestamp => {
                &naming::domain_types::GreaterThanCurrentTimestampUpperCamelCase
            }
            Self::CurrentTime => &naming::domain_types::CurrentTimeUpperCamelCase,
            Self::GreaterThanCurrentTime => {
                &naming::domain_types::GreaterThanCurrentTimeUpperCamelCase
            }
            Self::EqToEncodedStringRepresentation => {
                &naming::domain_types::EqToEncodedStringRepresentationUpperCamelCase
            }
            Self::FindRangesWithinGivenRange { .. } => {
                &naming::domain_types::FindRangesWithinGivenRangeUpperCamelCase
            }
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &naming::domain_types::FindRangesThatFullyContainTheGivenRangeUpperCamelCase
            }
            Self::StrictlyToLeftOfRange { .. } => {
                &naming::domain_types::StrictlyToLeftOfRangeUpperCamelCase
            }
            Self::StrictlyToRightOfRange { .. } => {
                &naming::domain_types::StrictlyToRightOfRangeUpperCamelCase
            }
            Self::IncludedLowerBound { .. } => {
                &naming::domain_types::IncludedLowerBoundUpperCamelCase
            }
            Self::ExcludedUpperBound { .. } => {
                &naming::domain_types::ExcludedUpperBoundUpperCamelCase
            }
            Self::GreaterThanIncludedLowerBound { .. } => {
                &naming::domain_types::GreaterThanIncludedLowerBoundUpperCamelCase
            }
            Self::GreaterThanExcludedUpperBound { .. } => {
                &naming::domain_types::GreaterThanExcludedUpperBoundUpperCamelCase
            }
            Self::OverlapWithRange { .. } => &naming::domain_types::OverlapWithRangeUpperCamelCase,
            Self::AdjacentWithRange { .. } => {
                &naming::domain_types::AdjacentWithRangeUpperCamelCase
            }
            Self::RangeLen => &naming::domain_types::RangeLenUpperCamelCase,
        }
    }
}
pub trait PgFilter {
    fn maybe_generic(
        &self,
    ) -> Option<macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream>;
    fn prefix_where_self_upper_camel_case(
        &self,
    ) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream;
    fn ucc(&self) -> &'static dyn naming::domain_types::DisplayPlusToTokens;
}
