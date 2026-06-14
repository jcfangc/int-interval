# int-interval

[![Crates.io](https://img.shields.io/crates/v/int-interval.svg)](https://crates.io/crates/int-interval)
[![Documentation](https://docs.rs/int-interval/badge.svg)](https://docs.rs/int-interval)
[![License](https://img.shields.io/crates/l/int-interval.svg)](https://crates.io/crates/int-interval)
[![CodSpeed](https://github.com/jcfangc/int-interval/actions/workflows/codspeed.yml/badge.svg?branch=main)](https://github.com/jcfangc/int-interval/actions/workflows/codspeed.yml)
[![Coverage](https://codecov.io/gh/jcfangc/int-interval/branch/main/graph/badge.svg)](https://codecov.io/gh/jcfangc/int-interval)

A zero-allocation, `no_std`-friendly half-open interval algebra library for primitive integers.

- Half-open intervals `[start, end)`
- Branch-light, allocation-free, and `const fn` friendly concrete APIs
- Capability traits for generic interval algorithms
- Close to `std::ops::Range` performance

Supported types:

```text
U8CO/I8CO, U16CO/I16CO, U32CO/I32CO, U64CO/I64CO, U128CO/I128CO, UsizeCO/IsizeCO
```

---

## Interval Model

Intervals are represented as `[start, end)`:

* `start < end_excl`
* `len = end_excl - start`
* Empty intervals are not representable

```rust
let x = U8CO::try_new(2, 5).unwrap(); // {2, 3, 4}
```

---

## Core API

Construction:

```rust
let x = U8CO::try_new(2, 8).unwrap();
let y = unsafe { U8CO::new_unchecked(2, 8) };
```

Accessors and predicates:

```rust
x.start();
x.end_excl();
x.end_incl();
x.len();
x.midpoint();

x.contains(4);
x.contains_interval(y);
x.intersects(y);
x.is_adjacent(y);
x.is_contiguous_with(y);

x.to_range();
x.iter();
```

---

## Interval Algebra

The core algebra returns fixed-capacity result containers rather than heap-backed collections:

| Operation              | Result          |
| ---------------------- | --------------- |
| `intersection`         | `Option<T>`     |
| `convex_hull`          | `T`             |
| `between`              | `Option<T>`     |
| `union`                | `OneTwo<T>`     |
| `difference`           | `ZeroOneTwo<T>` |
| `symmetric_difference` | `ZeroOneTwo<T>` |

```rust
pub enum OneTwo<T> {
    One(T),
    Two(T, T),
}

pub enum ZeroOneTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}
```

Both result containers implement owned iteration. Their iterators are stack-based and support:

* `Iterator`
* `DoubleEndedIterator`
* `ExactSizeIterator`
* `FusedIterator`

```rust
let lhs = U8CO::try_new(2, 10).unwrap();
let rhs = U8CO::try_new(4, 6).unwrap();

let mut residuals = lhs.difference(rhs).into_iter();

assert_eq!(residuals.len(), 2);
assert_eq!(residuals.next(), Some(U8CO::try_new(2, 4).unwrap()));
assert_eq!(residuals.next_back(), Some(U8CO::try_new(6, 10).unwrap()));
assert_eq!(residuals.next(), None);
```

This makes algebra results directly consumable without allocating an intermediate `Vec`:

```rust
for residual in lhs.difference(rhs) {
    // process each remaining interval
}
```

---

## Generic Programming with Traits

The crate exposes capability traits so downstream algorithms can operate over any supported closed-open integer interval type without selecting a concrete width or signedness up front.

### Primitive and Construction Capabilities

| Trait                 | Capability                                             |
| --------------------- | ------------------------------------------------------ |
| `IntPrimitive`        | Supported primitive integer coordinate type            |
| `UnsignedPrimitive`   | Supported unsigned exact-measure type                  |
| `COPrimitive`         | Associated coordinate and measure types                |
| `COConstruct`         | Checked and unchecked `[start, end_excl)` construction |
| `COMidpointConstruct` | Checked and saturating midpoint/length construction    |

### Observation and Algebra Capabilities

| Trait          | Capability                                                       |
| -------------- | ---------------------------------------------------------------- |
| `COBounds`     | `start`, `end_excl`, `end_incl`                                  |
| `COPredicates` | Containment, intersection, adjacency, contiguity                 |
| `CORange`      | Projection to `core::ops::Range` and iteration                   |
| `COMeasure`    | Exact interval length                                            |
| `COMidpoint`   | Midpoint coordinate                                              |
| `COAlgebra`    | Intersection, hull, gap, union, difference, symmetric difference |

### Minkowski Capabilities

| Trait                         | Capability                                        |
| ----------------------------- | ------------------------------------------------- |
| `COCheckedMinkowskiLinear`    | Exact checked add/subtract/translation operations |
| `COCheckedMinkowskiHull`      | Checked multiplication/division interval hulls    |
| `COSaturatingMinkowskiLinear` | Saturating add/subtract/translation operations    |
| `COSaturatingMinkowskiHull`   | Saturating multiplication/division interval hulls |

### `IntCO`

`IntCO` is the compact capability bound for ordinary closed-open integer interval algebra:

```rust
pub trait IntCO:
    COConstruct + COBounds + COPredicates + COAlgebra + COMeasure
{
}
```

It is suitable for downstream interval-set implementations and generic algorithms that need construction, bounds, predicates, algebra, and exact measures, while leaving midpoint, range projection, and Minkowski capabilities opt-in.

```rust
fn residual_measure<I>(lhs: I, rhs: I) -> I::MeasureType
where
    I: IntCO,
{
    lhs.difference(rhs)
        .into_iter()
        .map(|interval| interval.len())
        .sum()
}
```

When an algorithm needs additional capabilities, extend the bound explicitly:

```rust
fn midpoint_gap<I>(lhs: I, rhs: I) -> Option<I::CoordType>
where
    I: IntCO + COMidpoint,
{
    lhs.between(rhs).map(|gap| gap.midpoint())
}
```

Concrete interval methods remain the `const fn`-friendly API surface. The trait layer exists for stable generic programming; if Rust eventually supports the required const trait method model, these two surfaces may be consolidated.

---

## Minkowski Arithmetic

Minkowski APIs distinguish exact linear images from interval hulls of potentially non-contiguous images.

### Checked Linear Operations

These operations return `None` when the exact result cannot be represented:

```rust
x.checked_minkowski_add(y);
x.checked_minkowski_sub(y);

x.checked_minkowski_add_scalar(3);
x.checked_minkowski_sub_scalar(3);
```

### Checked Hull Operations

For discrete integer intervals, multiplication and division may produce non-contiguous point sets. These methods return a containing interval hull:

```rust
x.checked_minkowski_mul_hull(y);
x.checked_minkowski_div_hull(y);

x.checked_minkowski_mul_scalar_hull(3);
x.checked_minkowski_div_scalar_hull(3);
```

### Saturating Linear Operations

These operations clamp endpoint arithmetic to the representable primitive domain, then revalidate the half-open interval:

```rust
x.saturating_minkowski_add(y);
x.saturating_minkowski_sub(y);

x.saturating_minkowski_add_scalar(3);
x.saturating_minkowski_sub_scalar(3);
```

### Saturating Hull Operations

These operations return representable saturated hulls for multiplication and division images:

```rust
x.saturating_minkowski_mul_hull(y);
x.saturating_minkowski_div_hull(y);

x.saturating_minkowski_mul_scalar_hull(3);
x.saturating_minkowski_div_scalar_hull(3);
```

Example:

```rust
let a = I8CO::try_new(-2, 3).unwrap();
let b = I8CO::try_new(-1, 2).unwrap();

let sum = a.checked_minkowski_add(b);
let product_hull = a.checked_minkowski_mul_hull(b);
let saturated_hull = a.saturating_minkowski_mul_hull(b);
```

For bounded integer types, saturating results remain constrained by representability under the half-open model.

---

## Midpoint-Based Construction

`int-interval` supports constructing intervals from a midpoint and an exact unsigned length for every supported interval type.

```rust
let x = I16CO::checked_from_midpoint_len(-200, 255).unwrap();
assert_eq!(x.start(), -327);
assert_eq!(x.end_excl(), -72);

let y = I64CO::saturating_from_midpoint_len(i64::MAX - 1, 5).unwrap();
assert_eq!(y.end_excl(), i64::MAX);
```

* `checked_from_midpoint_len(mid, len)` returns `None` when `len` is zero or the resulting bounds are not representable.
* `saturating_from_midpoint_len(mid, len)` clamps endpoint arithmetic to the primitive domain.
* Both preserve the half-open `[start, end_excl)` invariant.

---

## Features

* Fast primitive interval algebra
* Zero-allocation fixed-capacity algebra results
* Direct iteration over `OneTwo<T>` and `ZeroOneTwo<T>`
* Public capability traits for generic downstream algorithms
* Explicit checked versus saturating Minkowski semantics
* Suitable for `no_std`, embedded, and constrained environments

Good fits include:

* Geometry and raster operations
* Compiler span analysis
* Scheduling ranges
* DNA and sequence slicing
* Numeric algorithms
* Interval-set building blocks

Not intended for large interval sets or tree-based interval queries.

---

## License

MIT OR Apache-2.0