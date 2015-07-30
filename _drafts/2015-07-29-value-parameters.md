---
layout: post
title:  "A praise for associated types in Rust"
categories: rust
---

I've [already talked](/rust/typechecked-matrix) about how much I love value
parameters and would like them integrated in the language. This is a follow-up
on these rants, except that this time, I'm going to show a few things that we
could do with such a construct. On the menu:

* Integers bound-checked at compile-time
* Super-efficient matrices
* __TODO__

Let's jump right into it!


## About the syntax

The syntax that I will use here simply extends the already-existing one:

```rust
struct MyType<TypeParam [: Trait], 'lifetime_param [: 'longer], VALUE_PARAM : Type> {
    // ...
}
```

Notice how type parameters might depend on traits, lifetime parameters (with
their prefixed apostrophe) might depend on other lifetimes, and value parameters
must depend on exactly one type &mdash; which is how the compiler actually
determines whether it's a value or a type parameter. We use the full uppercase
convention used by associated constants.

I will use `int` as the type of the value parameter for the rest of this article.


## Bounded integers

One thing that would be nice is to verify, at compile-time, that integers exist
in a specific interval. For example, one could statically check that accessing
an element of an array is safe. I will stick with an `isize` attribute for the
sake of clarity. I also never actually verify that `HIGH > LOW`. The type
definition is quite simple:

```rust
/// Integer x in [LOW, HIGH)
#[derive(Clone, Copy, Eq, PartialEq, ...)]
struct Integer<LOW: int, HIGH: int>(i32);
```

One would implement type-casting into different integer types. I will not do it,
though, because it would uselessly complicate things. What I would like to look
at, though would be the bound-checking functions:

```rust
impl<LOW: int, HIGH: int> Integer<LOW, HIGH> {
    fn new(val: i32) -> Result<Integer, String> {
        if val >= LOW && val < HIGH {
            Ok(Integer(val))
        } else {
            Err(format!("{} not included in range [{}, {})", val, LOW, HIGH))
        }
    }

    fn restrict<NEW_LOW: int, NEW_HIGH: int>(self) -> Result<Integer<NEW_LOW, NEW_HIGH>, String> {
        if self.0 >= NEW_LOW && self.0 < NEW_HIGH {
            Integer(self.0)
        } else {
            Err(format!("{} not included in range [{}, {})", val, NEW_LOW, NEW_HIGH))
        }
    }
}
```

We cannot be sure that a user input is contained within a certain range;
similarly, when we change the range over which the input is defined, the value
previously held might become out of bounds. We take care of this eventuality by
returning a `Result` in both functions.

Now that we've gone through that, we can define a few methods that directly
manipulate the integers without having to verify that the bounds are valid:

```rust
impl<L1: int, H1: int, L2:int, H2: int> Add<Integer<L2, H2>> to Integer<H1, L1> {
    type Output = Integer<L1 + L2, H1 + H2 - 1>;
    fn add(self, rhs: Integer<L2, H2>) -> Self::Output {
        Integer(self.0 + rhs.0)
    }
}

impl<L1: int, H1: int, L2:int, H2: int> Mul<Integer<L2, H2>> to Integer<H1, L1> {
    type Output = Integer<L1 * L2, H1 * H2 - H1 - H2 + 1>;
    fn mul(self, rhs: Integer<L2, H2>) -> Self::Output {
        Integer(self.0 * rhs.0)
    }
}

impl<L: int, H: int> Neg for Integer<L, H> {
    type Output = Integer<(-H) - 1, (-L) + 1>;
    fn neg(self) -> Self::Output {
        Integer(-self.0)
    }
}
```

As you might notice, except for the type annotations, these operations are
pretty straightforward.
