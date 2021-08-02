// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_doc_code_examples)]

//! *andex* is a single-file, zero-dependency rust crate that helps us
//! create a strongly-typed, zero-cost, safe array index and use it to
//! index a custom array wrapper.
//!
//! This is specially useful in scenarios where we have different arrays
//! inside a `struct` and we want reference members without holding "hard"
//! references to those members.
//!
//! # Basic usage
//!
//! ```
//! use andex::*;
//! use andex::impl_andex_for;
//!
//! // Create the array wrapper; we can use the andex size already:
//! #[derive(Default)]
//! pub struct MyU32([u32; MyIdx::SIZE]);
//!
//! // Create the andex type alias:
//! type MyIdx = Andex<12>;
//!
//! // Use `impl_andex_for` to make it indexable:
//! impl_andex_for!(MyU32, u32, MyIdx);
//!
//! // We can use `impl_andex_for` with other wrappers too:
//! pub struct MyF64([f64; MyIdx::SIZE]);
//! impl_andex_for!(MyF64, f64, MyIdx);
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

/// Array index generic type
///
/// This is the type of the array index that receives the size of
/// the array as a const generic `SIZE` parameter.
///
/// Note: the maximum numerical value in the andex is `SIZE - 1`.
///
/// # Example
///
/// This should be used in a type alias, as such:
/// ```
/// use andex::*;
///
/// type MyIdx = Andex<12>;
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Andex<const SIZE: usize>(usize);

impl<const SIZE: usize> Andex<SIZE> {
    /// The `SIZE` parameter, which is the size of the array that this
    /// andex indexes.
    pub const SIZE: usize = SIZE;

    /// Create a new andex instance
    ///
    /// Bounds are checked at compile time when this is used to create
    /// `const` instances - that's why this is the recommended usage:
    /// ```
    /// use andex::*;
    /// type MyIdx = Andex<12>;
    ///
    /// const MYVALUE : MyIdx = MyIdx::new::<0>();
    /// ```
    /// And that's why the following doesn't compile:
    /// ```compile_fail
    /// use andex::*;
    /// type MyIdx = Andex<12>;
    ///
    /// const MYVALUE : MyIdx = MyIdx::new::<12>();
    /// ```
    #[inline]
    pub const fn new<const N: usize>() -> Andex<SIZE> {
        // Trick for compile-time check of N:
        const ASSERT: [(); 1] = [(); 1];
        let _ = ASSERT[(N >= SIZE) as usize];
        Andex(N)
    }

    /// Extracts the numeric value of the index, consuming it
    #[inline]
    pub const fn into_usize(self) -> usize {
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
    /// use andex::*;
    ///
    /// #[derive(Default)]
    /// pub struct Scores([u32; PlayerId::SIZE]);
    ///
    /// type PlayerId = Andex<12>;
    ///
    /// impl_andex_for!(Scores, u32, PlayerId);
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
    pub fn iter() -> impl Iterator<Item = Andex<SIZE>> {
        (0..SIZE).into_iter().map(Andex)
    }

    /// Indexes the provided array
    ///
    /// Used internally by the `Index` trait implementation.
    #[inline]
    pub fn index_arr<'a, T>(&self, arr: &'a [T; SIZE]) -> &'a T {
        unsafe { arr.get_unchecked(self.into_usize()) }
    }

    /// Mut-indexes the provided array
    ///
    /// Used internally by the `IndexMut` trait implementation.
    #[inline]
    pub fn index_arr_mut<'a, T>(&self, arr: &'a mut [T; SIZE]) -> &'a mut T {
        unsafe { arr.get_unchecked_mut(self.into_usize()) }
    }
}

impl<const SIZE: usize> From<Andex<SIZE>> for usize {
    fn from(i: Andex<SIZE>) -> Self {
        i.into_usize()
    }
}

impl<const SIZE: usize> convert::TryFrom<usize> for Andex<SIZE> {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < SIZE {
            Ok(Andex(value))
        } else {
            Err(Error::OutOfBounds { value, size: SIZE })
        }
    }
}

impl<const SIZE: usize> fmt::Display for Andex<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_usize())
    }
}

/* Errors: */

/// Andex errors enum
#[derive(Debug, Clone)]
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
/// This macro "links" the andex to the provided array wrapper by
/// implementing appropriate Index and IndexMut. The underlying
/// implementation uses `get_unchecked` and `get_unchecked_mut` to
/// avoid checking array bounds - which were already checked when the
/// andex instance was instantiated.
#[macro_export]
macro_rules! impl_andex_for {
    ($name:ty, $base: ty, $andex:ty) => {
        impl std::ops::Index<$andex> for $name {
            type Output = $base;
            fn index(&self, i: $andex) -> &Self::Output {
                i.index_arr(&self.0)
            }
        }
        impl std::ops::IndexMut<$andex> for $name {
            fn index_mut(&mut self, i: $andex) -> &mut $base {
                i.index_arr_mut(&mut self.0)
            }
        }
    };
}

/// Implement `Deref` for the wrapped array, making the wrapper behave
/// like it except for only being indexable with the andex.
///
/// # Example
///
/// ```
/// use andex::*;
/// use andex::impl_andex_for;
/// use andex::impl_deref_for;
///
/// #[derive(Default)]
/// pub struct MyU32([u32; MyIdx::SIZE]);
/// type MyIdx = Andex<12>;
/// impl_andex_for!(MyU32, u32, MyIdx);
///
/// // Use `impl_deref_for` to make MyU32 behave like an array
/// impl_deref_for!(MyU32, u32, MyIdx);
///
/// fn example() {
///     let myu32 = MyU32::default();
///     // We can now use `iter` directly in the wrapper:
///     for value in myu32.iter() {
///         println!("value {}", value);
///     }
///     // But still only index with an andex:
///     println!("{}", myu32[MyIdx::new::<0>()]);
/// }
#[macro_export]
macro_rules! impl_deref_for {
    ($name:ty, $base: ty, $andex:ty) => {
        impl std::ops::Deref for $name {
            type Target = [$base; <$andex>::SIZE];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
