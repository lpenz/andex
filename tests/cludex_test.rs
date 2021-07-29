// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use cludex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Basic tests */

type C = Cludex<3>;

#[test]
fn test_basic() -> Result<()> {
    for i in C::iter() {
        assert!(usize::from(i) < 3_usize);
    }
    let i = C::default();
    assert_eq!(usize::from(i), 0_usize);
    const J: C = C::new::<2>();
    assert_eq!(usize::from(J), 2_usize);
    let k: C = C::new::<1>();
    assert_eq!(usize::from(k), 1_usize);
    Ok(())
}

#[test]
#[should_panic]
fn test_oob1() {
    let u: C = C::new::<5>();
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
impl_deref_for!(Marr, u32, Mcludex);

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
    let _ = m.iter().map(|i| i);
    Ok(())
}

#[test]
fn test_try_from() {
    let result = C::try_from(5);
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
    assert_send::<C>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<C>();
}
