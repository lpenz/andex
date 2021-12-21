[![CI](https://github.com/lpenz/andex/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/andex/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/andex/badge.svg?branch=main)](https://coveralls.io/github/lpenz/andex?branch=main)
[![crates.io](https://img.shields.io/crates/v/andex)](https://crates.io/crates/andex)
[![doc.rs](https://docs.rs/andex/badge.svg)](https://docs.rs/andex)


# andex

*andex* (Array iNDEX) is a zero-dependency rust crate that helps
us create strongly-typed, zero-cost, numerically bound array index
and the corresponding array type with the provided size. The index
is safe in the sense that an out-of-bounds value can't be created,
and the array type can't be indexed by any other types.

This is useful in scenarios where we have different arrays inside a
`struct` and we want reference members without holding proper
references that could "lock" the whole `struct`. It may also be useful
when programming an
[Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system).

And it's all done without requiring the use of any macros.

## Usage

### Creating the andex types

[`Andex`] is the index type and [`AndexableArray`] is the type of
the array wrapper.

The recommended approach to use andex is as follows:
- Create a unique empty type
  ```rust
  enum MyIdxMarker {}
  ```
- Create a type alias for the [`Andex`] type that's parameterized
  with that type:
  ```rust
  type MyIdx = Andex<MyIdxMarker, 12>;
  ```
- Create a type alias for the [`AndexableArray`] type that's
  indexed by the [`Andex`] alias created above:
  ```rust
  type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;
  // There is also a helper macro for this one:
  type MyOtherU32 = andex::array!(MyIdx, u32);
  ```

### Creating andex instances

When an andex is created, it knows *at compile time* the size of the
array it indexes, and all instances are assumed to be within bounds.

For this reason, it's useful to limit the way `Andex`'s are
created. The ways we can get an instance is:

- Via `new`, passing the value as a generic const argument:
  ```rust
  const first : MyIdx = MyIdx::new::<0>();
  ```
  This checks that the value is valid at compile time, as long as you
  use it to create `const` variables.

- Via `try_from`, which returns `Result<Andex, Error>` that has to be
  checked or explicitly ignored:
  ```rust
  if let Ok(first) = MyIdx::try_from(0) {
      // ...
  }
  ```

- Via `FIRST` and `LAST`:
  ```rust
  const first : MyIdx = MyIdx::FIRST;
  let last = MyIdx::LAST;
  ```

- By iterating:
  ```rust
  for idx in MyIdx::iter() {
      // ...
  }
  ```

The assumption that the instances can only hold valid values allows us
to use `get_unsafe` and `get_unsafe_mut` in the indexer
implementation, which provides a bit of optimization by preventing the
bound check when indexing.

### Creating andexable arrays

[`AndexableArray`] instances are less restrictive. They can be created
in several more ways:
- Using `Default` if the underlying type supports it:
  ```rust
  type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;

  let myu32 = MyU32::default();

  // We also have a helper macro that avoids repeating the size:
  type MyOtherU32 = andex::array!(MyIdx, u32);
  ```
- Using `From` with an appropriate array:
  ```rust
  let myu32 = MyU32::from([8; MyIdx::SIZE]);
  ```
- Collecting an iterator with the proper elements and size:
  ```rust
  let myu32 = (0..12).collect::<MyU32>();
  ```
  Note: `collect` panics if the iterator returns a different
  number of elements.

### Using andexable arrays

Besides indexing them with a coupled `Andex` instance, we can
also access the inner array by using `as_ref`, iterate it in a
`for` loop (using one of the `IntoIterator` implementations) or
even get the inner array by consuming the `AndexableArray`.

## Full example

```rust
use std::convert::TryFrom;
use std::error::Error;
use andex::*;

// Create the andex type alias:
//   First, we need an empty type that we use as a marker:
enum MyIdxMarker {}
//   The andex type takes the marker (for uniqueness)
//   and the size of the array as parameters:
type MyIdx = Andex<MyIdxMarker, 12>;

// Create the array wrapper:
type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;

// We can create other arrays indexable by the same Andex:
type MyF64 = AndexableArray<MyIdx, f64, { MyIdx::SIZE }>;

fn main() -> Result<(), Box<dyn Error>> {
    let myu32 = MyU32::default();

    // We can now only index MyU32 using MyIdx
    const first : MyIdx = MyIdx::new::<0>();
    println!("{:?}", myu32[first]);

    // Trying to create a MyIdx with an out-of-bounds value
    // doesn't work, this won't compile:
    // const _overflow : MyIdx = MyIdx::new::<30>();

    // Trying to index myu32 with a "naked" number
    // doesn't work, this won't compile:
    // println!("{}", myu32[0]);

    // We can create indexes via try_from with a valid value:
    let second = MyIdx::try_from(2);
    // ^ Returns a Result, which Ok(MyIdx) if the value provided is
    // valid, or an error if it's not.

    // We can also create indexes at compile-time:
    const third : MyIdx = MyIdx::new::<1>();

    // The index type has an `iter()` method that produces
    // all possible values in order:
    for i in MyIdx::iter() {
        println!("{:?}", i);
    }
    Ok(())
}
```

## Compile-time guarantees

This is the reason to use Andex instead of a plain array in the
first play, right? Below is a list of some of the compile-time
restrictions that we get.

- We can't index [`AndexableArray`] with a `usize`.

  The following code doesn't compile:

```rust
use andex::*;
enum MyIdxMarker {}
type MyIdx = Andex<MyIdxMarker, 12>;
type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;

fn main() {
    let myu32 = MyU32::default();

    // Error: can't index myu32 with a usize
    println!("{}", myu32[0]);
}
```

- We can't create a const [`Andex`] with an out-of-bounds value.

  The following code doesn't compile:

```rust
use andex::*;
enum MyIdxMarker {}
type MyIdx = Andex<MyIdxMarker, 12>;

fn main() {
    // Error: can't create out-of-bounds const:
    const myidx : MyIdx = MyIdx::new::<13>();
}
```

- We can't index [`AndexableArray`] with a different Andex, even when
  it has the same size. This is what using different markers gets
  us.

  The following code doesn't compile:

```rust
use andex::*;

enum MyIdxMarker {}
type MyIdx = Andex<MyIdxMarker, 12>;
type MyU32 = AndexableArray<MyIdx, u32, { MyIdx::SIZE }>;

enum TheirIdxMarker {}
type TheirIdx = Andex<TheirIdxMarker, 12>;
type TheirU32 = AndexableArray<TheirIdx, u32, { TheirIdx::SIZE }>;

fn main() {
    let myu32 = MyU32::default();
    let theirIdx = TheirIdx::FIRST;

    // Error: can't index a MyU32 array with TheirIdx
    println!("{}", myu32[theirIdx]);
}
```

## Alternatives

These alternatives may fit better cases where we need unbound indexes
(maybe for vector):

- [safe_index](https://crates.io/crates/safe_index)
- [typed-index-collections](https://crates.io/crates/typed-index-collections)


[`Andex`]: https://docs.rs/andex/0/andex/struct.Andex.html
[`AndexableArray`]: https://docs.rs/andex/0/andex/struct.AndexableArray.html

