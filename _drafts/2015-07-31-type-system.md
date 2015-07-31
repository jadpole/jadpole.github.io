---
layout: post
title:  "Visualizing Rust's type-system"
categories: rust
---

In a week, I'm giving a talk at the
[Montréal Rust Language Meetup](http://www.meetup.com/fr/Montreal-Rust-Language-Meetup/events/224148410/),
called "Rust after borrowing" and, as I'm searching for my words, I thought that
putting them down on a screen could ease the process.

The talk is going to be about Rust; more specifically, about medium-to-advanced
type-system business, such as dynamic dispatch, phantom types and higher-order
functions. While doing so, I'd like to share some methods of visualizing how the
different parts of Rust's type-system fit together.

And I thought that I would tackle that last part right now.


## Type hierarchy, Venn-style

The most flagrant example of Rust's minimalism is perhaps its type-system. It
traded classical OOP for a more _functional_ approach based solely around the
concept of traits. It is through traits that one defines the interface of his
data structures, overloads operators, marks concurrent types, and handles
dynamic dispatch. One concept to rule them all!

I assume that you know what traits are, so I won't go much further in there.
What I would like to ask you, though, is _what does the colon mean_? What are
you _really_ saying when you're writing `MyTrait: Add + Copy`? Well, let us
first consider the set of all possible __values__ for which `let x: i32` means
\\(x \in \mathbf i32\\).

This is trivial, yet things get interesting when you start to think about the
properties of __traits__ in this model. The _obvious_ approach would be to treat
traits as subsets of the collection of possible values. Within this
representation, traits start to behave in intuitive and powerful ways:

![Trait _addition_ using Venn diagram](/images/talk-1-1.png)

In this context, we have two traits, `A` and `B`, where each defines a certain
specification. When one is asking for a type to implement two traits, what we
_really_ mean is that the values of said type exist in the intersection between
both traits. In other words, `T: A + B` _really_ means \\(T \subset A \cap B\\).

What about traits that depend on other traits? Same thing! `C: A + B` _really_
means that every type implementing \\(C\\) must live within the intersection
\\(A \cap B\\); that is, \\(C \subset A \cap B\\). The two statements are
equivalent. Graphically, it is expressed by \\(C\\) being _contained_ by the
intersection:

![Trait dependency using Venn diagram](/images/talk-1-2.png)

You should start to see the pattern. A colon means _subset of_, while a plus
sign means _intersection of_.

According to this model, if I declare \\(T \subset C\\), it is implied that
\\(T \subset A \cap B\\). Therefore, the compiler should force us to implement
`A` and `B`, which is exactly what it does. Whether you view trait dependencies
as forming a tree or a Venn diagram, the result is the same.

The approach using sets, however, is _much_ simpler to think about and it is
possible because of the way __types__ work. Rust's adoption of the principle of
_composition over inheritance_ means that types are pretty much independent. As
such, you could say that they represent the smallest possible subset allowed by
the language; a type can contain _elements_, but it cannot contain other traits
or tangible types.

![Types represented by distinct subsets](/images/talk-1-3.png)

You should note the "tangible": this is because generic variants are different
types in this mental framework, like they are in compiled code. That way, they
can be in different trait subsets through conditional `impl`. This also means
that, given \\(sum : \mathbf{Vec\ i32} \rightarrow \mathbf{i32}\\), you cannot
map a `Vec<String>`: the two sets are necessarily disjoint.

We also consider that whatever methods are associated to a specific type make up
their own _implicit_ trait; types without any method map to marker traits
similar to `Sync`. The main difference is that _type traits_ typically implement
the `Sized` trait, which we generally omit because, well, it contains most
types. For example (where `MyType` does not implement `Hash`):

![Option defaults + Hash](/images/talk-1-4.png)

In this picture, `String`, `i32` and `Option<i32>` have access to `Hash::*`
while `Option<MyType>` and `Option<i32>` have access to `Option::*`. You can
notice that, because `Option<i32>` has access to both, it lives within the
intersection of `Hash` and `Option`. If it were to implement yet another trait
which depends on both (in this case, through conditional implementation), then
it would lay inside a subset of \\(A \cap B\\).

You could say that generics allow to statically generate types and that the
content of the `impl` block is _really_ just an anonymous trait which is
automatically implemented for all such types.

Now, this _diagram_ thing might make the type-system look even more daunting
then it already did. However, when compared to Ruby's mental model, for example,
we can see that Rust's is _much_ simpler to think about. It just asks you to be
super-duper explicit about everything &mdash; which is necessary when you want
either safety or performance, and even more so when you want both.

From our model, one could say that all types are created equal: there is no such
thing as a _parent_ or a _child_ record &mdash; just subsets in a pool of
possible value. Although it takes time to get used to this, it is much simpler
than traditional OOP. You do not have to worry about constructors, inheritance,
and all of these other things that we've became so accustomed to.

Before moving on, you should probably wait for everything to sink in, because
we're going to level up and review our model as we explore static and dynamic
dispatch and conditional `impl`, and discuss a method through which Rust might
be able to add inheritance-like code-reuse constructs while keeping its
simplistic design.


## Extending the model

What about static and dynamic dispatch? Well, let's tackle __static dispatch__
first. Say you define:

```rust
fn foo<T: HasUpdate>(xs: &mut [T]) {
    for x in xs {
        x.update();
    }
}
```

* `T: HasUpdate` means _`T` is a subset of `HasUpdate`_.
* `<T: HasUpdate>` means _given a subset `T` of `HasUpdate`_ (which will be
    chosen on the call-site), there is a function which accepts a slice whose
    items are elements of `T`.

In our model, different generic instances are associated with different maps.
