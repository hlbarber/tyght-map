use crate::indexable::{FindIndex, Indexable};

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
    S: FindIndex<T>,
    S: Indexable<{ S::INDEX }, Item = T>,
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
