use super::TestExpId;

#[track_caller]
pub(crate) fn panic_unexpected_variant(exp_id: impl Into<TestExpId>) -> ! {
    let exp_id = exp_id.into();
    panic!("4fe6f2e6 id={exp_id}");
}
