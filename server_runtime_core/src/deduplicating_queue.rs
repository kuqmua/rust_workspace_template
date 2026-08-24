#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct StdQueueMaximum(std::num::NonZeroUsize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueuePush {
    Duplicate,
    Full,
    Queued,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    items: StdCollectionsVecDeque<Item>,
    keys: StdCollectionsHashSet<Item>,
    maximum: StdQueueMaximum,
}
impl<Item> DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    #[must_use]
    pub fn new(maximum: StdQueueMaximum) -> Self {
        Self {
            items: StdCollectionsVecDeque::from(std::collections::VecDeque::with_capacity(
                maximum.0.get(),
            )),
            keys: StdCollectionsHashSet::from(std::collections::HashSet::with_capacity(
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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct StdCollectionsHashSet<Item>(std::collections::HashSet<Item>);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct StdCollectionsVecDeque<Item>(std::collections::VecDeque<Item>);

#[cfg(test)]
mod tests {
    #[test]
    fn queue_deduplicates_limits_and_releases_key_after_pop() {
        let mut queue = super::DeduplicatingQueue::new(super::StdQueueMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(queue.push(1u8), super::QueuePush::Queued);
        assert_eq!(queue.push(1u8), super::QueuePush::Duplicate);
        assert_eq!(queue.push(2u8), super::QueuePush::Full);
        assert_eq!(queue.pop(), Some(1u8));
        assert_eq!(queue.push(1u8), super::QueuePush::Queued);
    }
}
