// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Basic tests */

enum Marker {}
type C = Andex<Marker, 3>;

#[test]
fn test_basic() -> Result<()> {
    for i in C::iter() {
        assert!(usize::from(i) < 3_usize);
    }
    let is = C::iter().map(usize::from).collect::<Vec<_>>();
    assert_eq!(is, vec![0, 1, 2]);
    let i = C::default();
    assert_eq!(usize::from(i), 0_usize);
    const J: C = C::new::<2>();
    assert_eq!(usize::from(J), 2_usize);
    let k: C = C::new::<1>();
    assert_eq!(usize::from(k), 1_usize);
    assert_eq!(usize::from(C::try_from(2)?), 2);
    assert!(C::try_from(3).is_err());
    let _u = k.clone();
    Ok(())
}

#[test]
#[should_panic]
fn test_oob1() {
    let u: C = C::new::<5>();
    assert_eq!(usize::from(u), 5_usize);
}

// // This doesn't compile, which is correct:
// #[test]
// #[should_panic]
// fn test_oob2() {
//     const u: C = C::new::<5>();
//     assert_eq!(usize::from(u), 5_usize);
// }

#[test]
fn test_try_from() {
    let result = C::try_from(5);
    assert!(result.is_err());
    if let Err(ref err) = result {
        println!("{}, {:?}", err, err);
        let _ = err.clone();
    }
}

/* Iterator */

#[test]
fn test_iterator() {
    let mut it = C::iter();
    println!("{:?}", it);
    assert_eq!(usize::from(it.next().unwrap()), 0);
    assert_eq!(usize::from(it.next().unwrap()), 1);
    assert_eq!(usize::from(it.next().unwrap()), 2);
    assert!(it.next().is_none());
    assert!(it.next().is_none());
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