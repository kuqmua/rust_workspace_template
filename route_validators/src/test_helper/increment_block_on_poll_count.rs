use super::TestPollCount;

pub(super) fn increment_block_on_poll_count(poll_count: &mut TestPollCount) {
    poll_count.0 = poll_count.0.saturating_add(1);
}
