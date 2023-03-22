use crate::indexable::{FindIndex, Indexable, Map, MaybeIndexable};

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
    #[inline]
    fn try_get(&self) -> Option<&T> {
        let item = self.get_by_index().ok()?;
        Some(S::ItemMap::map_ref(item))
    }

    #[inline]
    fn try_get_mut(&mut self) -> Option<&mut T> {
        let item = self.get_by_index_mut().ok()?;
        Some(S::ItemMap::map_mut(item))
    }
}

pub trait Get<T>: TryGet<T> {
    /// Returns a reference to the value with corresponding type.
    fn get(&self) -> &T;

    /// Returns a mutable reference to the value with corresponding type.
    fn get_mut(&mut self) -> &mut T;
}

impl<T, S> Get<T> for S
where
    S: FindIndex<T>,
    S: Indexable<{ S::INDEX }>,
    S::ItemMap: Map<T, Source = S::Item>,
{
    #[inline]
    fn get(&self) -> &T {
        let item = self.get_by_index().unwrap_or_else(|never| match never {});
        S::ItemMap::map_ref(item)
    }

    #[inline]
    fn get_mut(&mut self) -> &mut T {
        let item = self
            .get_by_index_mut()
            .unwrap_or_else(|never| match never {});
        S::ItemMap::map_mut(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, u64, &'static str, f32): Get<u32>,
        Get<u64>,
        Get<&'static str>,
        Get<f32>,
        TryGet<&'static str>,
        TryGet<f64>
    );
    static_assertions::assert_not_impl_all!(
        (u32, u64): Get<&'static str>,
        Get<u128>,
        Get<i32>,
        Get<f32>
    );

    #[test]
    fn try_get() {
        let map = ("hi", 3u32, 4f32);
        let value: Option<&u32> = map.try_get();
        assert_eq!(value, Some(&3));

        let value: Option<&f64> = map.try_get();
        assert_eq!(value, None);
    }

    #[test]
    fn get() {
        let map = ("hi", 3u32, 4f32);
        let got: (&u32, &&'static str, &f32) = (map.get(), map.get(), map.get());

        assert_eq!((&map.1, &map.0, &map.2), got);
    }
}
