use crate::get::GetIndex;

/// A useful interface used to extend static containers.
pub trait Prepend<T> {
    type Prepended;

    /// Appends an item to a static container.
    fn prepend(self, item: T) -> Self::Prepended;
}

macro_rules! impl_prepend {
    ($($var:ident),*) => (
        impl<T, $($var,)*> Prepend<T> for ($($var,)*)
        {
            type Prepended = (T, $($var),*);

            #[allow(non_snake_case)]
            fn prepend(self, item: T) -> Self::Prepended {
                let ($($var,)*) = self;
                (item, $($var,)*)
            }
        }
    )
}

// `Append` is implemented on tuples.
crate::macros::impl_all!(impl_prepend);

/// Allows for the insertion of a unique type into a static container.
///
/// This is implemented for every tuple _not_ containing an element of type `T`.
pub trait Insert<T> {
    /// The container type post-insertion.
    type Output;

    /// Inserts an item with a unique type into a static container.
    fn insert(self, item: T) -> Self::Output;
}

impl<T, S> Insert<T> for S
where
    T: 'static,
    S: GetIndex<T, INDEX = { usize::MAX }>,
    S: Prepend<T>,
{
    type Output = S::Prepended;

    fn insert(self, item: T) -> Self::Output {
        self.prepend(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, u64): Insert<&'static str>,
        Insert<u128>,
        Insert<i32>,
        Insert<f32>
    );
    static_assertions::assert_not_impl_any!(
        (u32, u64, &'static str, f32): Insert<u32>,
        Insert<u64>,
        Insert<&'static str>,
        Insert<f32>
    );

    #[test]
    fn check_insert() {
        let x = ().insert(3u32).insert(4i32).insert("hey").insert(3f32);
        assert_eq!(x, (3f32, "hey", 4i32, 3u32));
    }
}
