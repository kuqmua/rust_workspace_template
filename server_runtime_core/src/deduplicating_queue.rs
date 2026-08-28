#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "collections_hash_set.rs"]
mod collections_hash_set;
#[path = "collections_vec_deque.rs"]
mod collections_vec_deque;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    items: CollectionsVecDeque<Item>,
    keys: CollectionsHashSet<Item>,
    maximum: QueueMaximumNonZeroUsize,
}

impl<Item> DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    #[must_use]
    pub fn new(maximum: QueueMaximumNonZeroUsize) -> Self {
        Self {
            items: CollectionsVecDeque::from(std::collections::VecDeque::with_capacity(
                maximum.0.get(),
            )),
            keys: CollectionsHashSet::from(std::collections::HashSet::with_capacity(
                maximum.0.get(),
            )),
            maximum,
        }
    }

    pub fn pop(&mut self) -> Option<Item> {
        let item = self.items.0.pop_front()?;
        let _removed = self.keys.0.remove(&item);
        Some(item)
    }

    pub fn push(&mut self, item: Item) -> QueuePush {
        if self.keys.0.contains(&item) {
            QueuePush::Duplicate
        } else if self.items.0.len() >= self.maximum.0.get() {
            QueuePush::Full
        } else {
            let _inserted = self.keys.0.insert(item.clone());
            self.items.0.push_back(item);
            QueuePush::Queued
        }
    }
}
#[path = "queue_maximum_non_zero_usize.rs"]
mod queue_maximum_non_zero_usize;
#[path = "queue_push.rs"]
mod queue_push;

use collections_hash_set::CollectionsHashSet;
use collections_vec_deque::CollectionsVecDeque;
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
