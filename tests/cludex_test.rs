// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use cludex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Basic tests */

#[test]
fn test_basic() -> Result<()> {
    for i in Cludex::<3>::iter() {
        assert!(usize::from(i) < 3_usize);
    }
    let i = Cludex::<3>::default();
    assert_eq!(usize::from(i), 0_usize);
    const J: Cludex<3> = Cludex::<3>::new::<2>();
    assert_eq!(usize::from(J), 2_usize);
    let k: Cludex<3> = Cludex::<3>::new::<1>();
    assert_eq!(usize::from(k), 1_usize);
    Ok(())
}

#[test]
#[should_panic]
fn test_oob1() {
    let u: Cludex<3> = Cludex::<3>::new::<5>();
    assert_eq!(usize::from(u), 5_usize);
}

// This doesn't compile, which is correct:
// #[test]
// #[should_panic]
// fn test_oob2() {
//     const u: Cludex<3> = Cludex::<3>::new::<5>();
//     assert_eq!(usize::from(u), 5_usize);
// }

/* Tests for arrays: */

#[derive(Default)]
struct Marr([u32; 5]);
type Mcludex = Cludex<5>;
impl_cludex_for!(Marr, u32, Mcludex);

#[test]
fn test_marr() -> Result<()> {
    let mut m = Marr::default();
    m[Cludex::new::<2>()] = 5;
    for (num, i) in Mcludex::iter().enumerate() {
        m[i] = num as u32 + 20;
    }
    for (num, i) in Mcludex::iter().enumerate() {
        assert_eq!(m[i], num as u32 + 20);
    }
    Ok(())
}

#[test]
fn test_try_from() {
    let result = Cludex::<3>::try_from(5);
    if let Err(ref err) = result {
        println!("{}, {:?}", err, err);
        let _ = err.clone();
    } else {
        panic!("wrong error!");
    }
}

/* Test automatic traits */

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Cludex<7>>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Cludex<5>>();
}
