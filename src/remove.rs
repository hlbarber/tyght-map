use crate::indexable::{FindIndex, Indexable};

/// Allows for the removal of a unique type from a static container.
///
/// This is implemented for every tuple containing an element of type `T`.
pub trait Remove<T> {
    /// The container type post-removal.
    type Output;

    /// Removes a type from the map, returning the value.
    fn remove(self) -> (T, Self::Output);
}

impl<T, S> Remove<T> for S
where
    S: FindIndex<T>,
    S: Indexable<{ S::INDEX }, Item = T>,
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
