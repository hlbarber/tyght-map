use crate::indexable::{FindIndex, MaybeIndexable};

/// Allows for the insertion of a unique type into a static container.
///
/// This is implemented for every tuple.
pub trait Insert<T> {
    /// The container type post-insertion.
    type Output;

    /// Inserts a value.
    fn insert(self, item: T) -> Self::Output;
}

impl<T, S> Insert<T> for S
where
    S: FindIndex<T>,
    S: MaybeIndexable<{ S::INDEX }>,
{
    type Output = S::Inserted<T>;

    fn insert(self, item: T) -> Self::Output {
        self.insert_by_index(item)
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
        Insert<u32>,
        Insert<u64>
    );

    #[test]
    fn check_insert() {
        let x = ().insert(3u32).insert(4i32).insert(5i32).insert("hey").insert(3f32);
        assert_eq!(x, (3f32, "hey", 5i32, 3u32));
    }
}
