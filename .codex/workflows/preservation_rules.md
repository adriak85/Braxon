# Preservation Rules

Do not broad-rewrite the repo.

Do not replace current architecture with standard Rust/binary assumptions.

Do not “simplify” NSQ into conventional CS language if that changes meaning.

Use the fewest effective edits.

Before changing:
1. identify exact failing behavior
2. identify exact file/function responsible
3. patch only the relevant area
4. run visible verification
5. report what changed and what remains

Never hide errors with quiet flags.
Never mark real required work as ignored just to pass tests.
Never replace behavior tests with string-only tests unless the target is explicitly a textual contract.
