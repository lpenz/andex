// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cludex<const SIZE: usize>(usize);

impl<const SIZE: usize> Cludex<SIZE> {
    pub const SIZE: usize = SIZE;
    pub const UNITS: [(); SIZE] = [(); SIZE];

    pub fn iter() -> impl Iterator<Item = Cludex<SIZE>> {
        (0..SIZE).into_iter().map(Cludex)
    }

    #[inline]
    pub const fn new<const N: usize>() -> Cludex<SIZE> {
        let _ = Cludex::<SIZE>::UNITS[N];
        Cludex(N)
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
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < SIZE {
            Ok(Cludex(value))
        } else {
            Err("value out of bounds")
        }
    }
}

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
