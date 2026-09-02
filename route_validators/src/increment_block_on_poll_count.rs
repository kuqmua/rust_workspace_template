pub(super) fn increment_block_on_poll_count(
    test_poll_count: &mut crate::test_poll_count::TestPollCount,
) {
    **test_poll_count = test_poll_count.saturating_add(1);
}
