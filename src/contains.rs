use crate::maybe_contains::MaybeContains;

/// A trait marking whether `T` is present.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait Contains<Item>: MaybeContains<Item, CONTAINS = true> {
    #[doc(hidden)]
    fn get(&self) -> &Item {
        self.try_get().unwrap()
    }

    #[doc(hidden)]
    fn get_mut(&mut self) -> &mut Item {
        self.try_get_mut().unwrap()
    }

    #[doc(hidden)]
    fn remove(self) -> (Item, Self::Removed)
    where
        Self: Sized,
    {
        let (item, removed) = self.try_remove();
        (item.unwrap(), removed)
    }
}

impl<Item, T> Contains<Item> for T where T: MaybeContains<Item, CONTAINS = true> {}
