---
layout: post
title:  "The art of Rust Thievery"
---

One of the reasons why we love Rust so much is because the language doesn't
_really_ innovate, <del>except for dropck</del> (shh!). Algebraic data types?
They're decades old! So are pattern matching, immutability by default, affine
type-systems, etc. Heck! even lifetimes were mostly stolen from Cyclone!

And this is great! By coming second, Rust manages to strengthen a lot of those
ideas and lets them fit together nicely. But y'know what's even better than
having a language inspired by a lot of good ideas? Having a language which
allows you to get some of that sweet sweet sugar for yourself!

Today's victim shall be Haskell, from whom we're going to vehemently plagiarize
its list building syntax, __TODO__.


## Preppin' the curry

First, let's get real: iterators are _awesome_ but, sometimes, you just want to
filter through an iterable or __TODO__ and, in those cases, you might feel like
writing the whole thing is a little long-winded. For instance, if you only knew
Rust, this might feel pretty satisfying:

```rust
// carresPairs = lazy [4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
let evenSquared = (1..)
    .filter(|x| x % 2 == 0).map(|x| x * x)
    .take(10);
```

If you're coming from Haskell, however, you're more likely to write something
like this:

```haskell
let evenSquared =
    take 10 [ x * x | x <- [1..], x `rem` 2 == 0 ]
```

Wouldn't it be nice if Rust allowed us to use the rest of both worlds &mdash;
nice abstractions _and_ performance. Oh, wait! It does. Here's how we would
compute the same thing using a macro:

```rust
hs_arr_into![ x <- (0..), x % 2 == 0 => x * x ].take(10)
```

Notice that, because we cannot use `|` after an expression, I put the result at
the end instead, [ECMAScript-style](https://developer.mozilla.org/fr/docs/Web/JavaScript/Reference/Op%C3%A9rateurs/Compr%C3%A9hensions_de_tableau)
(and I'm guessing that it would be trivial to implement the ES7 syntax instead).

If you got some experience with macros in Rust, you can probably handle this
case in a few minutes:

```rust
macro_rules! hs_arr_into {
    // The trivial case, where we just map over some iterable.
    [ $i:ident <- $src:expr => $res:expr ] => {
        $src.into_iter()
            .map(|$i| $res)
    };

    [ $i:ident <- $src:expr, $( $cond:expr ),* => $res:expr ] => {
        $src.into_iter()
            $(.filter(|$i| $cond))*
            .map(|$i| $res)
    }
}
```

Things start to get messy once we start iterating in parallel over many
iterators or when we try to return an iterator of iterators.

<!--
TODO: Doesn't work because
* `a <- b` is recognized as _placement syntax_ (use `in` instead)
* concat_ident! acts weird
* nested $( )* act weird    
-->

Final example (from LearnYouAHaskell)

```haskell
let rightTriangles =
    [ (a,b,c) | c <- [1..10], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2]
```

```rust
let rightTriangles =
    [ c in [1..10], b in [1..c], a in [1..b], a*a + b*b == c*c => (a, b, c) ]
```
