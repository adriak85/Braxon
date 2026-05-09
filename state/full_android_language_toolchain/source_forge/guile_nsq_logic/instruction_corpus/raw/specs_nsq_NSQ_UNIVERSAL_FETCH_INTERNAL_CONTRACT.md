# NSQ Universal Fetch Internal Contract

This app is an NSQ app carrier first.

Rules:

- NSQ is the lowest base language.
- NSQ is the substrate.
- NSQ is the machine.
- A lever is one switch.
- A lever is one eighth of an NSQ bit.
- Hertz frequency positions the lever.
- Target selection is runtime-injected.
- No target is compiled into this app.
- No source is compiled into this app.
- Raw body transfer is forbidden.
- Only NSQ wire units are valid input.
- Framed TOML/XML/RS/C/PY/Lua/Lisp/Java/Kotlin/Shell/YAML/Markdown/Text/SQLite/Cargo/Gradle/CMake/Make surfaces are translated internal NSQ surfaces.
- Startup flags decide which internal frames activate.
- Inactive frames do not execute.
- Source repositories are local before recode.
- If a source repository is already inside Braxon, it is used in place and no localization shim is used.
- If a source repository is outside Braxon, the shim first localizes it under state/nsq/source_repositories/local, then recodes from that local copy.
- Non-NSQ wide-band drift terms are forbidden in this app surface.

Compile proof must show these exact truth fields:

    compiled_from_nsq=true
    target_compiled_in=false
    source_compiled_in=false
    raw_body_transfer_forbidden=true
    nsq_wire_units_required=true
    internal_only=true
    local_repo_first=true
    repo_in_root_uses_shim=false
    dialects_can_interweave=true
