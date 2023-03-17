use crate::get::{GetByIndex, GetIndex};

/// An interface for removing a value from a static container by `INDEX`.
pub trait RemoveByIndex<const INDEX: usize>: GetByIndex<INDEX> {
    type Removed;

    /// Removes a value by index.
    fn remove_by_index(self) -> (Self::Item, Self::Removed);
}

macro_rules! remove_by_index {
    (@step $_idx:expr, $($_head:ident,)* ; ) => {};

    (@step $idx:expr, $($head:ident,)* ; $current:ident, $($tail:ident,)*) => {
        impl<$($head,)* $current, $($tail,)*> RemoveByIndex<{ $idx }> for ($($head,)* $current, $($tail,)*)
        {
            type Removed = ($($head,)* $($tail,)*);

            #[allow(non_snake_case)]
            fn remove_by_index(self) -> (Self::Item, Self::Removed) {
                let ($($head,)* current, $($tail,)*) = self;
                (current, ($($head,)* $($tail,)*))
            }
        }

        remove_by_index!(@step $idx + 1usize, $($head,)* $current, ; $($tail,)* );
    };

    ($($var:ident),*) => {
        remove_by_index!(@step 0usize, ; $($var,)*);
    }
}

// `RemoveByIndex` is implemented on tuples.
crate::macros::impl_all!(remove_by_index);

/// Allows for the removal of a unique type from a static container.
///
/// This is implemented for every tuple _not_ containing an element of type `T`.
pub trait Remove<T> {
    /// The container type post-removal.
    type Output;

    /// Removes an item with a unique type from a static container.
    fn remove(self) -> (T, Self::Output);
}

impl<T, S> Remove<T> for S
where
    T: 'static,
    S: GetIndex<T>,
    S: RemoveByIndex<{ S::INDEX }, Item = T>,
{
    type Output = S::Removed;

    fn remove(self) -> (T, Self::Output) {
        self.remove_by_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_not_impl_all!(
        (u32, u64): Remove<&'static str>,
        Remove<u128>,
        Remove<i32>,
        Remove<f32>
    );
    static_assertions::assert_impl_any!(
        (u32, u64, &'static str, f32): Remove<u32>,
        Remove<u64>,
        Remove<&'static str>,
        Remove<f32>
    );

    #[test]
    fn check_remove() {
        let map = ("hi", 3u32, 4f32);
        let (x, map): (&'static str, _) = map.remove();
        let (y, map): (u32, _) = map.remove();
        let (z, map): (f32, _) = map.remove();

        assert_eq!((), map);
        assert_eq!((x, y, z), ("hi", 3u32, 4f32));
    }
}
