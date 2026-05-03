# NSQ CPAN Surface Attach

CPAN tools are treated as declared runtime surfaces.

Real commands get wrappers under:

tools/nsq/cpan/bin

Module libraries are inventoried under:

state/nsq/cpan/cpan_module_manifest.tsv

Command surfaces are inventoried under:

state/nsq/cpan/cpan_command_manifest.tsv

Stamps are written under:

state/nsq/stamps/cpan_command_surface_stamps.jsonl

Rule:

Do not create fake commands for every Perl module. Attach real commands. Record libraries as module surfaces.
