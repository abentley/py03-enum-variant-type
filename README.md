# Return struct-like enum variant values to Python.

This crate provides the `EvtPyclass` derive macro, enabling enums with
struct-like variants to be returned to Python.

For example:
```rust

#[derive(Debug, EvtPyclass, EnumVariantType)]
enum Foo {
    #[evt(pyclass(set_all), derive(FromPyObject))]
    Blue { num: i32 },
    #[evt(pyclass, derive(FromPyObject))]
    Green { num: u32 },
}

#[pymodule]
fn enum_to_py(_py: Python, m: &PyModule) -> PyResult<()> {
    Foo::add_variant_structs(m)?;
    Ok(())
}

```

## What this actually does
This produces an enum `Foo` with variants `Foo::Blue` and `Foo::Green`, but
also, additional `Blue` and `Green` structs, with `#[pyclass]` annotations, via
`EnumVariantType`.  Finally, `EvtPyclass` provides an implementation of
`IntoPy<PyObject>` for Foo, leveraging the implementations in `Blue` and
`Green`.

## Relationship to Python
Through this approach, the enum is converted to a struct and that struct is
returned to the Python calling code.  This fits with the Python concept of
"duck typing", where values of arbitrary types may be supplied to or returned
from a function or method.  From this perspective, Rust's enums are like a
compiler-enforced `typing.Union`.  (Indeed, in other contexts, Rust's enums are
called "tagged unions")

In addition to the `typing.Union` idea, you may find it fruitful to express their
relationship in terms of a common `typing.Protocol` that they implement.  This
could be used as the return type of a function, for example.

You may also find it useful to use Python's `match` statement (as of 3.10), which
could be used similarly to how a Rust `match` statement is used with an enum:

```python
match return_foo():
    case Blue():
        print("Got Blue")
    case Green:
        print("Got Green")

```
