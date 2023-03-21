use crate::indexable::{FindIndex, Map, MaybeIndexable};

/// Allows for the retrieval of a unique type from a static container.
///
/// This is implemented for every tuple containing an element of type `T`.
pub trait TryGet<T> {
    /// Tries to return a reference to the value with corresponding type.
    fn try_get(&self) -> Option<&T>;

    /// Tries to return a mutable reference to the value with corresponding type.
    fn try_get_mut(&mut self) -> Option<&mut T>;
}

impl<T, S> TryGet<T> for S
where
    S: FindIndex<T>,
    S: MaybeIndexable<{ S::INDEX }>,
    S::ItemMap: Map<T, Source = S::Item>,
{
    fn try_get(&self) -> Option<&T> {
        let item = self.try_get_by_index()?;
        Some(S::ItemMap::map(item))
    }

    fn try_get_mut(&mut self) -> Option<&mut T> {
        let item = self.try_get_by_index_mut()?;
        Some(S::ItemMap::map_mut(item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, u64, &'static str, f32): TryGet<u32>,
        TryGet<u64>,
        TryGet<&'static str>,
        TryGet<f32>,
        TryGet<f64>
    );

    #[test]
    fn check_try_get() {
        let map = ("hi", 3u32, 4f32);
        let value: Option<&u32> = map.try_get();
        assert_eq!(value, Some(&3));

        let value: Option<&f64> = map.try_get();
        assert_eq!(value, None);
    }
}
