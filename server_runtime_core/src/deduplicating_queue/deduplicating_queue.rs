#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    items: super::CollectionsVecDeque<Item>,
    keys: super::CollectionsHashSet<Item>,
    maximum: super::QueueMaximumNonZeroUsize,
}

impl<Item> DeduplicatingQueue<Item>
where
    Item: Clone + Eq + std::hash::Hash,
{
    #[must_use]
    pub fn new(maximum: super::QueueMaximumNonZeroUsize) -> Self {
        Self {
            items: super::CollectionsVecDeque::from(std::collections::VecDeque::with_capacity(
                maximum.0.get(),
            )),
            keys: super::CollectionsHashSet::from(std::collections::HashSet::with_capacity(
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

    pub fn push(&mut self, item: Item) -> super::QueuePush {
        if self.keys.0.contains(&item) {
            super::QueuePush::Duplicate
        } else if self.items.0.len() >= self.maximum.0.get() {
            super::QueuePush::Full
        } else {
            let _inserted = self.keys.0.insert(item.clone());
            self.items.0.push_back(item);
            super::QueuePush::Queued
        }
    }
}
