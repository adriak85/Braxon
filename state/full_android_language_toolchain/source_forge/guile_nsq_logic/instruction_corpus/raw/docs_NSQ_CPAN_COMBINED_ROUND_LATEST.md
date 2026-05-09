# NSQ CPAN Combined Round

Pinned at: 20260428_182011  
Root: /data/data/com.termux/files/home/Braxon

This ledger combines the previous CPAN pass and the current CPAN pass.

## Strong pins

The following are usable surfaces from this combined round:

- Sub::Metadata
- UNIVERSAL::Object
- MOP
- decorators
- Alien::MariaDB
- Async::Interrupt
- IO::String
- Protocol::WebSocket
- Test::MonkeyMock
- SockJS
- Net::DirectConnect::TigerHash
- Sys::Sendfile
- DBD::libsql

## Partial pin

XML::ED built and force-installed. Its failure was a POD syntax test, not a compile/load proof failure from the shown output. Keep it marked as usable-but-doc-test-dirty.

## Do not force into hot path yet

- EV: event backend failure on Termux/Android.
- Proc::ProcessTable: Android symbol/platform failure.
- Runops::Trace: compiles, but runtime tests show stack_grow negative count.
- Devel::Dt: blocked by B::Utils and Runops::Trace.
- Devel::hdb: blocked by Devel::Callsite and Devel::Chitin.
- Devel::tkdb / Tcl::Tk: GUI/Tk stack not ready.
- DBD::mysql: MariaDB is found now, but DBD::mysql 5.013 wants MySQL client API symbols not present in this MariaDB header/library set.

## MariaDB note

MariaDB discovery is now good. Alien::MariaDB passed. The right DBI route for this device is likely DBD::MariaDB, not forcing DBD::mysql against MariaDB client headers.

## Speed note

The install loop got faster because the dependency base is now warmer: CPAN cache is populated, many XS prerequisites are already compiled, PATH has the selected CPAN command wrappers active, and repeated builds are reusing prepared/unwrapped CPAN build directories.
