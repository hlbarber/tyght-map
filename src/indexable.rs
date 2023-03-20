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
pub trait FindIndex<T> {
    const INDEX: usize;
}

impl<T, S> FindIndex<T> for S
where
    T: 'static,
    S: ~const IntoTypeIds,
    [(); S::LENGTH]:,
{
    const INDEX: usize = find_index::<T, S>();
}

/// An interface for performing actions on an element of a static container by `INDEX`.
pub trait Indexable<const INDEX: usize> {
    type Item;
    type Removed;

    /// Returns a reference to the value corresponding to the index.
    fn get_by_index(&self) -> &Self::Item;

    /// Returns a mutable reference to the value corresponding to the index.
    fn get_by_index_mut(&mut self) -> &mut Self::Item;

    /// Removes a type from the map, returning its value.
    fn remove_by_index(self) -> (Self::Item, Self::Removed);
}

pub trait MaybeIndexable<const INDEX: usize> {
    type Inserted<T>;

    /// Inserts a value into the map.
    fn insert_by_index<T>(self, item: T) -> Self::Inserted<T>;
}

macro_rules! indexable {
    (@step $_idx:expr, $($_head:ident,)* ; ) => {};

    (@step $idx:expr, $($head:ident,)* ; $current:ident, $($tail:ident,)*) => {
        impl<$($head,)* $current, $($tail,)*> Indexable<{ $idx }> for ($($head,)* $current, $($tail,)*)
        {
            type Item = $current;
            type Removed = ($($head,)* $($tail,)*);

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

            #[allow(non_snake_case)]
            fn remove_by_index(self) -> (Self::Item, Self::Removed) {
                let ($($head,)* current, $($tail,)*) = self;
                (current, ($($head,)* $($tail,)*))
            }

        }

        impl<$($head,)* $current, $($tail,)*> MaybeIndexable<{ $idx }> for ($($head,)* $current, $($tail,)*)
        {
            type Inserted<T> = ($($head,)* T, $($tail,)*);

            #[allow(non_snake_case)]
            fn insert_by_index<T>(self, item: T) -> Self::Inserted<T> {
                let ($($head,)* _current, $($tail,)*) = self;
                ($($head,)* item, $($tail,)*)
            }
        }

        indexable!(@step $idx + 1usize, $($head,)* $current, ; $($tail,)* );
    };

    ($($var:ident),*) => {
        indexable!(@step 0usize, ; $($var,)*);

        impl<$($var,)*> MaybeIndexable<{ usize::MAX }> for ($($var,)*) {
            type Inserted<T> = (T, $($var,)*);

            #[allow(non_snake_case)]
            fn insert_by_index<T>(self, item: T) -> Self::Inserted<T> {
                let ($($var,)*) = self;
                (item, $($var,)*)
            }
        }
    }
}

// `Indexable` is implemented on tuples.
crate::macros::impl_all!(indexable);
