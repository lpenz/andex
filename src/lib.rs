// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

//! *andex* is a single-file, zero-dependency rust crate that helps us
//! create a strongly-typed, zero-cost, safe array index and use it to
//! index an array wrapper.
//!
//! This is specially useful in scenarios where we have different arrays
//! inside a `struct` and we want reference members without holding "hard"
//! references to those members.
//!
//! And it's all done without requiring the use of any macros.
//!
//! # Usage
//!
//! ## Creating the andex type and array
//!
//! [`Andex`] is the index type and [`AndexableArray`] is the type of
//! the array wrapper.
//!
//! The recommended approach to use andex is as follows:
//! - Create a unique empty type
//!   ```rust
//!   # use andex::*;
//!   enum MyIdxMarker {};
//!   ```
//! - Create a type alias for the [`Andex`] type that's parameterized
//!   with that type:
//!   ```rust
//!   # use andex::*;
//!   # enum MyIdxMarker {};
//!   type MyIdx = Andex<MyIdxMarker, 12>;
//!   ```
//! - Create a type alias for the [`AndexableArray`] type that's
//!   indexed by the [`Andex`] alias created above:
//!   ```rust
//!   # use andex::*;
//!   # enum MyIdxMarker {};
//!   # type MyIdx = Andex<MyIdxMarker, 12>;
//!   type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
//!   ```
//!
//! ## Creating andex instances
//!
//! When an andex is created, it knows *at compile time* the size of the
//! array it indexes, and all instances are assumed to be within bounds.
//!
//! For this reason, it's useful to limit the way `Andex`'s are
//! created. The ways we can get an instance is:
//!
//! - Via `new`, passing the value as a generic const argument:
//!   ```rust
//!   # use andex::*;
//!   # enum MyIdxMarker {};
//!   # type MyIdx = Andex<MyIdxMarker, 12>;
//!   const first : MyIdx = MyIdx::new::<0>();
//!   ```
//!   This checks that the value is valid at compile time, as long as you
//!   use it to create `const` variables.
//!
//! - Via `try_from`, which returns `Result<Andex,Error>` that has to be
//!   checked or explicitly ignored:
//!   ```rust
//!   # use std::convert::TryFrom;
//!   # use andex::*;
//!   # enum MyIdxMarker {};
//!   # type MyIdx = Andex<MyIdxMarker, 12>;
//!   if let Ok(first) = MyIdx::try_from(0) {
//!       // ...
//!   }
//!   ```
//!
//! - By iterating:
//!   ```rust
//!   # use andex::*;
//!   # enum MyIdxMarker {};
//!   # type MyIdx = Andex<MyIdxMarker, 12>;
//!   for idx in MyIdx::iter() {
//!       // ...
//!   }
//!   ```
//!
//! The assumption that the instances can only hold valid values allows us
//! to use `get_unsafe` and `get_unsafe_mut` in the indexer
//! implementation, which provides a bit of optimization by preventing the
//! bound check when indexing.
//!
//! # Full example
//!
//! ```
//! use std::convert::TryFrom;
//! use andex::*;
//!
//! // Create the andex type alias:
//! //   First, we need an empty type that we use as a marker:
//! enum MyIdxMarker {};
//! //   The andex type takes the marker (for uniqueness)
//! //   and the size of the array as parameters:
//! type MyIdx = Andex<MyIdxMarker, 12>;
//!
//! // Create the array wrapper:
//! type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
//!
//! // We can create other arrays indexable by the same Andex:
//! type MyF64 = AndexableArray<MyIdx, f64, { MyIdx::SIZE }>;
//!
//! fn example() {
//!     let myu32 = MyU32::default();
//!
//!     // We can now only index MyU32 using MyIdx
//!     const first : MyIdx = MyIdx::new::<0>();
//!     println!("{:?}", myu32[first]);
//!
//!     // Trying to create a MyIdx with an out-of-bounds value
//!     // doesn't work, this won't compile:
//!     // const _overflow : MyIdx = MyIdx::new::<30>();
//!
//!     // Trying to index myu32 with a "naked" number
//!     // doesn't work, this won't compile:
//!     // println!("{}", myu32[0]);
//!
//!     // We can only create indexes at compile-time or via try_from:
//!     const second : MyIdx = MyIdx::new::<1>();
//!     let third = MyIdx::try_from(2);
//!     // ^ Returns a Result, which Ok(MyIdx) if the value provided is
//!     // valid, or an error if it's not.
//!
//!     // The index type has an `iter()` method that produces
//!     // all possible values in order:
//!     for i in MyIdx::iter() {
//!         println!("{:?}", i);
//!     }
//! }
//! ```
//!
//! # Compile-time guarantees
//!
//! This is the reason to use Andex instead of a plain array in the
//! first play, right? Below is a list of some of the compile-time
//! restrictions that we get.
//!
//! - We can't index [`AndexableArray`] with a `usize`.
//!
//!   The following code doesn't compile:
//!
//! ```compile_fail
//! use andex::*;
//! enum MyIdxMarker {};
//! type MyIdx = Andex<MyIdxMarker, 12>;
//! type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
//!
//! fn example() {
//!     let myu32 = MyU32::default();
//!
//!     println!("{}", myu32[0]);
//! }
//! ```
//!
//! - We can't create an [`Andex`] with a value out-of-bounds (mostly)
//!
//!   The following code doesn't compile:
//!
//! ```compile_fail
//! use andex::*;
//! enum MyIdxMarker {};
//! type MyIdx = Andex<MyIdxMarker, 12>;
//!
//! fn example() {
//!     const myidx : MyIdx = MyIdx::new::<13>();
//! }
//! ```
//!
//! - We can't index [`AndexableArray`] with a different Andex, even when
//!   it has the same size. This is what using different markers gets
//!   us.
//!
//!   The following code doesn't compile:
//!
//! ```compile_fail
//! use andex::*;
//!
//! enum MyIdxMarker {};
//! type MyIdx = Andex<MyIdxMarker, 12>;
//! type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
//!
//! enum TheirIdxMarker {};
//! type TheirIdx = Andex<TheirIdxMarker, 12>;
//! type TheirU32 = AndexableArray<TheirIdx, u32, { TheirIdx::SIZE }>;
//!
//! fn example() {
//!     let myu32 = MyU32::default();
//!     let theirIdx = TheirIdx::new::<0>();
//!
//!     // We can't index a MyU32 array with TheirIdx:
//!     println!("{}", myu32[theirIdx]);
//! }
//! ```

use std::convert;
use std::error;
use std::fmt;
use std::marker::PhantomData;

/// Array index generic type
///
/// This generic type receives a user-specified "marker" type as the
/// first type parameter to make it unique, and the size of the array
/// as a second const generic `SIZE` parameter.
///
/// Note: the maximum numerical value in the andex is `SIZE - 1`.
///
/// Recommended usage, with an empty type as a marker to create a type
/// alias:
///
/// ```
/// use andex::*;
///
/// enum MyIdxMarker {}
/// type MyIdx = Andex<MyIdxMarker, 12>;
/// ```
pub struct Andex<M, const SIZE: usize>(PhantomData<M>, usize);

/// Andex-wide methods
///
/// [`Andex::new`] and [`Andex::iter`] are public, most other methods
/// are only used in traits, and thus private.
impl<M, const SIZE: usize> Andex<M, SIZE> {
    /// The `SIZE` parameter, which is the size of the array that this
    /// andex indexes.
    pub const SIZE: usize = SIZE;

    /// Create a new andex instance
    ///
    /// We recomment using this method in `const` contexts, passing
    /// the index as a const generic function parameter. That allows
    /// the compiler to check the index against the array bounds at
    /// compile time.
    ///
    /// For instance, the following compiles:
    /// ```
    /// use andex::*;
    ///
    /// struct MyIdxMarker;
    /// type MyIdx = Andex<MyIdxMarker, 12>;
    ///
    /// const MYVALUE : MyIdx = MyIdx::new::<0>();
    /// ```
    ///
    /// While the following doesn't:
    /// ```compile_fail
    /// use andex::*;
    ///
    /// struct MyIdxMarker;
    /// type MyIdx = Andex<MyIdxMarker, 13>;
    ///
    /// const MYVALUE : MyIdx = MyIdx::new::<15>();
    /// ```
    #[inline]
    pub const fn new<const N: usize>() -> Self {
        // Trick for compile-time check of N:
        const ASSERT: [(); 1] = [(); 1];
        let _ = ASSERT[(N >= SIZE) as usize];
        Andex(PhantomData, N)
    }

    /// Indexes the provided array
    ///
    /// Used internally by the `Index` trait implementation.
    #[inline]
    fn index_arr<'a, T>(&self, arr: &'a [T]) -> &'a T {
        unsafe { arr.get_unchecked(usize::from(self)) }
    }

    /// Mut-indexes the provided array
    ///
    /// Used internally by the `IndexMut` trait implementation.
    #[inline]
    fn index_arr_mut<'a, T>(&self, arr: &'a mut [T]) -> &'a mut T {
        unsafe { arr.get_unchecked_mut(usize::from(self)) }
    }

    /// Iterate all possible values of the index
    ///
    /// Useful to loop over an array inside a `struct`, without
    /// holding a reference to the whole struct in the loop.
    ///
    /// # Example
    ///
    /// This prints all numbers from 0 to 11:
    ///
    /// ```
    /// use andex::*;
    ///
    /// pub struct PlayerIdMarker;
    /// type PlayerId = Andex<PlayerIdMarker, 12>;
    ///
    /// fn function() {
    ///     for i in PlayerId::iter() {
    ///         println!("{}", i);
    ///     }
    /// }
    /// ```
    pub fn iter() -> AndexIterator<Self> {
        AndexIterator::<Self>::default()
    }
}

/* Generic implementations
 * We can't use the automatic derives to avoid requiring them in the
 * Marker.
 */

impl<M, const SIZE: usize> Clone for Andex<M, SIZE> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M, const SIZE: usize> Copy for Andex<M, SIZE> {}

impl<M, const SIZE: usize> Default for Andex<M, SIZE> {
    fn default() -> Self {
        Andex(PhantomData, 0)
    }
}

impl<M, const SIZE: usize> From<Andex<M, SIZE>> for usize {
    fn from(andex: Andex<M, SIZE>) -> Self {
        andex.1
    }
}

impl<M, const SIZE: usize> From<&Andex<M, SIZE>> for usize {
    fn from(andex: &Andex<M, SIZE>) -> Self {
        andex.1
    }
}

impl<M, const SIZE: usize> convert::TryFrom<usize> for Andex<M, SIZE> {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < SIZE {
            Ok(Andex(PhantomData, value))
        } else {
            Err(Error::OutOfBounds { value, size: SIZE })
        }
    }
}

impl<M, const SIZE: usize> fmt::Debug for Andex<M, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", usize::from(self))
    }
}

impl<M, const SIZE: usize> fmt::Display for Andex<M, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", usize::from(self))
    }
}

/* Iterator */

/// Iterator for Andex instances
///
/// This is the type returned by Andex::<_,_>::iter().
/// There's no reason to use it directly.
///
/// Iterating example:
///
/// ```
/// use andex::*;
///
/// pub struct PlayerIdMarker;
/// type PlayerId = Andex<PlayerIdMarker, 12>;
///
/// fn function() {
///     for i in PlayerId::iter() {
///         println!("{}", i);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct AndexIterator<A> {
    next: Option<usize>,
    phantom: PhantomData<A>,
}

impl<A> Default for AndexIterator<A> {
    fn default() -> Self {
        AndexIterator {
            next: Some(0),
            phantom: PhantomData,
        }
    }
}

impl<A> Iterator for AndexIterator<A>
where
    A: convert::TryFrom<usize>,
{
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.next.take() {
            let value = A::try_from(i).ok();
            if value.is_some() {
                self.next = Some(i + 1);
            } else {
                self.next = None;
            }
            value
        } else {
            None
        }
    }
}

/// Array wrapper indexable by the provided Andex type.
///
/// Example:
///
/// ```
/// use andex::*;
///
/// enum MyIdxMarker {};
/// type MyIdx = Andex<MyIdxMarker, 12>;
///
/// // Create the array wrapper:
/// type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
///
/// // We can create other arrays with the same Andex type:
/// type MyF64 = AndexableArray<MyIdx, f64, { MyIdx::SIZE }>;
///
/// fn example() {
///     // Create a default array:
///     let myu32 = MyU32::default();
///     // Print the first element:
///     const first : MyIdx = MyIdx::new::<0>();
///     println!("{:?}", myu32[first]);
///     // Iterate and print all elements:
///     for i in MyIdx::iter() {
///         println!("{:?}", myu32[i]);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct AndexableArray<A, Item, const SIZE: usize>([Item; SIZE], PhantomData<A>);

impl<A, Item: Default + Copy, const SIZE: usize> Default for AndexableArray<A, Item, SIZE> {
    fn default() -> Self {
        AndexableArray([Default::default(); SIZE], Default::default())
    }
}

impl<A, Item, const SIZE: usize> std::ops::Index<Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    type Output = Item;
    fn index(&self, index: Andex<A, SIZE>) -> &Self::Output {
        index.index_arr(&self.0)
    }
}

impl<A, Item, const SIZE: usize> std::ops::IndexMut<Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    fn index_mut(&mut self, index: Andex<A, SIZE>) -> &mut Item {
        index.index_arr_mut(&mut self.0)
    }
}

/* Errors: */

/// Andex errors enum
///
/// This is used by try_from when an invalid value is passed.
///
/// For instance, this code prints the error:
///
/// ```
/// use std::convert::TryFrom;
/// use andex::*;
///
/// enum MyIdxMarker {};
/// type MyIdx = Andex<MyIdxMarker, 12>;
///
/// fn example() {
///     println!("{:?}", MyIdx::try_from(15_usize));
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Tried to use a out-of-bounds value to create an andex
    OutOfBounds {
        /// The out-of-bounds value that was provided at andex
        /// creation
        value: usize,
        /// The `SIZE` of the andex type
        ///
        /// The maximum value accepted is `SIZE - 1`
        size: usize,
    },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfBounds {
                ref value,
                ref size,
            } => write!(
                f,
                "value {} is out-of-bounds for index with size {}",
                value, size
            ),
        }
    }
}
