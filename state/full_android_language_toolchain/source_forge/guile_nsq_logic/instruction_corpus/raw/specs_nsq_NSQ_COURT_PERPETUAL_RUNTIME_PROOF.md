# NSQ Court Perpetual Runtime Proof

NSQ Court owns the perpetual runtime route.

Canonical identity:

- authority = NSQ_COURT
- architecture_root = true
- king = compositor
- queen = linter
- court_is_agents = false

The proof path is:

NSQ source -> NSQ Court route -> NSQASM/AArch64 -> no-libc host smoke artifact.

No C reference runner is allowed. No C reference runner is stored. No quarantine copy is kept.

The proof requires:

- NSQASM tick execution
- watchdog restart after an ASM crash artifact
- checkpoint restore
- journal replay
- resource ceiling enforcement
- manual stop behavior
