#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, optml::Optml)]
pub enum PgTypeFlt {
    Eq {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    GreaterThan {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    Btwn {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    In {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    Rgx,
    Before {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    CrntDate,
    GreaterThanCrntDate,
    CrntTimestamp,
    GreaterThanCrntTimestamp,
    CrntTime,
    GreaterThanCrntTime,
    EqToEncodedStringRepresentation,
    FindRangesWithinGivenRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    FindRangesThatFullyContainTheGivenRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    StrictlyToLeftOfRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    StrictlyToRightOfRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    IncludedLowerBound {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    ExcludedUpperBound {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    GreaterThanIncludedLowerBound {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    GreaterThanExcludedUpperBound {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    OverlapWithRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    AdjacentWithRange {
        ident: macros_helpers::generated_rust_ts::GeneratedRustTs,
    },
    RangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFlt for PgTypeFlt {
    fn mb_generic(&self) -> Option<macros_helpers::generated_rust_ts::GeneratedRustTs> {
        match &self {
            Self::Eq { ident }
            | Self::GreaterThan { ident }
            | Self::Btwn { ident }
            | Self::In { ident }
            | Self::Before { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident }
            | Self::IncludedLowerBound { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::AdjacentWithRange { ident } => Some(ident.clone()),
            Self::Rgx
            | Self::CrntDate
            | Self::GreaterThanCrntDate
            | Self::CrntTimestamp
            | Self::GreaterThanCrntTimestamp
            | Self::CrntTime
            | Self::GreaterThanCrntTime
            | Self::EqToEncodedStringRepresentation
            | Self::RangeLen => None,
        }
    }
    fn prefix_wh_self_ucc(&self) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
        let v = naming::prm::PgTypeWhSelfUcc::from_display(&self.ucc());
        macros_helpers::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#v})
    }
    fn ucc(&self) -> &'static dyn naming::DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &naming::EqUcc,
            Self::GreaterThan { .. } => &naming::GreaterThanUcc,
            Self::Btwn { .. } => &naming::BtwnUcc,
            Self::In { .. } => &naming::InUcc,
            Self::Rgx => &naming::RgxUcc,
            Self::Before { .. } => &naming::BeforeUcc,
            Self::CrntDate => &naming::CrntDateUcc,
            Self::GreaterThanCrntDate => &naming::GreaterThanCrntDateUcc,
            Self::CrntTimestamp => &naming::CrntTimestampUcc,
            Self::GreaterThanCrntTimestamp => &naming::GreaterThanCrntTimestampUcc,
            Self::CrntTime => &naming::CrntTimeUcc,
            Self::GreaterThanCrntTime => &naming::GreaterThanCrntTimeUcc,
            Self::EqToEncodedStringRepresentation => &naming::EqToEncodedStringRepresentationUcc,
            Self::FindRangesWithinGivenRange { .. } => &naming::FindRangesWithinGivenRangeUcc,
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &naming::FindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::StrictlyToLeftOfRange { .. } => &naming::StrictlyToLeftOfRangeUcc,
            Self::StrictlyToRightOfRange { .. } => &naming::StrictlyToRightOfRangeUcc,
            Self::IncludedLowerBound { .. } => &naming::IncludedLowerBoundUcc,
            Self::ExcludedUpperBound { .. } => &naming::ExcludedUpperBoundUcc,
            Self::GreaterThanIncludedLowerBound { .. } => &naming::GreaterThanIncludedLowerBoundUcc,
            Self::GreaterThanExcludedUpperBound { .. } => &naming::GreaterThanExcludedUpperBoundUcc,
            Self::OverlapWithRange { .. } => &naming::OverlapWithRangeUcc,
            Self::AdjacentWithRange { .. } => &naming::AdjacentWithRangeUcc,
            Self::RangeLen => &naming::RangeLenUcc,
        }
    }
}
pub trait PgFlt {
    fn mb_generic(&self) -> Option<macros_helpers::generated_rust_ts::GeneratedRustTs>;
    fn prefix_wh_self_ucc(&self) -> macros_helpers::generated_rust_ts::GeneratedRustTs;
    fn ucc(&self) -> &'static dyn naming::DisplayPlusToTokens;
}
