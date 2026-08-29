#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
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
                std::collections::VecDeque::with_capacity(maximum.0.get()),
            ),
            keys: crate::collections_hash_set::CollectionsHashSet::from(
                std::collections::HashSet::with_capacity(maximum.0.get()),
            ),
            maximum,
        }
    }

    pub fn pop(&mut self) -> Option<Item> {
        let item = self.items.0.pop_front()?;
        let _removed = self.keys.0.remove(&item);
        Some(item)
    }

    pub fn push(&mut self, item: Item) -> crate::queue_push::QueuePush {
        if self.keys.0.contains(&item) {
            crate::queue_push::QueuePush::Duplicate
        } else if self.items.0.len() >= self.maximum.0.get() {
            crate::queue_push::QueuePush::Full
        } else {
            let _inserted = self.keys.0.insert(item.clone());
            self.items.0.push_back(item);
            crate::queue_push::QueuePush::Queued
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn queue_deduplicates_limits_and_releases_key_after_pop() {
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
