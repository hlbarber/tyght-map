use crate::indexable::{FindIndex, Map, MaybeIndexable};

pub trait TryInsert<T> {
    /// The container type post-insertion.
    type InsertOutput;

    /// Inserts a value, possibly replacing.
    fn try_insert(self, item: T) -> (Option<T>, Self::InsertOutput);
}

impl<T, S> TryInsert<T> for S
where
    S: FindIndex<T>,
    S: MaybeIndexable<{ S::INDEX }>,
    S::ItemMap: Map<T, Source = S::Item>,
{
    type InsertOutput = S::Inserted<T>;

    #[inline]
    fn try_insert(self, item: T) -> (Option<T>, Self::InsertOutput) {
        let (item, output) = self.insert_by_index(item);
        let item = item.ok().map(S::ItemMap::map);
        (item, output)
    }
}

pub trait Insert<T>: TryInsert<T> {
    /// Inserts a value.
    fn insert(self, item: T) -> Self::InsertOutput;
}

impl<T, S> Insert<T> for S
where
    S: FindIndex<T, INDEX = { usize::MAX }>,
    S: TryInsert<T>,
{
    #[inline]
    fn insert(self, item: T) -> Self::InsertOutput {
        let (_, output) = self.try_insert(item);
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, u64): Insert<&'static str>,
        Insert<u128>,
        Insert<i32>,
        Insert<f32>,
        TryInsert<u32>,
        TryInsert<u64>
    );

    static_assertions::assert_not_impl_any!((u32, u64): Insert<u32>, Insert<u64>);

    #[test]
    fn try_insert() {
        let (item, map) = ().try_insert(3u32);
        assert_eq!(item, None);
        let (item, map) = map.try_insert(4i32);
        assert_eq!(item, None);
        let (item, map) = map.try_insert(5i32);
        assert_eq!(item, Some(4i32));
        let (_, map) = map.try_insert("hey");
        assert_eq!(map, ("hey", 5i32, 3u32));
    }

    #[test]
    fn insert() {
        let x = ().insert(3u32).insert(4i32).insert("hey").insert(3f32);
        assert_eq!(x, (3f32, "hey", 4i32, 3u32));
    }
}
