---
layout: post
title:  "No, really, Rust ain't that complicated"
categories: rust
---

In a week, I'm giving a talk at the
[Montréal Rust Language Meetup](http://www.meetup.com/fr/Montreal-Rust-Language-Meetup/events/224148410/),
called "Rust after borrowing" and, as I'm searching for my words, I thought that
putting them down on a screen could ease the process.

The talk is going to be about Rust; more specifically, about medium-to-advanced
type-system business, and insights on some ways to _visualize_ how everything
fits together. I want to talk about static VS dynamic dispatch, phantom types
and higher-order functions, and explore the future of the language, all while
stressing that Rust is, in essence, pretty simple.

And I thought that I would tackle that last part right now.


## Type hierarchy, Venn-style

The most flagrant example of Rust's minimalism is perhaps its type-system. It
traded classical OOP for a more _functional_ approach based solely around the
concept of traits. It is through traits that one defines the interface of his
data structures, overloads operators, marks concurrent types, and handles
dynamic dispatch. This allows Rust to throws away many complicated concepts such
as __TODO__ and inheritance, replacing the latter by _composition_.

With most of OOP's complexity out of the way, Rust compensates by providing
expressive constructs, such as conditional implementation

You can even declare dependencies between traits, for example by requiring from
values evolving according to an AI to be

Because Rust bet on a single, unifying concept, it can propose

Yet, it compensates this simplicity in design by proposing expressive constructs
such as

Part of Rust's simplicity comes from the fact that its type-system only provides
two ways to organize values: _types_, which specify the manipulated data, and
_traits_, which abstractly describe properties of the values without concerning
themselves with the underlying data. While types are independent objects,

You (hopefully) already know what type and traits are.

These two constructs can be combined through the principle of _composition over
inheritance_, which states that types should be distinct entities that do not
depend on one-another. One would then express
 common processes and properties of
the underlying data by implementing traits. Unlike types, traits can be combined
in many ways

require for on one-another. For example, one could require of every NPC that it
implements `Entity`.



Then, one could consider traits and types as being sets of possible values. Let
us first consider how __traits__ behave in this representation:

![Trait _addition_ using Venn diagram](/images/talk-1-1.png)

In this context, we have two traits, `A` and `B`, where each defines a certain
specification. When one is asking for a type to implement two traits, what we
_really_ mean is that said type exists in the intersection between both traits.
In other words, `T: A + B` _really_ means \\(T \subset A \cap B\\).

What about traits that depend on other traits? Same thing! `C: A + B` _really_
means that every type implementing \\(C\\) must live within the intersection
\\(A \cap B\\); that is, \\(C \subset A \cap B\\). The two statements are
equivalent. Graphically, it is expressed by \\(C\\) being _contained_ in the
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
that, when expecting a `Vec<i32>`, you cannot receive a `Vec<String>`: the two
sets are necessarily disjoint.

We also consider that whatever methods are associated to a specific type make up
their own _implicit_ trait associated with the type. For example (where `MyType`
does not implement `Hash`):

![Option defaults + Hash](/images/talk-1-4.png)

In this picture, `String`, `i32` and `Option<i32>` have access to `Hash::*`
while `Option<MyType>` and `Option<i32>` have access to `Option::*`. You can
notice that, because `Option<i32>` has access to both, it lives in the
intersection between `Hash` and `Option`. If it were to implement yet another
trait which depends on both (in this case, through conditional implementation),
then it would lay inside a subset of \\(A \cap B\\).

You could say that generics allow to statically generate types and that the
content of the `impl` block is _really_ just an anonymous trait which is
automatically implemented for all such types.

Due to all of these properties, one could say that all types are created equal:
there is no such thing as a _parent_ or a _child_ record &mdash; just subsets in
a pool of possible value. Although it takes time to get used to this, it is much
simpler than traditional OOP. You do not have to worry about constructors,
inheritance, __TODO__

Now, this _diagram_ thing might make the type-system look even more daunting
then it already did. However, when compared to Ruby's mental model, for example,
we can see that Rust's is _much_ simpler to think about, yet much more powerful.
It just asks you to be super-duper explicit about everything &mdash; which is
necessary when you want either safety or performance, and even more so when you
want both.

Let us now consider __values__. For example, `3` or `Some("Hello")`. Both of
these are simply _elements_ inside of their respective types. That is, when you
write `let x: i32 = 3`, you _really_ mean \\(x \in \mathbb {i32} \land x = 3\\).
Yet, Rust is able to infer this by itself because <strike>Rust contributors are
awesome</strike> these two values exist in disjoint sets (which, after all, is
what we defined types to be), so there is no confusion.


## Immutability and greppability

In Rust, one cannot modify the value of a variable unless it has specifically
been marked as `mut`. One would immediately think about `let mut` and mutable
references; however, this principle extends much further. For example, what is
wrong with the following snippet?

```rust
struct Point(f64, f64);

fn foo(z: Point) {
	z.0 += 10.0;
}
```

Well, `rustc` is quite clear about the issue:

```
<anon>:4:5: 4:16 error: cannot assign to immutable anonymous field `z.0`
<anon>:4     z.0 += 10.0;
             ^~~~~~~~~~~
```

The problem is that we are trying to modify an attribute of `z`, which is an
immutable value. The fix, as always, is to add a `mut` somewhere, namely before
the binding's name:

```rust
fn foo(mut z: Point) {
	z.0 += 10.0;
}
```

Simple, efficient, and if we ever were to declare a binding as mutable without
actually needing it, then Rust would ask us to change it:

```rust
fn foo(mut z: Point) {
	println!(“({}, {})”, z.0, z.1)
}
```

```
<anon>:3:8: 3:13 warning: variable does not need to be mutable, #[warn(unused_mut)] on by default
<anon>:3 fn foo(mut z: Point) {
                ^~~~~
```

The basic idea is that, if you intended to mutate the value, you can simply add
the `mut` keyword. However, if you did not and the value is mutable by default,
then you're gonna have a bad time! Rust assumes fallibility of the developer.
As such, it makes it really hard to shoot yourself to the foot.

Compare again with Ruby, where everything is mutable and such a bug might be a
pain to chase down; a bug which, by the way, almost doesn't exist in Rust. Yet,
Rust doesn't go as far as making mutation impossible Haskell-style, because it
acknowledges that, not only is it necessary in the niches that Rust targets,
some processes are clearly expressed better through mutation.

Another effect of this convention is that it makes mutation _greppable_. That
is, by executing:

```
$ cat *.rs | egrep “&?mut |$”
```

Tou can highlight every place where mutation occurs in your code. That is, if
your code is `mut`-free, you get most of the advantages of functional languages.
And this is very powerful, not only for people who use `grep` (which I almost
never do), but more generally for searching through code or writing tools that
manipulate it.

Want another example? Here's how to show the public API of a module:

```
$ cat wutm8.rs | egrep "^\s*pub "

pub struct Font {
	pub fn new(sdl_font: Rc<RefCell<SdlFont>>, color: Color) -> Font {
	pub fn get_sdl_font(&self) -> RefMut<SdlFont> {
	pub fn get_color(&self) -> Color {
pub struct Sprite {
	pub fn from_texture(texture: Texture) -> Sprite {
	pub fn load(renderer: &Renderer, path: &str) -> Result<Sprite, String> {
	pub fn size(&self) -> (u32, u32) {
	pub fn sub_sprite(&self, rect: Rect) -> Result<Sprite, String> {
	pub fn render(&self, renderer: &mut Renderer, dest: Rect) {
...
```

What, you also want the documentation? Here you go:

```
$ cat phi/bonzaïon.rs | egrep "^\s*(pub |///)"

/// Immutable region of a Texture used as the general types for
/// images. In general, the child of a sprite should not outlive
/// the sprite, as this may cause the texture's raw reference to
/// be freed before it is used.
pub struct Sprite {
	/// Creates a new sprite by wrapping a `Texture`.
    pub fn from_texture(texture: Texture) -> Sprite {
    /// Creates a new sprite from an image file located at the given path.
    /// Returns `Ok` if the file could be read, and `Err` otherwise.
    pub fn load(renderer: &Renderer, path: &str) -> Result<Sprite, String> {
```

Rust's syntax might be ugly and it may be more verbose than ML-like languages,
but it's also _much_ simpler to orient yourself after you've written it. It is
also much simpler to _think_ about Rust code; between other things, you can
easily follow and isolate mutation, which makes it easier to design clear,
independent interfaces.


## Lifetimes as a language

As I was writing [ArcadeRS](/arcaders/arcaders-1-4), I defined the `Phi` type
(responsible for caching, and simplifying common operations), as such:

```rust
pub struct Phi<'p, 'r> {
    pub events: Events<'p>,
    pub renderer: Renderer<'r>,
}
```

In this case, `Phi` is restrained by both of its attributes: `events` depends
on the SDL library being initialized, while `renderer` stores the data for the
window which, in turn, requires for the SDL library to be initialized. That is,
both of those, for different reasons, must have a shorter lifespan than the SDL
context.

It is upon realizing this that lifetimes _clicked_. They are often considered to
be uber-complicated by newcomers and, in some cases, they are; however, the idea
is incredibly simple. It's both a way to ask the compiler to watch our back, and
a way to express invariants to other programmers. You just say: it does not make
sense for a renderer to exist without a window to render to. In code. It's like
documentation, except that it never gets outdated. Moreover, naming a lifetime
is like naming a variable. Sometimes, the intent is obvious (e.g. `i` in a for
loop); other times, a longer, clearer name should be preferred.

It's upon this realization that I stopped viewing explicit lifetimes as a pain
in the ass, and started to consider them as teachers. They aren't always clear
(I mean, c'mon: "mdsi_cannot infer an appropriate lifetime for lifetime
parameter `'a` due to conflicting requirements_"... really?), but there is, more
often than not, a valuable lesson at the end of the road.

In this instance, though without the help of the compiler, I learned that I
could express `Phi` as:

```rust
pub struct Phi<'a> {
    pub events: Events<'a>,
    pub renderer: Renderer<'a>,
}
```

Why I am saying here is that `events` and `renderer` both depend on the same
value, namely `sdl_context`, living longer than they do. In just 12 characters!
Similarly:

```rust
fn get(&'a self, id: usize) -> &'a T
```

Not only means _the returned value shouldn't outlive `self`_, it also specifies
that, as long as it lives, `self` may not be mutated. This is because Rust
ensures that we cannot change a value while it is being immutably borrowed.

Although lifetimes are powerful, they _are_ complex concepts that need to be
learned, resulting from Rust's goal of getting zero-cost, safe references.
Yet, I would argue that dealing with lifetimes is often simpler than dealing
with raw references as you would in C++.
