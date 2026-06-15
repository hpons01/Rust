---
name: feedback-exercise-format
description: How to format Rustlings-style exercises for this user
metadata:
  type: feedback
---

Less scaffolding in exercise instructions — don't spell out every function signature.

**Why:** User wants to demonstrate understanding by choosing things themselves, not just fill in blanks. E.g. for method receivers, let them decide which functions take `&self`, `&mut self`, or `self` — that's part of what the exercise is testing.

**How to apply:** Give the *behaviour* expected (what the function does, what the tests verify), not the full signature. Let the user pick parameter names, receiver type, and return type. Reserve explicit signatures for concepts that haven't been introduced yet.
