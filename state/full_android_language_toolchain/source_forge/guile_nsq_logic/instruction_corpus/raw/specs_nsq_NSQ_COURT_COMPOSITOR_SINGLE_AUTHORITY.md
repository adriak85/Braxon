# NSQ Court Compositor Single Authority

## Canonical rule

The court is internal runtime structure, not a set of agents.

The compositor is the primary court component.

Do not instantiate, route to, schedule, message, or address a separate `king` component.

Do not instantiate, route to, schedule, message, or address a separate `queen` component.

## Correct component identities

- primary_component = compositor
- lint_component = linter
- court_is_agents = false

## Deprecated language

Older language may have described the compositor as the king and the linter as the queen.

That language is deprecated for runtime, config, carrier, benchmark, and loop behavior because it can cause the system to route to both the role label and the real component in the same pass.

If a historical note must mention the old label, it must state that the old label is non-routable and deprecated.

## Guard rule

Executable/config/runtime surfaces must not contain standalone routable `king` or `queen` identities.

The compositor is the only primary component identity.
