use crate::indexable::{FindIndex, Indexable, Map, MaybeIndexable};

pub trait TryRemove<T> {
    /// The container type post-removal.
    type RemoveOutput;

    /// Removes a type from the map, returning the value.
    fn try_remove(self) -> (Option<T>, Self::RemoveOutput);
}

impl<T, S> TryRemove<T> for S
where
    S: FindIndex<T>,
    S: MaybeIndexable<{ S::INDEX }>,
    S::ItemMap: Map<T, Source = S::Item>,
{
    type RemoveOutput = S::Removed;

    fn try_remove(self) -> (Option<T>, Self::RemoveOutput) {
        let (item, output) = self.remove_by_index();
        let item = item.ok().map(S::ItemMap::map);
        (item, output)
    }
}

/// Allows for the removal of a unique type from a static container.
///
/// This is implemented for every tuple containing an element of type `T`.
pub trait Remove<T>: TryRemove<T> {
    /// Removes a type from the map, returning the value.
    fn remove(self) -> (T, Self::RemoveOutput);
}

impl<T, S> Remove<T> for S
where
    S: FindIndex<T>,
    S: Indexable<{ S::INDEX }>,
    S::ItemMap: Map<T, Source = S::Item>,
{
    fn remove(self) -> (T, Self::RemoveOutput) {
        let (item, output) = self.remove_by_index();
        let item = item.unwrap_or_else(|never| match never {});
        let item = S::ItemMap::map(item);
        (item, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, u64, &'static str, f32): Remove<u32>,
        Remove<u64>,
        Remove<&'static str>,
        Remove<f32>,
        TryRemove<u64>,
        TryRemove<f64>
    );
    static_assertions::assert_not_impl_all!(
        (u32, u64): Remove<&'static str>,
        Remove<u128>,
        Remove<i32>,
        Remove<f32>
    );

    #[test]
    fn try_remove() {
        let map = ("hi", 3u32, 4f32);
        let (x, map): (Option<&'static str>, _) = map.try_remove();
        let (y, map): (Option<u32>, _) = map.try_remove();
        let (z, map): (Option<f64>, _) = map.try_remove();

        assert_eq!((4f32,), map);
        assert_eq!((x, y, z), (Some("hi"), Some(3u32), None));
    }

    #[test]
    fn remove() {
        let map = ("hi", 3u32, 4f32);
        let (x, map): (&'static str, _) = map.remove();
        let (y, map): (u32, _) = map.remove();
        let (z, map): (f32, _) = map.remove();

        assert_eq!((), map);
        assert_eq!((x, y, z), ("hi", 3u32, 4f32));
    }
}
