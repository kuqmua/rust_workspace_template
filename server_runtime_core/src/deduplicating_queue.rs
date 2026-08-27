#[path = "deduplicating_queue/collections_hash_set.rs"]
mod collections_hash_set;
#[path = "deduplicating_queue/collections_vec_deque.rs"]
mod collections_vec_deque;
#[path = "deduplicating_queue/deduplicating_queue.rs"]
mod deduplicating_queue;
#[path = "deduplicating_queue/queue_maximum_non_zero_usize.rs"]
mod queue_maximum_non_zero_usize;
#[path = "deduplicating_queue/queue_push.rs"]
mod queue_push;

use collections_hash_set::CollectionsHashSet;
use collections_vec_deque::CollectionsVecDeque;
pub use deduplicating_queue::DeduplicatingQueue;
pub use queue_maximum_non_zero_usize::QueueMaximumNonZeroUsize;
pub use queue_push::QueuePush;

#[cfg(test)]
mod tests {
    #[test]
    fn queue_deduplicates_limits_and_releases_key_after_pop() {
        let mut queue = super::DeduplicatingQueue::new(super::QueueMaximumNonZeroUsize::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(queue.push(1u8), super::QueuePush::Queued);
        assert_eq!(queue.push(1u8), super::QueuePush::Duplicate);
        assert_eq!(queue.push(2u8), super::QueuePush::Full);
        assert_eq!(queue.pop(), Some(1u8));
        assert_eq!(queue.push(1u8), super::QueuePush::Queued);
    }
}
