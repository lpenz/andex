// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Basic tests */

enum Marker {}
type C = Andex<Marker, usize, 3>;

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
    let k: C = C::try_from(1).unwrap();
    assert_eq!(usize::from(k), 1_usize);
    assert_eq!(usize::from(C::try_from(2)?), 2);
    assert!(C::try_from(3).is_err());
    let u = k.clone();
    assert_eq!(u, k);
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

#[test]
fn test_parse() {
    let c = "0".parse::<C>();
    assert!(c.is_ok());
    assert_eq!(usize::from(c.unwrap()), 0_usize);
    let c = "asdf".parse::<C>();
    if let Err(ref err) = c {
        println!("{}", err);
    }
    assert!(c.is_err());
    let c = "4".parse::<C>();
    if let Err(ref err) = c {
        println!("{}", err);
    }
    assert!(c.is_err());
}

#[test]
fn test_pair() {
    let f: C = C::first();
    assert_eq!(f.pair(), C::last());
    let f: C = C::LAST;
    assert_eq!(f.pair(), C::FIRST);
}

/* Iterator */

#[test]
fn test_iterator() {
    let mut it = C::iter();
    println!("{:?}", it);
    let first = it.next().unwrap();
    assert_eq!(usize::from(first), 0);
    let second = it.next().unwrap();
    assert_eq!(usize::from(second), 1);
    assert!(first < second);
    assert!(first <= first);
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
