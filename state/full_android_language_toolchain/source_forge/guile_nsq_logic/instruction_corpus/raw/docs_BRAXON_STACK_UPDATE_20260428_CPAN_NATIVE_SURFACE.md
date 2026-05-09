# Braxon Stack Update — CPAN Native Surface Attachment

Date: 2026-04-28

This update records a Braxon stack breakthrough, not merely a CPAN install round.

## What changed

Braxon now has a declared CPAN/Perl/XS runtime surface that can be inventoried, selected, wrapped, stamped, classified, and routed.

This is not a second Braxon runtime.

This is not Codex.

This is not silent module mixing.

This is a native tool/source surface attached to Braxon through declared command entrances and stamp records.

## Confirmed Braxon stack effects

- CPAN metadata cache is active.
- Local Perl library tree is populated under `$HOME/perl5`.
- Native XS compilation works repeatedly on Termux aarch64.
- CPAN module inventory exists under `state/nsq/cpan`.
- CPAN command wrappers exist under `tools/nsq/cpan/bin`.
- CPAN command stamps exist under `state/nsq/stamps`.
- Failure modes are now classified instead of treated as global blockers.
- MariaDB discovery/linkage is partly resolved through `Alien::MariaDB` and the MariaDB env hook.
- Web/socket tool surfaces have begun landing successfully.

## Important stack distinction

Braxon does not load CPAN as a competing runtime.

Braxon sees CPAN as a declared external tool/source surface.

NSQ remains the bus. The CPAN surface must enter through declared Braxon/NSQ command routing, stamping, and later translation/recomposition where appropriate.

## Successful / useful passes observed

- `Sub::Metadata`
- `UNIVERSAL::Object`
- `MOP`
- `decorators`
- `XML::ED` forced install after doc-only POD failure
- `Alien::MariaDB`
- `Async::Interrupt`
- `IO::String`
- `Protocol::WebSocket`
- `Test::MonkeyMock`
- `SockJS`
- `Net::DirectConnect::TigerHash`
- `Sys::Sendfile`
- `DBD::libsql`

## Classified blockers / non-core failures

These are not proof that the Braxon stack update failed.

- `DBD::mysql`: MariaDB path barrier was broken; remaining issue is MySQL-client API mismatch against MariaDB headers.
- `DBD::mysql` old `CAPTTOFU` route: old Perl API incompatibility.
- `Proc::ProcessTable`: Android process-table / missing `OS_get_table` failure.
- `Devel::hdb`: debugger dependency chain failure.
- `EV`: event backend failure on this Android/Termux context.
- `Runops::Trace`: old Perl/runops stack behavior failure under Perl 5.42 / Android.
- `Devel::Dt`: dependent on failed `B::Utils` / `Runops::Trace`.
- `Devel::tkdb`: GUI Tk dependency failure.
- `XML::XS` / `XML::xs`: name failure, no matching namespace.
- `implied` / `impied`: name failure, no matching namespace.
- `statlib`: stopped on outdated distribution protection.

## Meaning

This update gives Braxon a reusable native command/tool surface.

The breakthrough is that Braxon can now classify, remember, and route installed native tools instead of rediscovering the same CPAN/XS/build state from zero each time.

This is why the stack begins feeling faster:

- fewer repeated dependency solves
- warmed CPAN source cache
- compiled XS modules already installed
- command wrappers already placed
- surface manifests already written
- failure categories already known
- Braxon can select known-good paths instead of testing everything blind

## Rule going forward

Do not treat every CPAN failure as a blocker.

Treat each failure by class:

- PASS
- PARTIAL_FORCED_INSTALL
- BUILD_OK_TEST_FAIL
- FAIL_DOC_ONLY
- FAIL_NAME
- FAIL_LIB_API
- FAIL_ANDROID_PROC
- FAIL_EVENT_BACKEND
- FAIL_GUI_TK
- FAIL_DEBUGGER_DEPS
- FAIL_OLD_PERL_API
- FAIL_DEP_CHAIN
- STOP_OUTDATED_DIST

The Braxon stack should prefer stable native surfaces that build, load, and can be stamped cleanly.
