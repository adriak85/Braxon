# No-Hidden-Files and Rebuild Audit

## Verified traversal contract

The executable `nsq-system::SourceTree::scan` walks the complete repository tree and excludes only directories named `.git`. It does not exclude dotfiles, `target` directories, generated outputs, backups, uncommon extensions, or intermediate build products. The scanner classifies those artifacts so they remain visible to audit and intent extraction.

The scanner previously represented an unreadable file as the string `unreadable`, which could conceal a real traversal failure inside an otherwise successful report. That behavior is corrected. File-open and read errors now propagate as `io::Error`, causing the scan to fail rather than downgrade the condition to a warning or placeholder. Digest hints are computed by streaming file contents in bounded chunks, so the fail-visible behavior does not require loading an entire large artifact into memory.

The regression test creates hidden, generated, backup, binary, and `.git`-internal files. It confirms that every non-`.git` artifact is returned and that `.git` contents alone are excluded.

## Peripheral repository disposition

The previous all-branch audit covered `0`, `DAX-FULL`, `Dax`, `Dax-Autonomous-System`, `PAPI`, `f1ux-service`, `fastapi-llm-bot`, and `termux-packages`. Candidate branches were not promoted by label or wholesale tree replacement. Their implementation-bearing intent was transferred only where it could be reconciled with the current NSQ API and validated.

| Source family | Disposition | Rebuild status |
|---|---|---|
| Dax and DAX-FULL semantic/addressing surfaces | Extracted and represented through NSQ intent, Target Field, Ghost Memory, and kinetic reflexor contracts | Rebuilt and tested in Braxon-core |
| Dax-Autonomous-System command and loopback intent | Reviewed as donor material; invalid or multi-root build surfaces were not copied as runtime authority | Validated intent retained only where it mapped to the offline ingress and NSQ contracts |
| Titan/Vulkan, Android, and device-specific lanes | Platform-dependent and not safely runnable under the host no-Android-build policy | Explicitly platform-gated; not falsely claimed complete |
| JULES/deployment synchronization branches | Operationally coupled, branch-specific, or incomplete | Rejected as runtime authority; disposition remains documented |
| Python/Perl/Go helper and launcher surfaces | Kept as source evidence where relevant; runtime authority remains Rust/NSQ-native | Rebuilt only as typed NSQ contracts when independently validated |
| Historical backups and generated artifacts | Kept visible to the absolute-tree audit; duplicate recovery copies are not silently promoted | Explicitly classified and dispositioned rather than treated as current implementation |

No branch-provided script was executed merely because it existed in a candidate branch. This prevents an unvalidated repair script, build product, or generated artifact from becoming an accidental runtime authority. Rejected material is documented; validated intent is rebuilt into the final target rather than silently discarded.

## Failure visibility

A passing scan means that every traversed file was readable and classified; it does not mean every historical artifact is valid runtime code. Compilation, tests, scanner errors, stale watermarks, unsupported platform gates, and rejected peripheral candidates remain distinct evidence states. The final branch reports those states explicitly and does not turn warnings into success claims.
