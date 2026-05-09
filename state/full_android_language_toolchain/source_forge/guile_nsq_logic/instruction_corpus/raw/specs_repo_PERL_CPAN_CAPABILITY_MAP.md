# Perl CPAN Capability Map

## Role

Perl and CPAN modules expose useful structure that Braxon can build into the native NSQ substrate spine.
They are not called down as a parallel runtime.
They are not a plugin layer.
They are not a separate authority.

NSQ is the only runtime.

## Confirmed Live Modules

- `SQ` 0.0.6: compact query/selection structure for audit tables, capability maps, and manifest slices.
- `Alien::Build` 2.84 / `Alien::Base` 2.84: native dependency discovery/build structure for Termux and Android-facing toolchains.
- `PPI` and `Perl::Tidy`: Perl source structure and formatting shape for native NSQ incorporation.
- `Parse::RecDescent`, `Parser::MGC`, `Regexp::Grammars`: grammar-shape discovery for NSQ-native parser/court construction.
- `Data::MessagePack`, `Cpanel::JSON::XS`, `JSON::XS`, `YAML::LibYAML`: compact state-shape discovery and verification.
- `DBI`, `DBD::SQLite`, `SQL::Statement`, `SQL::Maker`, `SQL::QueryMaker`, `SQLib`: audit/query structure for repo and model status.
- `Text::Xslate`, `Text::CSV_XS`, `Text::Table::*`: report and operator-surface rendering structure.
- `FFI::Platypus`, `ExtUtils::*`, `Module::Build::*`: native dependency and build-compatibility structure.
- `Plack`, `Path::AttrRouter`, `Commandable`, `Future`, `Tickit`: local operator routes, async task shape, and terminal UI structure.

## Best Fit In Braxon

- CPAN audit lane: inventory installed module structure, classify capability, and build useful structure into NSQ manifests.
- Grammar lane: use parser modules to discover grammar shape, then implement the court-readable shape natively in NSQ.
- Native dependency lane: use Alien/FFI module knowledge to expose library/build assumptions to the NSQ substrate spine.
- State lane: use SQLite/MessagePack/JSON as structure witnesses; canonical runtime meaning remains NSQ.
- Operator lane: use Plack/Tickit/Text module lessons for local dashboards if needed, with NSQ court route as authority.

## Guardrails

- CPAN metadata is not runtime incorporation.
- Perl modules do not replace NSQ court surfaces.
- Generated reports must not be mistaken for completed runtime lanes.
- Useful language/runtime structure must be built into the native NSQ substrate spine before it counts.
