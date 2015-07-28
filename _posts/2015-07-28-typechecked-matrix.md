---
layout: post
title:  "Type-checked matrix operations in Rust"
categories: arcaders
---

I recently started playing with Rust's `PhantomData`, and decided to implement
type-checked matrix operations. It turns out that, not only does it work like a
charm, the implementation is also surprisingly self-evident.

So, let's see how this works!


## Integer generics

The first problem that I encountered was Rust's lack of support for value
parameters; Although this is a hack, we can easily simulate them using traits,
macros, and associated functions:

```rust
use ::std::marker::PhantomData;
use ::std::ops::{Add, Mul};

pub trait Num {
    fn val() -> usize;
}

macro_rules! nums {
    ( $( $name:ident => $num:expr ),* ) => {
        $(
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub struct $name;
            impl Num for $name {
                fn val() -> usize { $num }
            }
        )*
    }
}

nums1 {
    N1 => 1,
    N2 => 2,
    N3 => 3,
    N4 => 4,
    N5 => 5,
    N6 => 6
}
```

As we will see soon enough, once the Rust compiler understands value parameters,
everything will be _much_ better. For the moment, however, it will do the job.


## Type-checked matrix dimensions

Now that this is out of the way, we can actually create the `Matrix` type. For
the sake of this demo, I chose to keep it simple by defining it over the real
field (`f64`) only:

```rust
#[derive(Copy, PartialEq)]
pub struct Matrix<M: Num, N: Num> {
    rows: PhantomData<M>,
    cols: PhantomData<N>,
    entries: Vec<Vec<f64>>
}

impl<M: Num, N: Num> Matrix<M, N> {
    pub fn new_map<F>(func: F) -> Matrix<M, N>
    where F: Fn(usize, usize) -> f64 {
        Matrix {
            rows: PhantomData,
            cols: PhantomData,
            entries: (0..M::val()).map(|row|
                        (0..N::val()).map(|col|
                            func(row, col)).collect()).collect()
        }
    }
}
```

Notice the calls to `Num::val()` in the construction of the matrix instance.
Through them, we ensure that the entries &mdash; if inaccessible from the
_outside world_ &mdash;, will always have the desired size. With the `new_map`
method at hand, we could define \\(\mathbf I\_3\\) as:

```rust
let i3 = Matrix::<N3, N3>::new_map(|i, j| (i == j) as isize as f64);
```

Even better, let's define it through a new associated function to `Matrix`.

```rust
impl<M: Num> Matrix<M, M> {
    pub fn identity() -> Matrix<M, M> {
        Matrix::new_map(|i, j| (i == j) as isize as f64)
    }
}

let i3 = Matrix::<N3,N3>::identity();
```

The way we defined it, the type-checker enforces that an identity matrix must
always have an equal number of rows an columns. In fact, because of this, Rust
can figure what the second type should be by itself:

```rust
// Only specify the first (or second) size type...

let i3 = Matrix::<N3,_>::identity();

// ... or, rely fully on type inference:
//
// (__*__)(N3*N2)  is how type inference starts
// (__*N3)(N3*N2)  from the definition of matrix multiplication
// (N3*N3)(N3*N2)  from the definition of `Matrix::identity`
//  ^~        ^~
// Therefore, m2: Matrix<N3, N2>.
// Therefore, equality is defined.

let m = Matrix::<N3, N2>::new_map(|i, j| (i + j) as f64);
let m2 = Matrix::identity() * m.clone();

assert!(m == m2);  // true => the test passes
```

If we were to get distracted and ask Rust for a rectangular identity matrix,
either through explicit typing or during a type-infered computation, then it
would _reject_ our code:

```rust
let i3 = Matrix::<N3,N2>::identity();

// <anon>:73:14: 73:39 error: no associated item named `identity` found for type `Matrix<N3, N2>` in the current scope
// <anon>:73     let i3 = Matrix::<N3,N2>::identity();
                       ^~~~~~~~~~~~~~~~~~~~~~~~~
```

As a side note, we can also use `new_map` to, quite elegantly, define matrix
transpose:

```rust
impl<M: Num, N: Num> Matrix<M, N> {
    pub fn transpose(&self) -> Matrix<N, M> {
        Matrix::new_map(|i, j| self.entries[j][i])
    }
}
```

You got to admit that it's quite close to the mathematical notation:
\\({\mathbf M}^T\_{ij} = {\mathbf M}\_{ji}\\).



## Matrix multiplication

Let us first define matrix multiplication by a scalar:

$$\alpha({\mathbf m}\_{ij}) = (\alpha\ {\mathbf m}\_{ij})$$

```rust
// matrix * scalar
impl<M: Num, N: Num> Mul<f64> for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn mul(self, rhs: f64) -> Matrix<M, N> {
        Matrix::new_map(|i, j| self.entries[i][j] * rhs)
    }
}

// scalar * matrix
impl<M: Num, N: Num> Mul<Matrix<M, N>> for f64 {
    type Output = Matrix<M, N>;
    fn mul(self, rhs: Matrix<M, N>) -> Matrix<M, N> {
        rhs * self
    }
}
```

Next, we shall define multiplication between matrices. Remember that, when using
the usual rule, an \\(M \times N\\) matrix applied on an \\(N \times L\\) matrix
produce an \\(M \times L\\) matrix:

```rust
impl<M: Num, N: Num, L: Num> Mul<Matrix<N, L>> for Matrix<M, N> {
    type Output = Matrix<M, L>;
    fn mul(self: Matrix<M, N>, rhs: Matrix<N, L>) -> Matrix<M, L> {
        // ...
    }
}
```

Here, I kept the type of `self` in order to clearly illustrate the role of the
matrix's size in the operation. By relying once again on the `new_map` function,
we get the following definition for matrix multiplication:

```rust
impl<M: Num, N: Num, L: Num> Mul<Matrix<N, L>> for Matrix<M, N> {
    type Output = Matrix<M, L>;
    fn mul(self: Matrix<M, N>, rhs: Matrix<N, L>) -> Matrix<M, L> {
        Matrix::new_map(|i, j|
            (0..N::val()).map(|k| self.entries[i][k] * rhs.entries[k][j])
                .fold(0.0, Add::add))
    }
}
```

Moreover, once `Iterator::sum` stabilizes, we will be able to something _much_
closer to the mathematical definition:

$$(\mathbf{AB})\_{ij} = \sum\_{k=1}\^n A\_{ik}B\_{kj}$$

```rust
impl<M: Num, N: Num, L: Num> Mul<Matrix<N, L>> for Matrix<M, N> {
    type Output = Matrix<M, L>;
    fn mul(self: Matrix<M, N>, rhs: Matrix<N, L>) -> Matrix<M, L> {
        Matrix::new_map(|i, j|
            (0..N::val()).map(|k| self.entries[i][k] * rhs.entries[k][j]).sum()
    }
}
```

One nice thing is that this definition even works if we change the type of the
entries, as long as those implement `Add`, `Mul` and, when using `sum()`, the
(still unstable) `Zero` trait.

Another interesting consequence of multiplication being type-checked in this way
is that we do not actually _need_ the bound-checking enabled in Rust by default.
As such, even when we push our program to production &mdash; with optimizations
turned on and bound-checking turned off &mdash; we can be confident that our
code will act as expected.

Also, once integer generics and associated constants make their way into the
language, we will be able to use C-like arrays instead of vectors inside of the
`Matrix` structure, and every single matrix function using `new_map` will profit
from the optimization, simply by updating a single method.


## Adding vectors

Now that we've implemented all of these matrix operations, we might want to play
with vectors. I mean, it would be nice to have `Vector<N>` and `Covector<N>`
types which interact nicely with matrices without having to rewrite too much
code.

Well, it just so happens that Rust has a feature for that: _type aliases_.

```rust
type Vector<N> = Matrix<N, N1>;
type Covector<N> = Matrix<N1, N>;
```

Or, if you're doing quantum mechanics:

```rust
type Ket<N> = Matrix<N, N1>;
type Bra<N> = Matrix<N1, N>;
```

We can now implement vector-specific methods:

```rust
impl<N: Num> Ket<N> {
    fn vec_entries(&self) -> Vec<f64> {
        self.entries.iter().map(|row| row[0]).collect()
    }
}

impl<N: Num> Bra<N> {
    fn vec_entries(&self) -> Vec<f64> {
        self.entries[0].clone()
    }
}
```

Whatever method we implement for `Ket<N>` will be available for `Matrix<N, N1>`.
However, it's more readable this way, and the intent is obvious. Notice how
these methods, too, would benefit from value parameters by returning an array of
the good length instead of a vector.


## Conclusion

So, this was how to take advantage of Rust's type-system when playing with
matrices and vectors. This is obviously a very basic toy example, however one
could easily extend these principles to make strong guarantees about abstract
systems, and it may very well be my favorite feature of the language.

I just _really hope_ that value parameters and `impl` specialization will be
added to the language; that and the ability to return unboxed closures. These
would make the type-system ridiculously powerful and expressive.

Anyway, hope you found that interesting!
