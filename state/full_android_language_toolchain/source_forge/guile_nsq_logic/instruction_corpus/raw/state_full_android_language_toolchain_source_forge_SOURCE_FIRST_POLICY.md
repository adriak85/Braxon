# Braxon Source-First Forge Policy

Use package-manager tools as bootstrap only.

Preferred order:
1. Build from source into the Braxon state registry.
2. Verify with command, version, compile smoke, and hash manifest.
3. Promote into Braxon env path only after proof.
4. Keep Termux package binaries as fallback.
5. Do not replace working proven host tools blindly.
6. Use j7 as the default phone-local concurrency.
7. Build small/core dependencies first, then larger stacks.
8. Keep every source-built lane isolated, lockable, and reproducible.

Target source lanes:
- shell/help: zsh, fish, guile, mandoc/man helpers
- build tools: cmake, ninja, pkgconf where practical
- language tools: Zig, Tree-sitter, Lua, Perl modules, Ruby gems where practical
- Android lane: Gradle/AGP/cargo-apk bridge later
- graphics/text: HarfBuzz, FreeType, Fontconfig later if needed
