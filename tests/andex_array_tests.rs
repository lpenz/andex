// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;

use std::convert::TryFrom;

use anyhow::Result;

/* Tests for arrays: */

pub struct MyIdxInner;
type MyIdx = Andex<MyIdxInner, 12>;

type MyArray = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;

type MyArray2 = andex::array!(MyIdx, u32);

pub struct NoTraits {}
type _MyArrayNoTraits = AndexableArray<MyIdx, NoTraits, { MyIdx::SIZE }>;

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
    for (num, i) in MyIdx::iter().enumerate() {
        m[&i] = num as u32 + 30;
    }
    for (num, i) in MyIdx::iter().enumerate() {
        assert_eq!(m[&i], num as u32 + 30);
    }
    let _ = MyIdx::iter().map(|i| i);
    println!("{:?}", m);
    Ok(())
}

#[test]
fn test_conversions() -> Result<()> {
    let mut myarray1 = MyArray::from([3; 12]);
    let array1 = myarray1.as_mut();
    for i in 0..12 {
        array1[i] = i as u32;
    }
    for i in MyIdx::iter() {
        assert_eq!(myarray1[i], usize::from(i) as u32);
    }
    let array2 = <[u32; 12]>::from(&myarray1);
    assert_eq!(&array2, myarray1.as_ref());
    let array3 = <[u32; 12]>::from(myarray1);
    assert_eq!(array3, array2);
    let myarray2 = MyArray::from(&array2);
    assert_eq!(&array2, myarray2.as_ref());
    for a in myarray2 {
        println!("{:?}", a);
    }
    let myarray3 = array3.iter().collect::<MyArray>();
    assert_eq!(myarray3.as_ref(), &array3);
    let myarray4 = array3.iter().cloned().collect::<MyArray>();
    assert_eq!(myarray4.as_ref(), &array3);
    let _myarray5 = myarray4.clone();
    let _myarray6 = *&myarray4;
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    let mut myarray = MyArray2::from([3; 12]);
    for item in &mut myarray {
        *item = 5;
    }
    for item in &myarray {
        assert_eq!(*item, 5);
    }
    for item in myarray {
        assert_eq!(item, 5);
    }
    Ok(())
}
