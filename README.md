# int-interval

A zero-allocation, **half-open interval** library for primitive integers.

- **Half-open intervals** `[start, end)`
- **Branch-light**, **allocation-free**, **`const fn` friendly**
- Close to `std::ops::Range` performance

Supported types:

```
U8CO/I8CO, U16CO/I16CO, U32CO/I32CO, U64CO/I64CO, U128CO/I128CO, UsizeCO/IsizeCO
```

---

## Interval Model

- `[start, end)` with `start < end`
- `len = end - start`
- Empty intervals are **not representable**

```rust
U8CO::try_new(2,5) = {2,3,4}
```

---

## Core API

**Construction:**

```rust
let x = U8CO::try_new(2,8).unwrap();
let y = U8CO::new_unchecked(2,8); // unchecked
```

**Accessors:**

```rust
x.start()
x.end_excl()
x.len()
x.contains(v)
x.iter()
```

**Predicates:**

```rust
x.intersects(y)
x.is_adjacent(y)
x.is_contiguous_with(y)
```

---

## Interval Algebra

- **Intersection** `[A ∩ B]` → `Option<T>`
- **Convex Hull** → `T`
- **Between** → `Option<T>`
- **Union** → `OneTwo<T>`
- **Difference** → `ZeroOneTwo<T>`
- **Symmetric Difference** → `ZeroOneTwo<T>`

```rust
pub enum OneTwo<T> { One(T), Two(T,T) }
pub enum ZeroOneTwo<T> { Zero, One(T), Two(T,T) }
```

- Fully stack-based, no heap allocation

---

## Minkowski Arithmetic (New)

- **Interval-to-interval:** `add`, `sub`, `mul`, `div`
- **Interval-to-scalar:** `add_n`, `sub_n`, `mul_n`, `div_n`
- **Checked** operations, overflow-safe
- Supports negative numbers (`i8`/signed types)
- Fully `const fn` compatible, preserves `[start,end)` semantics

```rust
let a = I8CO::try_new(-2,3).unwrap();
let b = I8CO::try_new(-1,2).unwrap();
let res = a.minkowski_mul(b).unwrap(); // [-2,3)
```

---

## Features

- Fast primitive interval algebra
- Predictable, allocation-free behavior
- Suitable for embedded or constrained environments
- Ideal for:
  - Geometry / raster operations
  - Compiler span analysis
  - Scheduling ranges
  - DNA / sequence slicing
  - Numeric algorithms

Not designed for large interval sets or tree-based queries.

---

## License

MIT
