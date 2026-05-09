# Patch Discipline

Use targeted patches.

Avoid global search/replace unless the target is exact, bounded, and safe.

After automated patching:
- grep the changed keys
- inspect the affected functions
- run formatting
- run the smallest relevant test
- then run broader tests

Do not patch by guessing old line numbers after cargo fmt changes structure.

Prefer function-bounded edits over whole-file substitutions.

If a generated patch creates duplicate JSON keys, undefined variables, or dead fake fields, fix that before claiming progress.
