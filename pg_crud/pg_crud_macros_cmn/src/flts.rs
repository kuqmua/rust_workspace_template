use naming::prm::PgTypeWhSelfUcc;
use naming::{
    AdjacentWithRangeUcc, BeforeUcc, BtwnUcc, CrntDateUcc, CrntTimeUcc, CrntTimestampUcc,
    DimOneAdjacentWithRangeUcc, DimOneBeforeUcc, DimOneBtwnUcc, DimOneCrntDateUcc,
    DimOneCrntTimeUcc, DimOneCrntTimestampUcc, DimOneEqToEncodedStringRepresentationUcc,
    DimOneEqUcc, DimOneExcludedUpperBoundUcc, DimOneFindRangesThatFullyContainTheGivenRangeUcc,
    DimOneFindRangesWithinGivenRangeUcc, DimOneGreaterThanCrntDateUcc,
    DimOneGreaterThanCrntTimeUcc, DimOneGreaterThanCrntTimestampUcc,
    DimOneGreaterThanExcludedUpperBoundUcc, DimOneGreaterThanIncludedLowerBoundUcc,
    DimOneGreaterThanUcc, DimOneInUcc, DimOneIncludedLowerBoundUcc, DimOneLenEqUcc,
    DimOneLenGreaterThanUcc, DimOneOverlapWithRangeUcc, DimOneRangeLenUcc, DimOneRgxUcc,
    DimOneStrictlyToLeftOfRangeUcc, DimOneStrictlyToRightOfRangeUcc, DisplayPlusToTokens,
    EqToEncodedStringRepresentationUcc, EqUcc, ExcludedUpperBoundUcc,
    FindRangesThatFullyContainTheGivenRangeUcc, FindRangesWithinGivenRangeUcc,
    GreaterThanCrntDateUcc, GreaterThanCrntTimeUcc, GreaterThanCrntTimestampUcc,
    GreaterThanExcludedUpperBoundUcc, GreaterThanIncludedLowerBoundUcc, GreaterThanUcc, InUcc,
    IncludedLowerBoundUcc, OverlapWithRangeUcc, RangeLenUcc, RgxUcc, StrictlyToLeftOfRangeUcc,
    StrictlyToRightOfRangeUcc,
};
use optml::Optml;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
use strum_macros::{Display, EnumIter};
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, Display, EnumIter, Optml)]
pub enum PgTypeFlt {
    Eq { ident: Ts2 },
    DimOneEq { ident: Ts2 },
    GreaterThan { ident: Ts2 },
    DimOneGreaterThan { ident: Ts2 },
    Btwn { ident: Ts2 },
    DimOneBtwn { ident: Ts2 },
    In { ident: Ts2 },
    DimOneIn { ident: Ts2 },
    Rgx,
    DimOneRgx,
    Before { ident: Ts2 },
    DimOneBefore { ident: Ts2 },
    CrntDate,
    DimOneCrntDate,
    GreaterThanCrntDate,
    DimOneGreaterThanCrntDate,
    CrntTimestamp,
    DimOneCrntTimestamp,
    GreaterThanCrntTimestamp,
    DimOneGreaterThanCrntTimestamp,
    CrntTime,
    DimOneCrntTime,
    GreaterThanCrntTime,
    DimOneGreaterThanCrntTime,
    DimOneLenEq,
    DimOneLenGreaterThan,
    EqToEncodedStringRepresentation,
    DimOneEqToEncodedStringRepresentation,
    FindRangesWithinGivenRange { ident: Ts2 },
    DimOneFindRangesWithinGivenRange { ident: Ts2 },
    FindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    DimOneFindRangesThatFullyContainTheGivenRange { ident: Ts2 },
    StrictlyToLeftOfRange { ident: Ts2 },
    DimOneStrictlyToLeftOfRange { ident: Ts2 },
    StrictlyToRightOfRange { ident: Ts2 },
    DimOneStrictlyToRightOfRange { ident: Ts2 },
    IncludedLowerBound { ident: Ts2 },
    DimOneIncludedLowerBound { ident: Ts2 },
    ExcludedUpperBound { ident: Ts2 },
    DimOneExcludedUpperBound { ident: Ts2 },
    GreaterThanIncludedLowerBound { ident: Ts2 },
    DimOneGreaterThanIncludedLowerBound { ident: Ts2 },
    GreaterThanExcludedUpperBound { ident: Ts2 },
    DimOneGreaterThanExcludedUpperBound { ident: Ts2 },
    OverlapWithRange { ident: Ts2 },
    DimOneOverlapWithRange { ident: Ts2 },
    AdjacentWithRange { ident: Ts2 },
    DimOneAdjacentWithRange { ident: Ts2 },
    RangeLen,
    DimOneRangeLen,
    //BitVecPositionEq,//currently deactivated
}
impl PgFlt for PgTypeFlt {
    fn mb_generic(&self) -> Option<Ts2> {
        match &self {
            Self::Eq { ident }
            | Self::DimOneEq { ident }
            | Self::GreaterThan { ident }
            | Self::DimOneGreaterThan { ident }
            | Self::Btwn { ident }
            | Self::DimOneBtwn { ident }
            | Self::In { ident }
            | Self::DimOneIn { ident }
            | Self::Before { ident }
            | Self::DimOneBefore { ident }
            | Self::FindRangesWithinGivenRange { ident }
            | Self::DimOneFindRangesWithinGivenRange { ident }
            | Self::FindRangesThatFullyContainTheGivenRange { ident }
            | Self::DimOneFindRangesThatFullyContainTheGivenRange { ident }
            | Self::StrictlyToLeftOfRange { ident }
            | Self::DimOneStrictlyToLeftOfRange { ident }
            | Self::StrictlyToRightOfRange { ident }
            | Self::DimOneStrictlyToRightOfRange { ident }
            | Self::IncludedLowerBound { ident }
            | Self::DimOneIncludedLowerBound { ident }
            | Self::ExcludedUpperBound { ident }
            | Self::DimOneExcludedUpperBound { ident }
            | Self::GreaterThanIncludedLowerBound { ident }
            | Self::DimOneGreaterThanIncludedLowerBound { ident }
            | Self::GreaterThanExcludedUpperBound { ident }
            | Self::DimOneGreaterThanExcludedUpperBound { ident }
            | Self::OverlapWithRange { ident }
            | Self::DimOneOverlapWithRange { ident }
            | Self::AdjacentWithRange { ident }
            | Self::DimOneAdjacentWithRange { ident } => Some(ident.clone()),
            Self::Rgx
            | Self::DimOneRgx
            | Self::CrntDate
            | Self::DimOneCrntDate
            | Self::GreaterThanCrntDate
            | Self::DimOneGreaterThanCrntDate
            | Self::CrntTimestamp
            | Self::DimOneCrntTimestamp
            | Self::GreaterThanCrntTimestamp
            | Self::DimOneGreaterThanCrntTimestamp
            | Self::CrntTime
            | Self::DimOneCrntTime
            | Self::GreaterThanCrntTime
            | Self::DimOneGreaterThanCrntTime
            | Self::DimOneLenEq
            | Self::DimOneLenGreaterThan
            | Self::EqToEncodedStringRepresentation
            | Self::DimOneEqToEncodedStringRepresentation
            | Self::RangeLen
            | Self::DimOneRangeLen => None,
        }
    }
    fn prefix_wh_self_ucc(&self) -> Ts2 {
        let v = PgTypeWhSelfUcc::from_display(&self.ucc());
        quote! {#v}
    }
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens {
        match &self {
            Self::Eq { .. } => &EqUcc,
            Self::DimOneEq { .. } => &DimOneEqUcc,
            Self::GreaterThan { .. } => &GreaterThanUcc,
            Self::DimOneGreaterThan { .. } => &DimOneGreaterThanUcc,
            Self::Btwn { .. } => &BtwnUcc,
            Self::DimOneBtwn { .. } => &DimOneBtwnUcc,
            Self::In { .. } => &InUcc,
            Self::DimOneIn { .. } => &DimOneInUcc,
            Self::Rgx => &RgxUcc,
            Self::DimOneRgx => &DimOneRgxUcc,
            Self::Before { .. } => &BeforeUcc,
            Self::DimOneBefore { .. } => &DimOneBeforeUcc,
            Self::CrntDate => &CrntDateUcc,
            Self::DimOneCrntDate => &DimOneCrntDateUcc,
            Self::GreaterThanCrntDate => &GreaterThanCrntDateUcc,
            Self::DimOneGreaterThanCrntDate => &DimOneGreaterThanCrntDateUcc,
            Self::CrntTimestamp => &CrntTimestampUcc,
            Self::DimOneCrntTimestamp => &DimOneCrntTimestampUcc,
            Self::GreaterThanCrntTimestamp => &GreaterThanCrntTimestampUcc,
            Self::DimOneGreaterThanCrntTimestamp => &DimOneGreaterThanCrntTimestampUcc,
            Self::CrntTime => &CrntTimeUcc,
            Self::DimOneCrntTime => &DimOneCrntTimeUcc,
            Self::GreaterThanCrntTime => &GreaterThanCrntTimeUcc,
            Self::DimOneGreaterThanCrntTime => &DimOneGreaterThanCrntTimeUcc,
            Self::DimOneLenEq => &DimOneLenEqUcc,
            Self::DimOneLenGreaterThan => &DimOneLenGreaterThanUcc,
            Self::EqToEncodedStringRepresentation => &EqToEncodedStringRepresentationUcc,
            Self::DimOneEqToEncodedStringRepresentation => {
                &DimOneEqToEncodedStringRepresentationUcc
            }
            Self::FindRangesWithinGivenRange { .. } => &FindRangesWithinGivenRangeUcc,
            Self::DimOneFindRangesWithinGivenRange { .. } => &DimOneFindRangesWithinGivenRangeUcc,
            Self::FindRangesThatFullyContainTheGivenRange { .. } => {
                &FindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::DimOneFindRangesThatFullyContainTheGivenRange { .. } => {
                &DimOneFindRangesThatFullyContainTheGivenRangeUcc
            }
            Self::StrictlyToLeftOfRange { .. } => &StrictlyToLeftOfRangeUcc,
            Self::DimOneStrictlyToLeftOfRange { .. } => &DimOneStrictlyToLeftOfRangeUcc,
            Self::StrictlyToRightOfRange { .. } => &StrictlyToRightOfRangeUcc,
            Self::DimOneStrictlyToRightOfRange { .. } => &DimOneStrictlyToRightOfRangeUcc,
            Self::IncludedLowerBound { .. } => &IncludedLowerBoundUcc,
            Self::DimOneIncludedLowerBound { .. } => &DimOneIncludedLowerBoundUcc,
            Self::ExcludedUpperBound { .. } => &ExcludedUpperBoundUcc,
            Self::DimOneExcludedUpperBound { .. } => &DimOneExcludedUpperBoundUcc,
            Self::GreaterThanIncludedLowerBound { .. } => &GreaterThanIncludedLowerBoundUcc,
            Self::DimOneGreaterThanIncludedLowerBound { .. } => {
                &DimOneGreaterThanIncludedLowerBoundUcc
            }
            Self::GreaterThanExcludedUpperBound { .. } => &GreaterThanExcludedUpperBoundUcc,
            Self::DimOneGreaterThanExcludedUpperBound { .. } => {
                &DimOneGreaterThanExcludedUpperBoundUcc
            }
            Self::OverlapWithRange { .. } => &OverlapWithRangeUcc,
            Self::DimOneOverlapWithRange { .. } => &DimOneOverlapWithRangeUcc,
            Self::AdjacentWithRange { .. } => &AdjacentWithRangeUcc,
            Self::DimOneAdjacentWithRange { .. } => &DimOneAdjacentWithRangeUcc,
            Self::RangeLen => &RangeLenUcc,
            Self::DimOneRangeLen => &DimOneRangeLenUcc,
        }
    }
}
pub trait PgFlt {
    fn mb_generic(&self) -> Option<Ts2>;
    fn prefix_wh_self_ucc(&self) -> Ts2;
    fn ucc(&self) -> &'static dyn DisplayPlusToTokens;
}
