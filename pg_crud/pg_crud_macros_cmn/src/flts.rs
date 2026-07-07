type Ts2 = proc_macro2::TokenStream;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, strum_macros::Display, strum_macros::EnumIter, optml::Optml)]
pub enum PgTypeFlt {
    Eq { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    Btwn { ident: Ts2 },
    In { ident: Ts2 },
    Rgx,
    Before { ident: Ts2 },
    CrntDate,
    GreaterThanCrntDate,
    CrntTimestamp,
    GreaterThanCrntTimestamp,
    CrntTime,
    GreaterThanCrntTime,
    EqToEncodedStringRepresentation,
    FindRangesWithinGivenRange { ident: Ts2 },
    FindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    StrictlyToLeftOfRange { ident: Ts2 },
    StrictlyToRightOfRange { ident: Ts2 },
    IncludedLowerBound { ident: Ts2 },
    ExcludedUpperBound { ident: Ts2 },
    GreaterThanIncludedLowerBound { ident: Ts2 },
    GreaterThanExcludedUpperBound { ident: Ts2 },
    OverlapWithRange { ident: Ts2 },
    AdjacentWithRange { ident: Ts2 },
    RangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFlt for PgTypeFlt {
    fn mb_generic(&self) -> Option<Ts2> {
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
    fn prefix_wh_self_ucc(&self) -> Ts2 {
        let v = naming::prm::PgTypeWhSelfUcc::from_display(&self.ucc());
        quote::quote! {#v}
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
    fn mb_generic(&self) -> Option<Ts2>;
    fn prefix_wh_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn naming::DisplayPlusToTokens;
}
