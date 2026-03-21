
# int-interval

A zero-allocation, half-open interval library for primitive integers.

- Half-open intervals `[start, end)`
- Branch-light, allocation-free, `const fn` friendly
- Close to `std::ops::Range` performance

Supported types:

```text
U8CO/I8CO, U16CO/I16CO, U32CO/I32CO, U64CO/I64CO, U128CO/I128CO, UsizeCO/IsizeCO
```

---

## Interval Model

* Intervals are `[start, end)` with `start < end`
* `len = end - start`
* Empty intervals are not representable

```rust
let x = U8CO::try_new(2, 5).unwrap(); // {2,3,4}
```

---

## Core API

Construction:

```rust
let x = U8CO::try_new(2, 8).unwrap();
let y = U8CO::new_unchecked(2, 8);
```

Accessors and predicates:

```rust
x.start();
x.end_excl();
x.end_incl();
x.len();
x.contains(v);
x.iter();
x.to_range();

x.intersects(y);
x.is_adjacent(y);
x.is_contiguous_with(y);
```

---

## Interval Algebra

* `intersection` → `Option<T>`
* `convex_hull` → `T`
* `between` → `Option<T>`
* `union` → `OneTwo<T>`
* `difference` → `ZeroOneTwo<T>`
* `symmetric_difference` → `ZeroOneTwo<T>`

```rust
pub enum OneTwo<T> { One(T), Two(T, T) }
pub enum ZeroOneTwo<T> { Zero, One(T), Two(T, T) }
```

Fully stack-based, with no heap allocation.

---

## Minkowski Arithmetic

Supported operations:

* Interval-to-interval: `add`, `sub`, `mul`, `div`
* Interval-to-scalar: `add_n`, `sub_n`, `mul_n`, `div_n`

Two overflow policies are provided:

* **Checked**: returns `None` when the result cannot be represented
* **Saturating**: clamps intermediate boundary arithmetic to the primitive type range, then re-validates the resulting half-open interval

This distinction is explicit in the API:

```rust
x.checked_minkowski_add(y);
x.checked_minkowski_mul_n(3);

x.saturating_minkowski_add(y);
x.saturating_minkowski_mul_n(3);
```

All Minkowski operations preserve half-open `[start, end)` semantics and are available as `const fn`.

```rust
let a = I8CO::try_new(-2, 3).unwrap();
let b = I8CO::try_new(-1, 2).unwrap();

let c = a.checked_minkowski_mul(b);
let d = a.saturating_minkowski_mul(b);
```

For bounded integer types, saturating results are still constrained by representability under the half-open model.

---

## Features

* Fast primitive interval algebra
* Predictable, allocation-free behavior
* Explicit checked vs saturating Minkowski semantics
* Suitable for embedded or constrained environments

Good fits include:

* Geometry / raster operations
* Compiler span analysis
* Scheduling ranges
* DNA / sequence slicing
* Numeric algorithms

Not intended for large interval sets or tree-based interval queries.

---

## License

MIT
