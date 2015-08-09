---
layout: post
title:  "Making Rust smarter"
categories: rust
---

After writing my articles on [type-checked matrices](/rust/typechecked-matrix)
and [ways to visualize Rust's type-system](/rust/type-system/), I started to
think about how one might encode the state of his data in the type-system. If
it were a thing, then one could statically check most properties of his program.

Say you are modeling a system which may be in any one of a few discreet states,
or collections of states. Your objective is to _measure_ said state. Once this
is done, you would like to manipulate it, however not all states can undergo the
same operations. This is similar to the way that the type-system _forces you_ to
verify that a reference is non-null before accessing the borrowed value through
the `Option` type.

A first approach might be to declare an `enum` wrapping the possible states that
the system might be in. Take, for example, an articulation which may only be in
an angle between 0° and 45°. Then, you could have three types, each representing
a distinct family of states, wrapped by `Angle`.

```rust
/// Angle = 0°
struct LowerBound;

/// Angle = 45°
struct HigherBound;

/// Angle in (0°, 45°)
struct InBetween(i32);

enum Angle {
    LowerBound(LowerBound),
    HigherBound(HigherBound),
    InBetween(InBetween)
}
```

As a result of the way in which Rust implements discriminated unions, an object
of type `Angle` is actually pretty small; two of the structures, in fact, do not
contain any data. Also note that `HigherBound` (the `struct`) is different from
`Angle::HigherBound` (the `enum` variant).

The idea is that, once you get to the lower bound, you cannot reduce the angle
any more, and once you get to the higher bound, you may not increase it; both of
which should be statically checked. To do this, we may use traits:

```rust
trait CanIncrease {
    fn increase(self) -> Angle;
}

trait CanDecrease {
    fn decrease(self) -> Angle;
}
```

Notice how these methods actually _move_ `self`. It would be trivial to derive
`Copy` for `Angle`, but we don't because it allows us to statically verify that
nobody is still referring to the old state. We _could_ pass an `&mut` reference
instead &mdash; the result would be the same &mdash; however our data types are
incredibly thin. As such, using a reference may actually _hurt_ the performance.

As long as we make sure that our measurement functions are correct, we can
safely implement `CanIncrease` and `CanDecrease` for the wrapped structures:

```rust

```

You might say that this is a lot of overhead for little benefit, however recall
that it is now _impossible_ to increase the value of the angle once it is maxed
out (it cannot crash, because it won't even _compile_).
