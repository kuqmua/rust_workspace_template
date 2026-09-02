#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    items: crate::collections_vec_deque::CollectionsVecDeque<Item>,
    keys: crate::collections_hash_set::CollectionsHashSet<Item>,
    maximum: crate::queue_maximum_non_zero_usize::QueueMaximumNonZeroUsize,
}

impl<Item> DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    #[must_use]
    pub fn new(maximum: crate::queue_maximum_non_zero_usize::QueueMaximumNonZeroUsize) -> Self {
        Self {
            items: crate::collections_vec_deque::CollectionsVecDeque::from(
                std::collections::VecDeque::with_capacity(maximum.get()),
            ),
            keys: crate::collections_hash_set::CollectionsHashSet::from(
                std::collections::HashSet::with_capacity(maximum.get()),
            ),
            maximum,
        }
    }

    pub fn pop(&mut self) -> Option<Item> {
        let item = self.items.pop_front()?;
        let _removed = self.keys.remove(&item);
        Some(item)
    }

    pub fn push(&mut self, item: Item) -> crate::queue_push::QueuePush {
        if self.keys.contains(&item) {
            crate::queue_push::QueuePush::Duplicate
        } else if self.items.len() >= self.maximum.get() {
            crate::queue_push::QueuePush::Full
        } else {
            let _inserted = self.keys.insert(item.clone());
            self.items.push_back(item);
            crate::queue_push::QueuePush::Queued
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_queue_deduplicates_limits_and_releases_key_after_pop() {
        let mut queue = super::DeduplicatingQueue::new(
            crate::queue_maximum_non_zero_usize::QueueMaximumNonZeroUsize::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        assert_eq!(queue.push(1u8), crate::queue_push::QueuePush::Queued);
        assert_eq!(queue.push(1u8), crate::queue_push::QueuePush::Duplicate);
        assert_eq!(queue.push(2u8), crate::queue_push::QueuePush::Full);
        assert_eq!(queue.pop(), Some(1u8));
        assert_eq!(queue.push(1u8), crate::queue_push::QueuePush::Queued);
    }
}
