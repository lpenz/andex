[![CI](https://github.com/lpenz/andex/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/andex/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/andex/badge.svg?branch=main)](https://coveralls.io/github/lpenz/andex?branch=main)

# andex

*andex* (Array iNDEX) is a single-file, zero-dependency, rust
crate that helps us create a strongly-typed, zero-cost, safe array
index and the corresponding array type.

This is specially useful in scenarios where we have different arrays
inside a `struct` and we want reference members without holding "hard"
references. May also be useful when programming an
[Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system).


## Basic usage

```rust
use andex::*;
use andex::impl_andex_for;

// Create the type alias:
type MyIdx = Andex<12>;

// Create the array wrapper:
#[derive(Default)]
pub struct MyU32([u32; MyIdx::SIZE]);

// Use `impl_andex_for` to make it indexable:
impl_andex_for!(MyU32, u32, MyIdx);

fn example() {
    // Iterate:
    for i in MyIdx::iter() {
        println!("{:?}", i);
    }
    // Generate first index at compile time:
    const first = MyIdx::new::<0>();
    // Index the collection:
    let myu32 = MyU32::default();
    const first = MyIdx::new::<0>();
    println!("{:?}", myu32[first]);
}
```


## Creating index instances

When an andex is created, it knows *at compile time* the size of the
array it indexes, and all instances are assumed to be within bounds.

For this reason, it's useful to limit the way andex's are
created. The ways we can get an instance is:

- Via `new`, passing the value as a generic const argument:
  ```rust
  const first = Idx::new::<0>::()
  ```
  This checks that the value is valid at compile time, as long as you
  use it to create `const` variables.

- Via `try_from`, which returns `Result<Andex,Error>` that has to be
  checked or explicitly ignored:
  ```rust
  if let Ok(first) = Idx::try_from(0) {
      ...
  }
  ```

- By iterating:
  ```rust
  for idx in Idx::iter() {
      ...
  }
  ```

The assumption that the instances can only hold valid values allows us
to use `get_unsafe` and `get_unsafe_mut` in the indexer
implementation, which provides a bit of optimization by preventing the
bound check when indexing.


## Creating the indexable array wrapper

To use the index, we first create the array wrapper, and then use the
`impl_andex_for` to make it indexable by the andex:

```rust
pub struct ArrayWrapper([u32; 12])

impl_andex_for!(ArrayWrapper, u32, Idx);
```

This macro creates the appropriate `Index` and `IndexMut`
implementations. These implementations use `get_unchecked` and
`get_unchecked_mut` under the wraps, as the array bounds are checked
when the andex instance is created and we don't have to check them
again.

Note: the user is responsible for making the limit of the andex and
the wrapper equal.


## Full example

```rust
use andex::*;
use std::convert::TryFrom;

/// A player with score
#[derive(Default)]
pub struct Player {
    pub score: i32,
}

/// All players in the game
#[derive(Default)]
pub struct Players([Player; 4]);

/// The player identifier
type PlayerId = Andex<4>;

// Make Players[PlayerId] work
impl_andex_for!(Players, Player, PlayerId);

/// The game state
#[derive(Default)]
pub struct Game {
    pub players: Players,
}

impl Game {
    pub fn play(&mut self) {
        // Increment all scores
        for playerid in PlayerId::iter() {
            self.players[playerid].score += 1;
        }
        // Increment the first player's score:
        self.players[PlayerId::new::<0>()].score += 1;
        // ^ note that we had to use a const generic parameter so that
        // the index bound is checked at compile time.
        // If we want to create an index at run time, we have to use
        // TryInto/TryFrom, which returns Result:
        if let Ok(playerid) = PlayerId::try_from(4) {
            self.players[playerid].score = 3;
        }
        // ^ This "if" is never true because 4 >= 4, which is out-of-bounds.
    }
}
```


## Alternatives

These alternatives may fit better cases where we need unbound indexes
(into vector, for instance):

- [safe_index](https://crates.io/crates/safe_index)
- [typed-index-collections](https://crates.io/crates/typed-index-collections)

