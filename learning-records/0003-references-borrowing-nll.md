# References, borrowing rules, and NLL

S6 completed. User understands the core borrowing constraint: either N immutable references OR one mutable reference, never both simultaneously. The hardware register analogy (multiple readers OK, one writer with exclusive access) was the session's framing and maps well to the user's embedded background. Dangling references (returning &local) are detected at compile time — contrast with C where this is silent UB. NLL (Non-Lexical Lifetimes) is understood: borrow lifetimes end at last use, not at closing brace, which makes the borrow checker more permissive in practice.

**Implications** — foundation for Bloc 2 is complete. Next logical step: slices (&[T], &str) as reference types that carry length, then structs with lifetime annotations when references appear in struct fields. The permission model (R/W/O) introduced in the Brown Book is a strong mental model to reinforce.
