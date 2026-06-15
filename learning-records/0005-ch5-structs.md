---
name: ch5-structs
description: Ch.5 structs, impl blocks, methods, associated functions — Bloc 3 entry
metadata:
  type: project
---

Ch.5 finished (Brown Book). First Bloc 3 chapter. Covers struct syntax, impl blocks,
method receivers (&self / &mut self / self), associated functions, tuple structs,
unit-like structs, and #[derive(Debug)].

Lesson + quiz delivered as ch5-structs.html.
Mini-project exercise created at exercises/ch5_structs/ (SensorLog, BME280-style).
9/9 tests passing. Implementation correct and idiomatic.

**Evidence of understanding:**
- Field init shorthand used correctly in `new()`
- `as f32` cast placed on the operand before division (correct)
- Struct update syntax `..base` used without `clone()` — user understands all fields are Copy
- `&self` / `&mut self` distinction applied correctly

**One C-ism to watch:** used a block `{ expr && expr }` for the validity boolean inside `new()`. Idiomatic Rust writes it as a plain boolean expression without the wrapping block.

**Why:** First chapter where ownership of struct fields becomes concrete (String vs &str → lifetimes preview).
Struct update syntax directly exercises Ch.4 move/copy rules.

**Implications:** Ready for Ch.6 (enums + pattern matching). Zone of proximal development = Option<T>, Result<T,E>, match arms, if let.
