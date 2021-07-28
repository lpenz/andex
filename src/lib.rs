// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_doc_code_examples)]

//! *cludex* (exCLUsive inDEXes) is a single-file, zero-dependency rust
//! crate that helps us create a strongly-typed, zero-cost, safe array
//! index and use it to index a custom array wrapper.
//!
//! This is specially useful in scenarios where we have different arrays
//! inside a `struct` and we want reference members without holding "hard"
//! references.
//!
//! # Basic usage
//!
//! ```
//! use cludex::*;
//! use cludex::impl_cludex_for;
//!
//! // Create the type alias:
//! type MyIdx = Cludex<12>;
//!
//! // Create the array wrapper:
//! #[derive(Default)]
//! pub struct MyU32([u32; MyIdx::SIZE]);
//!
//! // Use `impl_cludex_for` to make it indexable:
//! impl_cludex_for!(MyU32, u32, MyIdx);
//!
//! fn example() {
//!     // Iterate:
//!     for i in MyIdx::iter() {
//!         println!("{:?}", i);
//!     }
//!     // Generate first index at compile time:
//!     const first : MyIdx = MyIdx::new::<0>();
//!     // Index the collection:
//!     let myu32 = MyU32::default();
//!     println!("{:?}", myu32[first]);
//! }
//! ```

use std::convert;
use std::error;
use std::fmt;

/// exCLUsive inDEX generic type
///
/// This is the type of the exclusive index that receives the size of
/// the array as a const generic `SIZE` parameter.
///
/// Note: the maximum numerical value in the cludex is `SIZE - 1`.
///
/// # Example
///
/// This should be used in a type alias, as such:
/// ```
/// use cludex::*;
///
/// type MyIndex = Cludex<12>;
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cludex<const SIZE: usize>(usize);

impl<const SIZE: usize> Cludex<SIZE> {
    /// The `SIZE` parameter, which is the size of the array that this
    /// cludex indexes.
    pub const SIZE: usize = SIZE;

    /// Create a new cludex instance
    ///
    /// Bounds are checked at compile time when this is used to create
    /// `const` instances - that's why this is the recommended usage:
    /// ```
    /// use cludex::*;
    /// type MyIndex = Cludex<12>;
    ///
    /// const MYVALUE : MyIndex = MyIndex::new::<0>();
    /// ```
    /// And that's why the following doesn't compile:
    /// ```compile_fail
    /// use cludex::*;
    /// type MyIndex = Cludex<12>;
    ///
    /// const MYVALUE : MyIndex = MyIndex::new::<12>();
    /// ```
    #[inline]
    pub const fn new<const N: usize>() -> Cludex<SIZE> {
        // Trick for compile-time check of N:
        const ASSERT: [(); 1] = [(); 1];
        let _ = ASSERT[(N >= SIZE) as usize];
        Cludex(N)
    }

    /// Extracts the numeric value of the index, consuming it
    pub const fn into_inner(self) -> usize {
        self.0
    }

    /// Iterate all possible values of the index
    ///
    /// Useful to loop over an array inside a `struct`, without
    /// holding a reference to the whole struct in the loop.
    ///
    /// # Example
    ///
    /// ```
    /// use cludex::*;
    ///
    /// type PlayerId = Cludex<12>;
    ///
    /// #[derive(Default)]
    /// pub struct Scores([u32; 12]);
    ///
    /// impl_cludex_for!(Scores, u32, PlayerId);
    ///
    /// #[derive(Default)]
    /// struct Game {
    ///     scores: Scores,
    /// }
    ///
    /// fn function(game: &Game) {
    ///     for i in PlayerId::iter() {
    ///         println!("score of player {} is {}", i, game.scores[i]);
    ///     }
    /// }
    /// ```
    pub fn iter() -> impl Iterator<Item = Cludex<SIZE>> {
        (0..SIZE).into_iter().map(Cludex)
    }

    /// Indexes the provided array
    ///
    /// Used internally by the `Index` trait implementation.
    #[inline]
    pub fn index_arr<'a, T>(&self, arr: &'a [T; SIZE]) -> &'a T {
        unsafe { arr.get_unchecked(self.0) }
    }

    /// Mut-indexes the provided array
    ///
    /// Used internally by the `IndexMut` trait implementation.
    #[inline]
    pub fn index_arr_mut<'a, T>(&self, arr: &'a mut [T; SIZE]) -> &'a mut T {
        unsafe { arr.get_unchecked_mut(self.0) }
    }
}

impl<const SIZE: usize> From<Cludex<SIZE>> for usize {
    fn from(i: Cludex<SIZE>) -> Self {
        i.0
    }
}

impl<const SIZE: usize> convert::TryFrom<usize> for Cludex<SIZE> {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < SIZE {
            Ok(Cludex(value))
        } else {
            Err(Error::OutOfBounds { value, size: SIZE })
        }
    }
}

impl<const SIZE: usize> fmt::Display for Cludex<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* Errors: */

/// Cludex errors enum
#[derive(Debug, Clone)]
pub enum Error {
    /// Tried to use a out-of-bounds value to create a cludex
    OutOfBounds {
        /// The out-of-bounds value that was provided at cludex
        /// creation
        value: usize,
        /// The `SIZE` of the cludex type
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
                "value {} is out-of-bounds for index index with size {}",
                value, size
            ),
        }
    }
}

/* Helper macros */

/// Implement Index and IndexMut for the provided array wrapper, base
/// type and array size
///
/// This macro "links" the cludex to the provided array wrapper by
/// implementing appropriate Index and IndexMut. The underlying
/// implementation uses `get_unchecked` and `get_unchecked_mut` to
/// avoid checking array bounds - which were already checked when the
/// cludex instance was instantiated.
#[macro_export]
macro_rules! impl_cludex_for {
    ($name:ty, $base: ty, $cludex:ty) => {
        impl std::ops::Index<$cludex> for $name {
            type Output = $base;
            fn index(&self, i: $cludex) -> &Self::Output {
                i.index_arr(&self.0)
            }
        }
        impl std::ops::IndexMut<$cludex> for $name {
            fn index_mut(&mut self, i: $cludex) -> &mut $base {
                i.index_arr_mut(&mut self.0)
            }
        }
    };
}
