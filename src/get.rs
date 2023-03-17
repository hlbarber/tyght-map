use core::any::TypeId;

/// An interface for converting static containers into [`TypeIds`].
#[const_trait]
trait IntoTypeIds {
    const LENGTH: usize;

    /// Converts a static container into an array of [`TypeIds`].
    fn into_ids() -> [TypeId; Self::LENGTH];
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! impl_into_type_ids {
    ($($var:ident),*) => (
        impl<$($var,)*> const IntoTypeIds for ($($var,)*)
        where
            $($var: 'static,)*
        {
            const LENGTH: usize = count!($($var)*);

            fn into_ids() -> [TypeId; Self::LENGTH] {
                [$(TypeId::of::<$var>(),)*]
            }
        }
    )
}

// `IntoTypeIds` is implemented on tuples.
crate::macros::impl_all!(impl_into_type_ids);

/// Finds the index of the first element in `S` to match type `T`. Failures return [`usize::MAX`].
///
/// This means that, the absence of `T` in `S` can be inferred from `find_index() == `usize::MAX`.
const fn find_index<T, S>() -> usize
where
    T: 'static,
    S: ~const IntoTypeIds,
    [(); S::LENGTH]:,
{
    let id = TypeId::of::<T>();
    let arr = S::into_ids();

    let mut i = 0;
    while i < S::LENGTH {
        if arr[i] == id {
            return i;
        }
        i += 1;
    }
    usize::MAX
}

/// A useful interface for providing the `INDEX` of a type `T`, given by [`find_index`], within a static container.
pub(crate) trait GetIndex<T> {
    const INDEX: usize;
}

impl<T, S> GetIndex<T> for S
where
    T: 'static,
    S: ~const IntoTypeIds,
    [(); S::LENGTH]:,
{
    const INDEX: usize = find_index::<T, S>();
}

/// An interface for retrieving a value from a static container by `INDEX`.
pub trait GetByIndex<const INDEX: usize> {
    type Item;

    /// Retrieves a value by index.
    fn get_by_index(&self) -> &Self::Item;

    /// Retrieves a mutable value by index.
    fn get_by_index_mut(&mut self) -> &mut Self::Item;
}

macro_rules! get_by_index {
    (@step $_idx:expr, $($_head:ident,)* ; ) => {};

    (@step $idx:expr, $($head:ident,)* ; $current:ident, $($tail:ident,)*) => {
        impl<$($head,)* $current, $($tail,)*> GetByIndex<{ $idx }> for ($($head,)* $current, $($tail,)*)
        {
            type Item = $current;

            #[allow(unused_variables, non_snake_case)]
            fn get_by_index(&self) -> &Self::Item {
                let ($($head,)* current, $($tail,)*) = self;
                current
            }

            #[allow(unused_variables, non_snake_case)]
            fn get_by_index_mut(&mut self) -> &mut Self::Item {
                let ($($head,)* current, $($tail,)*) = self;
                current
            }
        }

        get_by_index!(@step $idx + 1usize, $($head,)* $current, ; $($tail,)* );
    };

    ($($var:ident),*) => {
        get_by_index!(@step 0usize, ; $($var,)*);
    }
}

// `GetByIndex` is implemented on tuples.
crate::macros::impl_all!(get_by_index);

/// Allows for the retrieval of a unique type from a static container.
///
/// This is implemented for every tuple containing an element of type `T`.
pub trait Get<T> {
    /// Retrieves an item with a unique type from a static container.
    fn get(&self) -> &T;

    /// Retrieves a mutable item with a unique type from a static container.
    fn get_mut(&mut self) -> &mut T;
}

impl<T, S> Get<T> for S
where
    T: 'static,
    S: GetIndex<T>,
    S: GetByIndex<{ S::INDEX }, Item = T>,
{
    fn get(&self) -> &T {
        self.get_by_index()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_by_index_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_not_impl_all!(
        (u32, u64): Get<&'static str>,
        Get<u128>,
        Get<i32>,
        Get<f32>
    );
    static_assertions::assert_impl_all!(
        (u32, u64, &'static str, f32): Get<u32>,
        Get<u64>,
        Get<&'static str>,
        Get<f32>
    );

    #[test]
    fn check_get() {
        let map = ("hi", 3u32, 4f32);
        let got: (&u32, &&'static str, &f32) = (map.get(), map.get(), map.get());

        assert_eq!((&map.1, &map.0, &map.2), got);
    }
}
