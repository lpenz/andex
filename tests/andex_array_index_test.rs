// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Tests for arrays: */

#[derive(Default)]
struct Marr([u32; 5]);
type Mandex = Andex<5>;
impl_andex_for!(Marr, u32, Mandex);
impl_deref_for!(Marr, u32, Mandex);

#[test]
fn test_marr() -> Result<()> {
    let mut m = Marr::default();
    m[Mandex::new::<2>()] = 5;
    m[Mandex::try_from(2)?] = 5;
    for (num, i) in Mandex::iter().enumerate() {
        m[i] = num as u32 + 20;
    }
    for (num, i) in Mandex::iter().enumerate() {
        assert_eq!(m[i], num as u32 + 20);
    }
    let _ = m.iter().map(|i| i);
    Ok(())
}
