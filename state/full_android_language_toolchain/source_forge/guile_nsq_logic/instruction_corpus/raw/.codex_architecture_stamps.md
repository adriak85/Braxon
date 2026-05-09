# Stamp Wake Architecture

A stamp is an addressed wake trigger.

A stamp is not:
- a full payload
- a byte blob
- a token
- a primitive integer
- a container
- a scalar
- a fake placeholder
- a comment-only marker

When a stamp is used, something must actually happen.

Required behavior:
1. Stamp is called, thrown, placed, or resolved at an address.
2. The stored operation/framework connected to that stamp wakes.
3. Its magic-wake packet projects to the stamp address.
4. The operation/framework assembles according to its instructions.
5. The system records the wake, route, authority, and result boundary.

Tests must prefer actual behavior checks over string-existence checks.

A passing stamp test should prove that stamp use changes or emits an observable verified state, route, event, artifact, or report.
