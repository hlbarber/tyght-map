#![feature(
    generic_const_exprs,
    const_trait_impl,
    const_type_id,
    associated_const_equality
)]
#![allow(incomplete_features)]
#![deny(missing_debug_implementations, missing_docs, unsafe_code)]
#![no_std]

//! The `tyght-map` crate provides a static type map implementation.
//!
//! A type map is a map where the values are indexed by their types.
//!
//! The map, [`TyghtMap`], enjoys the following properties:
//!
//! - The size of the map will match the size of its items.
//! - No heap allocations, this crate is `!#[no_std]`.
//! - Provides both infallible and fallible methods.
//! - No unsafe.
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
//! let mut map = map.insert("Hey".to_string());
//! *map.get_mut::<String>() += ", world!";
//!
//! // Try to get a `u8` from the map
//! let item = map.try_get::<u8>();
//! assert_eq!(item, None);
//!
//! // Remove the `String` from the map
//! let (item, _map) = map.remove::<String>();
//! println!("{item}");
//! ```
//!
//! # Traits
//!
//! Placing constraints on the `S` of `TyghtMap<S>` acts as a constraint on the values it contains.
//!
//! There are three important marker traits:
//!
//! - [`Contains<T>`](Contains) is implemented on `S` when it contains `T` allowing:
//!     - [`replace`](TyghtMap::replace)
//!     - [`get`](TyghtMap::get)
//!     - [`get_mut`](TyghtMap::get_mut)
//!     - [`remove`](TyghtMap::remove)
//! - [`MaybeContains<T>`](MaybeContains) is always([`*`](#known-limitations)) implemented on `S`
//! allowing:
//!     - [`try_insert`](TyghtMap::try_insert)
//!     - [`try_get`](TyghtMap::try_get)
//!     - [`try_get_mut`](TyghtMap::try_get_mut)
//!     - [`try_remove`](TyghtMap::try_remove)
//! - [`Missing<T>`](Missing) is implemented on `S` when it doesn't contain `T` allowing:
//!     - [`insert`](TyghtMap::insert)
//!
//! The following function _cannot_ be called using a map which does not contain a `String` and a `u32`.
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
//! Currently, the map can only store 32 items. This is done out of consideration for compile times.
//!
//! The storage size can be changed by the use of feature flags:
//!
//! - `size-16` is 16 items.
//! - `size-32` is 32 items (default).
//! - Otherwise 8 items (no default features).
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
mod indexable;
mod insert;
mod macros;
mod remove;

use get::{Get, TryGet};
use insert::{Insert, TryInsert};
use remove::{Remove, TryRemove};

/// A trait marking whether `T` is present.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait Contains<T>: MaybeContains<T> + Get<T> + Remove<T> {}

/// A trait marking whether `T` is maybe present.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait MaybeContains<T>: TryInsert<T> + TryGet<T> + TryRemove<T> {}

/// A trait marking whether `T` is absent.
///
/// See [Traits](crate#traits) section of crate documentation for more information.
pub trait Missing<T>: MaybeContains<T> + Insert<T> {}

impl<T, S> Contains<T> for S where S: MaybeContains<T> + Get<T> + Remove<T> {}
impl<T, S> MaybeContains<T> for S where S: TryInsert<T> + TryGet<T> + TryRemove<T> {}
impl<T, S> Missing<T> for S where S: MaybeContains<T> + Insert<T> {}

/// A static type map.
///
/// See the [crate-level documentation](crate) for more information.
#[derive(Debug, Clone, Copy)]
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
    /// Replaces a value.
    pub fn replace<T>(&mut self, item: T) -> T
    where
        S: Contains<T>,
    {
        let old = core::mem::replace(self.get_mut(), item);
        old
    }

    /// Tries to insert a value. If a value, with the same type, already exists then replace
    /// and return it.
    ///
    /// This consumes the map then returns an `(optional_item, map)` pair.
    #[must_use]
    pub fn try_insert<T>(self, item: T) -> (Option<T>, TyghtMap<S::InsertOutput>)
    where
        S: MaybeContains<T>,
    {
        let (item, output) = self.0.try_insert(item);
        (item, TyghtMap(output))
    }

    /// Inserts a value.
    ///
    /// This consumes then returns the map.
    #[must_use]
    pub fn insert<T>(self, item: T) -> TyghtMap<S::InsertOutput>
    where
        S: Missing<T>,
    {
        TyghtMap(self.0.insert(item))
    }

    /// Returns a reference to a value with a given type.
    pub fn get<T>(&self) -> &T
    where
        S: Contains<T>,
    {
        self.0.get()
    }

    /// Tries to return a reference to a value with a given type.
    pub fn try_get<T>(&self) -> Option<&T>
    where
        S: MaybeContains<T>,
    {
        self.0.try_get()
    }

    /// Returns a mutable reference to a value with a given type.
    pub fn get_mut<T>(&mut self) -> &mut T
    where
        S: Contains<T>,
    {
        self.0.get_mut()
    }

    /// Tries to return a reference to a value with a given type.
    pub fn try_get_mut<T>(&mut self) -> Option<&mut T>
    where
        S: MaybeContains<T>,
    {
        self.0.try_get_mut()
    }

    /// Removes a value with a given type.
    ///
    /// This consumes the map and returns an `(item, map)` pair.
    #[must_use]
    pub fn remove<T>(self) -> (T, TyghtMap<S::RemoveOutput>)
    where
        S: Contains<T>,
    {
        let (item, map) = self.0.remove();
        (item, TyghtMap(map))
    }

    /// Tries to remove a value with a given type.
    ///
    /// This consumes the map and returns an `(optional_item, map)` pair.
    #[must_use]
    pub fn try_remove<T>(self) -> (Option<T>, TyghtMap<S::RemoveOutput>)
    where
        S: MaybeContains<T>,
    {
        let (item, map) = self.0.try_remove();
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
