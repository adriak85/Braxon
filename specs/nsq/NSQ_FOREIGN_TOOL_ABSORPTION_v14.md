# NSQ Foreign Tool Absorption v14

Generated: 20260428_141459

This install absorbs CPAN/native/build tool source surfaces into NSQ stamp bodies.

It does not load CPAN as runtime.
It does not create a parallel runtime.
It forbids silent foreign execution.
The active authority after this pass is the NSQ substrate stamp registry.

Registry: `state/nsq/stamps/foreign_tool_absorption/20260428_141459/foreign_tool_substrate_stamp_registry.jsonl`
Capture manifest: `state/nsq/stamps/foreign_tool_absorption/20260428_141459/capture_manifest.tsv`
Stamp count: 523
Guard: `tools/nsq_no_foreign_runtime_guard_v14.sh`
