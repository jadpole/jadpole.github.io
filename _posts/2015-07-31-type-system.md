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
functions. While doing so, I'd like to share some methods to visualize how the
different parts of Rust's type-system fit together.

And I thought that I would tackle that last part right now.


## Type hierarchy, Venn-style

The most flagrant example of Rust's minimalism is perhaps its type-system. It
traded classical OOP for a more _functional_ approach based solely around the
concept of traits. It is through traits that we define the interface of our
data structures, overload operators, marks concurrent types, and handle
dynamic dispatch. One concept to rule them all!

I assume that you already know what traits are, and so I won't linger too much
on this topic. However, I would like to ask you: _what does the colon mean_?
What are you _really_ saying when you write `MyTrait: Add + Copy`? Well, let us
first consider the set of all possible __values__. In this context  `let x: i32`
means \\(x \in \mathbf i32\\).

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
the language; a type can contain _elements_ (values), but it cannot contain
other _subsets_ (traits or tangible types).

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

> This works because, somewhere in the standard library, there is:
>
>     impl<T: Hash> Hash for Option<T>

In this picture, `String`, `i32` and `Option<i32>` have access to `Hash::*`
while `Option<MyType>` and `Option<i32>` have access to `Option::*`. You can
notice that, because `Option<i32>` has access to both, it lives within the
intersection of `Hash` and `Option`.

From this example, you could infer that generics allow to automatically generate
types on-demand and that an `impl MyType` block is _really_ just an anonymous
trait which is automatically implemented for all instances of `MyType`.

Now, this _diagram_ thing might make the type-system look even more daunting
than it already did. However, when compared to Ruby's mental model, for example,
we can see that Rust's is _much_ simpler to think about. For instance, all types
are created equal; it makes no sense to talk about a _parent_ or a _child_ type,
and we do not have to concern ourselves with constructors, inheritance, and all
of these other concepts which we've become so accustomed to. The language just
forces you to be super-duper explicit about everything.

Before moving on, you should probably wait for everything to sink in, because
we're going to level up and make our model even more abstract as we explore
static and dynamic dispatch and conditional `impl`.


## Extending the model

What about static and dynamic dispatch? Well, let's tackle __static dispatch__
first. Say you define:

```rust
fn update_all<T: HasUpdate>(xs: &mut [T]) {
    for x in xs {
        x.update();
    }
}
```

* `T: HasUpdate` means _`T` is a subset of `HasUpdate`_.
* `<T: HasUpdate>` means _given a subset `T` of `HasUpdate`_ (which will be
    chosen on the call-site), there is a function `update_all::<T>` which
    accepts a slice whose items are elements of `T`.

In our model, different generic instances are associated with different maps.
That is, like in mathematics, `update_all::<Ork>` is different from
`update_all::<Gnome>`: they are defined over _completely distinct domains_.
Trait-wise, the first implements `Fn(&mut [Ork])` and, the second,
`Fn(&mut [Gnome])`.

![A function instance selects only one type](/images/talk-1-5.png)

Simple, isn't it? Now, handling __dynamic dispatch__ is a bit more convoluted,
but bare with me!

Remember when I told you that we omitted the `Sized` trait? Well, now, we're
going to have to take it into account. As we go along, we're going to blur more
and more the line between types and traits, data and processes.

First, the _deeper_ you go in the trait hierarchy, the more _precise_ you get,
and _types_ are the deepest that you can go (values are not considered). They
allow you to access the physical data directly. But what if you didn't actually
_need_ to be this precise; what if you wanted to talk abstractly about data,
that is only through the traits that it implements?

In this abstract model, types are just traits that provide accessor methods with
a special syntax to manipulate the attributes. As such, when you declare a
function...

```rust
fn foo(x: i32) -> i32
```

... you _really_ mean that it accepts an argument `x` which is an element of
`i32`: an element which _implements_ `i32`. Types do not exist at this level
of abstraction. Only set-like traits and element-like values exist.

![Types are now subsets, like traits are](/images/talk-1-6.png)

Types implement the `Sized` trait, a property which may not be possessed by
_normal_ traits, due to the fact that they do not carry any data. This explains
the error message shown when attempting to define a function which accepts a
trait instance as an argument:

```
<anon>:5:15: 5:17 error: the trait `core::marker::Sized` is not implemented for the type `HasUpdate` [E0277]
<anon>:5 fn update_all(xs: HasUpdate) {
                       ^~
<anon>:5:15: 5:17 note: `HasUpdate` does not have a constant size known at compile-time
<anon>:5 fn update_all(xs: HasUpdate) {
```

Notice how it complains about the _type_ `HasUpdate` not implementing `Sized`.
But... we passed in a trait, right? It turns out that, behind the scenes, Rust
actually generates _trait types_, which have the same name as their corresponding
traits (for convenience only, it's quite confusing at first), but do not
implement `Sized`, so that a collection can store diverse values.

However, as _users_ of these abstractions, we do not _need_ to worry about such
details. We gain much better insights by ditching types and thinking exclusively
in terms of traits as subsets (and not the other way around, which is the
compiler's job).

What you have to do, then, is to create a sized value which holds on to an
unsized one. Another way to put it is: we must store unsized trait objects
behind a pointer. One usually uses a `Box` or a reference, but any pointer-like
object defined by the standard library should do. For example:

```rust
fn update_all(xs: &mut [Box<HasUpdate>]) {
    for x in xs {
        x.update();
    }
}
```

With this function defined, all you have to do is store your values behind boxes
and let `rustc` figure out how to call the good `update` method for each object;
how to _dynamically dispatch_ the correct method at run-time. This is where the
name of this trick comes from.

Now that we got working _code_, how can we _visualize_ this new way to think
about data and processes? Simply in the same way that we always did:

![Blurring the line between traits and data](/images/talk-1-7.png)

Now that we blurred the line between types and traits, we do not _need_ to go
as deep to describe values: `HasUpdate` _is_ the type. In the same way that we
can say that many values inhabit a given type, many (diverse) values inhabit a
trait. All you need is a bit of indirection for each leap of abstraction.

Why doesn't Rust default to this system, then, if it is so powerful? Well, every
indirection has a cost in terms of memory and performance. Rust is also designed
for niches that require bare-metal manipulation of memory and greatly benefit
from defaulting to the stack. As with everything which has a cost-penalty, Rust
gives you the option to explicitly opt-into the feature, but will strive for the
better performance (and safety) by default.


## Conditional implementation

Newcomers to Rust often complain about the verbosity of Rust's `impl` statements
and, if you only use them to implement traits for types, they're quite boring. I
mean, their sole purpose in this context is to say that a type set is included
inside of a trait set. That's it!

However, the place where the syntax really shines is when you start to talk
about the ways in which _traits_ are related to each other. You do that through
_conditional implementation_, which allow you to implement a trait for all types
that respect a specific condition. For example:

```rust
impl<T: Mob> Entity for T {
    // ...
}
```

This says: every `T` which is a subset of `Mob` is also a subset of `Entity`,
and every `Mob` shares the same implementation for the `Entity` trait.
Graphically, `Mob` is contained by `Entity`, because while every item
in `Mob` implements `Entity`, the inverse isn't always true. This pattern is
easily extended to intersections. For instance:

```rust
impl<T: A + B> C for T
```

... means that for every \\(T \in A \cap B,\ T \in C\\). In other words, the
intersection of `A` and `B` is a subset of `C`. In general, it can be easily
read out loud:

* `impl`: let me implement
* `<T: A + B>`: where \\(T \subset A \cap B\\)
* `C`: the trait \\(C\\)
* `for T` (a subset of \\(A \cap B\\))

Or, all in one sentence:

> Let me implement the trait \\(C\\) for all \\(T \subset A \cap B\\).

When you get something more complicated, it becomes harder to say out loud, but
the principle stays the same:

```rust
impl<R: Num, C: Num> Add<Matrix<R, C>> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn add(self, rhs: Matrix<R, C>) -> Matrix<R, C> { ... }
}
```

> For all \\(R, C \subset Num\\), let `Matrix<R, C>` be a subset of
> `Add<Matrix<R, C>, Output=Matrix<R, C>>` with the following implementation.

You could say that conditional implementations enforce high-level behavior
(`Entity`) shared by many types, provided that they implement the required
lower-level subtleties (`Mob`).


## Conclusion

So, what is the _big picture_ here? Well, traits and values are _really_ all
there is; the deeper you go, the more precise you get, until you reach _types_
which are nothing more than subsets of `Sized` with a special syntax to
manipulate the underlying memory and from where you cannot go deeper.

Because of these properties, it is possible to manipulate values as though they
were traits: greater abstraction at the price of control and indirection.

Trait dependencies specify the subset in which a trait must exist, without
specifying how to implement it, while conditional implementations (which are
_really_ just the trait version of generic functions) specify the subset in
which a trait exists, forcing similar types to share a common behavior.

I'm still developing these ideas, so any insight that you might have is more
than welcome. I believe that it is important to state the philosophy and the
mathematical foundations of a language when we want it to evolve without it
becoming the next bloated mess. It also helps to _reason_ in a given language
and write _idiomatic code_, whatever this means.

Plus I'm a big mathematics lover, and it makes me happy to connect
([join](https://github.com/rust-lang/rfcs/blob/master/text/1102-rename-connect-to-join.md)? :P)
the semantics of software development and set theory. Even if it probably
doesn't change much in everyday programming.

Anyway! Hope you enjoyed.
