# Ch.4 Complete — Slices and full Bloc 2

Ch.4 finished (Brown Book). All Bloc 2 concepts validated: ownership/move/copy/drop (S5), references/borrowing/NLL (S6), and now slices as fat-pointer reference types.

User can reason about &str vs &String (deref coercion), &[T] for buffers, and knows why slices matter for no_std embedded (zero-copy, uniform API over any contiguous memory).

**Evidence**: User self-reports Ch.4 completion; prior records show 100% quiz on S5 and S6.

**Implications** — Bloc 2 is complete. Ready for Bloc 3: structs (Ch.5) and enums/pattern matching (Ch.6). First struct with String fields will immediately exercise all Bloc 2 knowledge. Zone of proximal development = struct syntax + impl blocks.
