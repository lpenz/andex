// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! andex module
//!
//! andex code is structure in a way that allows users to copy this
//! file to their projects and use andex as its own module, without a
//! crate dependency.

use std::cmp;
use std::convert;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num;
use std::ops;
use std::str;

/* Andex index type */

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

    /// The first possible value.
    pub const FIRST: Andex<M, SIZE> = Andex(PhantomData, 0);

    /// The last possible value.
    pub const LAST: Andex<M, SIZE> = Andex(PhantomData, SIZE - 1);

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
        #[allow(clippy::no_effect)]
        ASSERT[(N >= SIZE) as usize];
        Andex(PhantomData, N)
    }

    /// Returns the pair of the provided Andex.
    ///
    /// The "pair" is the element that is at the same distance from
    /// the center. This definition is useful in some contexts. For
    /// instance, the pair of [`Self::FIRST`] is [`Self::LAST`].
    #[inline]
    pub const fn pair(self) -> Self {
        Andex(PhantomData, SIZE - self.1 - 1)
    }

    /// Return the next Andex in sequence, or None if it's the last one.
    #[inline]
    pub fn next(self) -> Option<Self> {
        let i = usize::from(self);
        if i < SIZE - 1 {
            Some(Andex(PhantomData, i + 1))
        } else {
            None
        }
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
    /// for i in PlayerId::iter() {
    ///     println!("{}", i);
    /// }
    /// ```
    pub fn iter() -> AndexIterator<M, SIZE> {
        AndexIterator::<M, SIZE>::default()
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

impl<M, const SIZE: usize> PartialEq for Andex<M, SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<M, const SIZE: usize> Eq for Andex<M, SIZE> {}

impl<M, const SIZE: usize> PartialOrd for Andex<M, SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M, const SIZE: usize> Ord for Andex<M, SIZE> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1)
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

impl<M, const SIZE: usize> str::FromStr for Andex<M, SIZE> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(usize::from_str(s)?)
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
/// for i in PlayerId::iter() {
///     println!("{}", i);
/// }
/// ```
pub struct AndexIterator<M, const SIZE: usize>(Option<Andex<M, SIZE>>);

impl<M, const SIZE: usize> fmt::Debug for AndexIterator<M, SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AndexIterator({:?})", self.0)
    }
}

impl<M, const SIZE: usize> Default for AndexIterator<M, SIZE> {
    fn default() -> Self {
        AndexIterator(Some(Andex::<M, SIZE>::default()))
    }
}

impl<M, const SIZE: usize> Iterator for AndexIterator<M, SIZE> {
    type Item = Andex<M, SIZE>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.0.take() {
            self.0 = i.next();
            Some(i)
        } else {
            None
        }
    }
}

/* Array wrapper */

/// Array wrapper indexable by the provided Andex type.
///
/// Example:
///
/// ```
/// use andex::*;
///
/// enum MyIdxMarker {}
/// type MyIdx = Andex<MyIdxMarker, 12>;
///
/// // Create the array wrapper:
/// type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
///
/// // We can create other arrays with the same Andex type:
/// type MyF64 = AndexableArray<MyIdx, f64, { MyIdx::SIZE }>;
///
/// // Create a default array:
/// let myu32 = MyU32::default();
/// // Print the first element:
/// const first : MyIdx = MyIdx::new::<0>();
/// println!("{:?}", myu32[first]);
/// // Iterate and print all elements:
/// for i in MyIdx::iter() {
///     println!("{:?}", myu32[i]);
/// }
/// // Print the whole array
/// println!("{:?}", myu32);
/// ```
#[derive(Debug)]
pub struct AndexableArray<A, Item, const SIZE: usize>(PhantomData<A>, [Item; SIZE]);

/// Helper macro that creates an AndexableArray from an Andex
///
/// This macro just uses the Andex argument to figure out the array
/// size, so that we don't have to repeat it here.
///
/// Example:
/// ```
/// use andex::*;
///
/// enum MyIdxMarker {};
/// type MyIdx = Andex<MyIdxMarker, 12>;
///
/// // Create the array wrapper with the macro:
/// type MyU32 = andex_array!(MyIdx, u32);
/// ```
#[macro_export]
macro_rules! andex_array {
    ($andex: ty, $item: ty) => {
        $crate::AndexableArray<$andex, $item, { <$andex>::SIZE }>
    };
}

impl<A, Item, const SIZE: usize> AndexableArray<A, Item, SIZE> {
    /// Returns an iterator over the `AnexableArray`.
    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.1.iter()
    }
}

impl<A, Item: Copy, const SIZE: usize> Clone for AndexableArray<A, Item, SIZE> {
    fn clone(&self) -> Self {
        AndexableArray::<A, Item, SIZE>::from(self.1)
    }
}

impl<A, Item: Copy, const SIZE: usize> Copy for AndexableArray<A, Item, SIZE> {}

impl<A, Item: Default + Copy, const SIZE: usize> Default for AndexableArray<A, Item, SIZE> {
    fn default() -> Self {
        AndexableArray(Default::default(), [Default::default(); SIZE])
    }
}

impl<A, Item, const SIZE: usize> ops::Index<Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    type Output = Item;
    fn index(&self, index: Andex<A, SIZE>) -> &Self::Output {
        index.index_arr(&self.1)
    }
}

impl<A, Item, const SIZE: usize> ops::IndexMut<Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    fn index_mut(&mut self, index: Andex<A, SIZE>) -> &mut Item {
        index.index_arr_mut(&mut self.1)
    }
}

impl<A, Item, const SIZE: usize> ops::Index<&Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    type Output = Item;
    fn index(&self, index: &Andex<A, SIZE>) -> &Self::Output {
        index.index_arr(&self.1)
    }
}

impl<A, Item, const SIZE: usize> ops::IndexMut<&Andex<A, SIZE>>
    for AndexableArray<Andex<A, SIZE>, Item, SIZE>
{
    fn index_mut(&mut self, index: &Andex<A, SIZE>) -> &mut Item {
        index.index_arr_mut(&mut self.1)
    }
}

impl<A, Item, const SIZE: usize> convert::AsRef<[Item; SIZE]> for AndexableArray<A, Item, SIZE> {
    fn as_ref(&self) -> &[Item; SIZE] {
        &self.1
    }
}

impl<A, Item, const SIZE: usize> convert::AsMut<[Item; SIZE]> for AndexableArray<A, Item, SIZE> {
    fn as_mut(&mut self) -> &mut [Item; SIZE] {
        &mut self.1
    }
}

impl<A, Item, const SIZE: usize> From<[Item; SIZE]> for AndexableArray<A, Item, SIZE> {
    fn from(array: [Item; SIZE]) -> Self {
        Self(PhantomData, array)
    }
}

impl<A, Item, const SIZE: usize> From<&[Item; SIZE]> for AndexableArray<A, Item, SIZE>
where
    Item: Copy,
{
    fn from(array: &[Item; SIZE]) -> Self {
        Self(PhantomData, *array)
    }
}

impl<A, Item, const SIZE: usize> From<AndexableArray<A, Item, SIZE>> for [Item; SIZE]
where
    Item: Copy,
{
    fn from(andexable_array: AndexableArray<A, Item, SIZE>) -> [Item; SIZE] {
        andexable_array.1
    }
}

impl<A, Item, const SIZE: usize> From<&AndexableArray<A, Item, SIZE>> for [Item; SIZE]
where
    Item: Copy,
{
    fn from(andexable_array: &AndexableArray<A, Item, SIZE>) -> [Item; SIZE] {
        andexable_array.1
    }
}

// impl<A, Item, const SIZE: usize> IntoIterator for AndexableArray<A, Item, SIZE> {
//     type Item = Item;
//     type IntoIter = std::array::IntoIter<Item, SIZE>;
//     fn into_iter(self) -> Self::IntoIter {
//         IntoIterator::into_iter(self.1)
//     }
// }

impl<'a, A, Item, const SIZE: usize> IntoIterator for &'a AndexableArray<A, Item, SIZE> {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.1.iter()
    }
}

impl<'a, A, Item, const SIZE: usize> IntoIterator for &'a mut AndexableArray<A, Item, SIZE> {
    type Item = &'a mut Item;
    type IntoIter = std::slice::IterMut<'a, Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.1.iter_mut()
    }
}

impl<A, Item, const SIZE: usize> core::iter::FromIterator<Item> for AndexableArray<A, Item, SIZE> {
    fn from_iter<I: core::iter::IntoIterator<Item = Item>>(intoiter: I) -> Self {
        let mut andexable = AndexableArray::<A, MaybeUninit<Item>, SIZE>(PhantomData, unsafe {
            std::mem::MaybeUninit::uninit().assume_init()
        });
        let mut iter = intoiter.into_iter();
        for item in &mut andexable {
            if let Some(fromiter) = iter.next() {
                item.write(fromiter);
            } else {
                panic!("iterator too short for andexable type");
            }
        }
        if iter.next().is_some() {
            panic!("iterator too long for andexable type");
        }

        unsafe { std::mem::transmute_copy::<_, AndexableArray<A, Item, SIZE>>(&andexable) }
    }
}

impl<'a, A, Item: 'a + Copy, const SIZE: usize> core::iter::FromIterator<&'a Item>
    for AndexableArray<A, Item, SIZE>
{
    fn from_iter<I: core::iter::IntoIterator<Item = &'a Item>>(intoiter: I) -> Self {
        let mut andexable = AndexableArray::<A, MaybeUninit<Item>, SIZE>(PhantomData, unsafe {
            std::mem::MaybeUninit::uninit().assume_init()
        });
        let mut iter = intoiter.into_iter();
        for item in &mut andexable {
            if let Some(&fromiter) = iter.next() {
                item.write(fromiter);
            } else {
                panic!("iterator too short for andexable type");
            }
        }
        if iter.next().is_some() {
            panic!("iterator too long for andexable type");
        }

        unsafe { std::mem::transmute_copy::<_, AndexableArray<A, Item, SIZE>>(&andexable) }
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
/// enum MyIdxMarker {}
/// type MyIdx = Andex<MyIdxMarker, 12>;
///
/// println!("{:?}", MyIdx::try_from(15_usize));
/// ```
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
    /// Underlying ParseIntError from integer parsing
    ParseIntError(num::ParseIntError),
}

impl error::Error for Error {}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

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
            Error::ParseIntError(err) => write!(f, "{}", err),
        }
    }
}
