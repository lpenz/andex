// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Tests for arrays: */

pub struct MyIdxInner;
type MyIdx = Andex<MyIdxInner, 12>;

type MyArray = AndexableArray<MyIdx, u32, 12>;

#[test]
fn test_myarr() -> Result<()> {
    let mut m = MyArray::default();
    m[MyIdx::new::<2>()] = 5;
    m[MyIdx::try_from(2)?] = 5;
    for (num, i) in MyIdx::iter().enumerate() {
        m[i] = num as u32 + 20;
    }
    for (num, i) in MyIdx::iter().enumerate() {
        assert_eq!(m[i], num as u32 + 20);
    }
    let _ = MyIdx::iter().map(|i| i);
    println!("{:?}", m);
    Ok(())
}
