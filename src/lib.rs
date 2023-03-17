#![feature(
    generic_const_exprs,
    const_trait_impl,
    const_type_id,
    associated_const_equality
)]
#![allow(incomplete_features)]
#![deny(missing_debug_implementations, missing_docs)]
#![no_std]

//! The `tyght-map` crate provides a static type map implementation.
//!
//! The map, [`TyghtMap`], enjoys the following properties:
//!
//! - The size of the map will match the size of its items.
//! - No heap allocations, this crate is `!#[no_std]`.
//! - All methods on the map are infallible.
//!
//! # Example
//!
//! ```
//! #![feature(generic_const_exprs)]
//! # use tyght_map::*;
//! // Insert some different integer types into the map and check the size
//! let map = TyghtMap::new()
//!     .insert(3u32)
//!     .insert(4i32)
//!     .insert(3f32);
//! assert_eq!(std::mem::size_of_val(&map), 12);
//!
//! // Retrieve the `u32` from the map
//! let item: &u32 = map.get();
//! assert_eq!(*item, 3);
//!
//! // Insert a `String` into the map, then mutate it
//! let mut map = map.insert("Hello".to_string());
//! *map.get_mut::<String>() += ", world!";
//!
//! // Remove the `String` from the map
//! let (item, _map) = map.remove::<String>();
//! println!("{item}");
//! ```
//!
//! # Traits
//!
//! For each operation on [`TyghtMap`] there is an associated trait:
//!
//! - [`Contains<T>`](Contains) is implemented on `S` when it contains `T` allowing [`get`](TyghtMap::get) and [`remove`](TyghtMap::remove)
//! - [`Missing<T>`](Missing) is implemented on `S` when it doesn't contain `T` allowing [`insert`](TyghtMap::insert)
//!
//! This means that placing constraints on the `S` of `TyghtMap<S>` acts as a constraint on the items it contains.
//!
//! For example, the following function _cannot_ be called using a map which does not contain a `String` and a `u32`.
//!
//! ```
//! # use tyght_map::*;
//! fn print_string<S>(map: &TyghtMap<S>)
//! where
//!     S: Contains<String>,
//!     S: Contains<u32>
//! {
//!     let string: &String = map.get();
//!     let int: &u32 = map.get();
//!     println!("{string} {int}");
//! }
//! ```
//!
//! # Known Limitations
//!
//! Currently, the map can only store 32 items. This limit can be raised if compile-times are known to be reasonable.
//!
//! Future improvements to `rustc`s type system may remove the need for a limit all together.
//!
//! # Nightly
//!
//! In contrast to other attempts, this implementation does not rely on specialization. It does however rely on a
//! variety of nightly features:
//!
//! - [`generic_const_exprs`](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html)
//! - [`const_trait_impl`](https://doc.rust-lang.org/beta/unstable-book/language-features/const-trait-impl.html)
//! - [`const_type_id`](https://doc.rust-lang.org/beta/unstable-book/library-features/const-type-id.html)
//! - [`associated_const_equality`](https://doc.rust-lang.org/beta/unstable-book/language-features/associated-const-equality.html)
//!
//! These can be expected to be stabilized, in some form, before specialization.
//!

mod get;
mod insert;
mod macros;
mod remove;

use get::Get;
use insert::Insert;
use remove::Remove;

/// A trait marking whether `T` is present.
pub trait Contains<T>: Get<T> + Remove<T> {}

/// A trait marking whether `T` is absent.
pub trait Missing<T>: Insert<T> {}

impl<T, S> Contains<T> for S where S: Get<T> + Remove<T> {}
impl<T, S> Missing<T> for S where S: Insert<T> {}

/// A static type map.
///
/// See the [crate-level documentation](crate) for more information.
#[derive(Debug, Clone)]
pub struct TyghtMap<S>(S);

impl Default for TyghtMap<()> {
    fn default() -> Self {
        Self(())
    }
}

impl TyghtMap<()> {
    /// Constructs an empty map.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> TyghtMap<S> {
    /// Inserts an item.
    ///
    /// This consumes then returns the map.
    #[must_use]
    pub fn insert<T>(self, item: T) -> TyghtMap<S::Output>
    where
        S: Missing<T>,
    {
        TyghtMap(self.0.insert(item))
    }

    /// Retrieves an item.
    pub fn get<T>(&self) -> &T
    where
        S: Contains<T>,
    {
        self.0.get()
    }

    /// Retrieves an item mutably.
    pub fn get_mut<T>(&mut self) -> &mut T
    where
        S: Contains<T>,
    {
        self.0.get_mut()
    }

    /// Removes an item.
    ///
    /// This consumes the map and returns an `(item, map)` pair.
    #[must_use]
    pub fn remove<T>(self) -> (T, TyghtMap<S::Output>)
    where
        S: Contains<T>,
    {
        let (item, map) = self.0.remove();
        (item, TyghtMap(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(
        (u32, f32, f64): Contains<u32>,
        Contains<f32>,
        Contains<f64>,
        Missing<&'static str>,
        Missing<i32>,
        Missing<u8>
    );
    static_assertions::assert_not_impl_any!(
        (u32, f32, f64): Missing<u32>,
        Missing<f32>,
        Missing<f64>,
        Contains<&'static str>,
        Contains<i32>,
        Contains<u8>
    );
}
