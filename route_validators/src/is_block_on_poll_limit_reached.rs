pub(super) fn is_block_on_poll_limit_reached(
    poll_count: crate::test_poll_count::TestPollCount,
) -> crate::test_poll_limit_reached::TestPollLimitReached {
    crate::test_poll_limit_reached::TestPollLimitReached::from(
        *poll_count >= crate::max_block_on_polls::MAX_BLOCK_ON_POLLS,
    )
}
