// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert;
use std::error;
use std::fmt;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cludex<const SIZE: usize>(usize);

impl<const SIZE: usize> Cludex<SIZE> {
    pub const SIZE: usize = SIZE;
    pub const ASSERT: [(); 1] = [(); 1];

    #[inline]
    pub const fn new<const N: usize>() -> Cludex<SIZE> {
        // Trick for compile-time check of N:
        let _ = Cludex::<SIZE>::ASSERT[(N >= SIZE) as usize];
        Cludex(N)
    }

    pub const fn into_inner(self) -> usize {
        self.0
    }

    pub fn iter() -> impl Iterator<Item = Cludex<SIZE>> {
        (0..SIZE).into_iter().map(Cludex)
    }

    pub fn index_arr<'a, T>(&self, arr: &'a [T; SIZE]) -> &'a T {
        unsafe { arr.get_unchecked(self.0) }
    }

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/* Errors: */

#[derive(Debug, Clone)]
pub enum Error {
    OutOfBounds { value: usize, size: usize },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
