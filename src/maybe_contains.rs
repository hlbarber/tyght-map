use core::intrinsics::type_id;

use crate::{Cons, Nil};

pub trait LocalMaybeContains<Item, const HEAD: bool> {
    const CONTAINS: bool;

    type Inserted;
    type Removed;

    fn try_get(&self) -> Option<&Item>;

    fn try_get_mut(&mut self) -> Option<&mut Item>;

    fn try_remove(self) -> (Option<Item>, Self::Removed);

    fn try_insert(self, value: Item) -> (Option<Item>, Self::Inserted);
}

impl<Item> LocalMaybeContains<Item, true> for Nil
where
    Item: 'static,
{
    const CONTAINS: bool = false;

    type Inserted = Cons<Item, Nil>;
    type Removed = Nil;

    fn try_get(&self) -> Option<&Item> {
        None
    }

    fn try_get_mut(&mut self) -> Option<&mut Item> {
        None
    }

    fn try_remove(self) -> (Option<Item>, Self::Removed) {
        (None, self)
    }

    fn try_insert(self, head: Item) -> (Option<Item>, Self::Inserted) {
        (None, Cons { head, tail: self })
    }
}

impl<Item, T> LocalMaybeContains<Item, true> for Cons<Item, T>
where
    Item: 'static,
{
    const CONTAINS: bool = true;

    type Inserted = Self;
    type Removed = T;

    fn try_get(&self) -> Option<&Item> {
        Some(&self.head)
    }

    fn try_get_mut(&mut self) -> Option<&mut Item> {
        Some(&mut self.head)
    }

    fn try_remove(self) -> (Option<Item>, Self::Removed) {
        let Self { head, tail } = self;
        (Some(head), tail)
    }

    fn try_insert(self, item: Item) -> (Option<Item>, Self::Inserted) {
        let Self { head, tail } = self;
        (Some(head), Cons { head: item, tail })
    }
}

impl<Item, H, T> LocalMaybeContains<Item, false> for Cons<H, T>
where
    Item: 'static,
    T: MaybeContains<Item>,
{
    const CONTAINS: bool = T::CONTAINS;

    type Inserted = Cons<H, T::Inserted>;
    type Removed = Cons<H, T::Removed>;

    fn try_get(&self) -> Option<&Item> {
        self.tail.try_get()
    }

    fn try_get_mut(&mut self) -> Option<&mut Item> {
        self.tail.try_get_mut()
    }

    fn try_remove(self) -> (Option<Item>, Self::Removed) {
        let Self { head, tail } = self;
        let (item, removed) = tail.try_remove();
        let removed: Self::Removed = Cons {
            head,
            tail: removed,
        };
        (item, removed)
    }

    fn try_insert(self, item: Item) -> (Option<Item>, Self::Inserted) {
        let Self { head, tail } = self;
        let (item, inserted) = tail.try_insert(item);
        let inserted = Cons {
            head,
            tail: inserted,
        };
        (item, inserted)
    }
}

/// A trait marking whether `T` is maybe present.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait MaybeContains<Item> {
    #[doc(hidden)]
    const CONTAINS: bool;

    #[doc(hidden)]
    type Removed;
    #[doc(hidden)]
    type Inserted;

    #[doc(hidden)]
    fn try_get(&self) -> Option<&Item>;

    #[doc(hidden)]
    fn try_get_mut(&mut self) -> Option<&mut Item>;

    #[doc(hidden)]
    fn try_insert(self, item: Item) -> (Option<Item>, Self::Inserted);

    #[doc(hidden)]
    fn try_remove(self) -> (Option<Item>, Self::Removed);
}

impl<Item> MaybeContains<Item> for Nil {
    const CONTAINS: bool = false;

    type Inserted = Cons<Item, Nil>;
    type Removed = Nil;

    fn try_get(&self) -> Option<&Item> {
        None
    }

    fn try_get_mut(&mut self) -> Option<&mut Item> {
        None
    }

    fn try_insert(self, head: Item) -> (Option<Item>, Self::Inserted) {
        (None, Cons { head, tail: self })
    }

    fn try_remove(self) -> (Option<Item>, Self::Removed) {
        (None, Nil)
    }
}

impl<Item, H, T> MaybeContains<Item> for Cons<H, T>
where
    Item: 'static,
    H: 'static,
    Self: LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>,
{
    type Inserted =
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::Inserted;
    type Removed =
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::Removed;

    const CONTAINS: bool =
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::CONTAINS;

    fn try_get(&self) -> Option<&Item> {
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::try_get(self)
    }

    fn try_get_mut(&mut self) -> Option<&mut Item> {
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::try_get_mut(
            self,
        )
    }

    fn try_insert(self, item: Item) -> (Option<Item>, Self::Inserted) {
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::try_insert(
            self, item,
        )
    }

    fn try_remove(self) -> (Option<Item>, Self::Removed) {
        <Self as LocalMaybeContains<Item, { type_id::<Item>() == type_id::<H>() }>>::try_remove(
            self,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cons, MaybeContains, Nil};

    static_assertions::assert_impl_all!(Nil: MaybeContains<u32, CONTAINS = false>);
    static_assertions::assert_impl_all!(Cons<u32, Nil>: MaybeContains<u32, CONTAINS = true>);
}
