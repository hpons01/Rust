# Ownership: move semantics, Copy trait, and drop

S5 completed with 100% quiz and Rustlings. User can explain why `let s2 = s1` invalidates `s1` for `String` but not for `i32` — the distinction between heap types (move) and Copy-implementing stack types (implicit copy). Understanding of drop is solid: values are freed when the owner goes out of scope, no manual free needed. Coming from C, the "one owner → one free" invariant clicked naturally against the manual free/double-free problem.

**Implications** — ready to learn references as the mechanism to access data without transferring ownership. The C analogy (pointer vs value) will be useful but must be handled carefully: Rust references carry compile-time validity guarantees that raw C pointers do not.
