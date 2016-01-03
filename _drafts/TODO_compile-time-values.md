Compile-time values.

<T> : a type
<T: U> : a type implementing a trait (subset)
<'a> : a lifetime
<'a: 'b> : a lifetime subset of another lifetime

<#v: T> : an instance of T (element of) known at compile-time.
<@v: T> : an instance of T known at run-time, but which can be used for testing
    some properties at compile-time.

E.g.

```rust
#[compile_time_annotation(...)]
struct Array<T, #len: usize> {
    len: #len,
    values: [T, #len]
}
```

VS

```rust
// Also a compile-time annotation, but if the compiler cannot prove that the
// provided condition is true, then it will add runtime checks. This would
// allow us to add dependent types to rust through @-directives.
@[assert(@len == values.@len)]
struct Vec<T, @len: usize> {
    len: @len,
    values: Box<[T]>,
}
```
