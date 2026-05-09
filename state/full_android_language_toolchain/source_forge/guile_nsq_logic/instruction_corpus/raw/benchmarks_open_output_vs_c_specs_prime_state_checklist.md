# NSQ Prime State Checklist

Before timing a run, confirm:

- no forced live decode in the timed phase
- no wrapper path that is not part of NSQ's real native route
- no duplicate doctor/probe loops in the hot path
- no directory-as-file mistakes
- no artificial output caps
- no parity flattening against C
- time budget starts immediately before native production
- time budget stops immediately after native production
- normalization happens only after stop
- artifact size is measured
- human-readable projection is measured
- replay stability is measured
- corruption survival is measured

If any of the above is false, NSQ is not in prime benchmark state.
