use crate::maybe_contains::MaybeContains;

/// A trait marking whether `T` is absent.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait Missing<Item>: MaybeContains<Item, CONTAINS = false> {
    #[doc(hidden)]
    fn insert(self, item: Item) -> Self::Inserted
    where
        Self: Sized,
    {
        let (_, inserted) = self.try_insert(item);
        inserted
    }
}

impl<Item, T> Missing<Item> for T where T: MaybeContains<Item, CONTAINS = false> {}
