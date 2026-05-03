# NSQ Global Metadata Law

Purpose: when one repo object changes, the repo must know what else is affected.

Core law:

- Every tracked source object has identity metadata.
- Every generated object has source lineage.
- Every dependency has a reverse dependency.
- Every alteration creates an impact event.
- Every impact event marks affected proofs, stamps, routes, indexes, and reports stale until refreshed.
- Generated reports are not source authority by default.
- Generated harm reports and generated stamp registries must not self-poison scored scans.
- Metadata is tracking and identification, not payload replacement.
- Metadata may route, verify, compare, and invalidate.
- Metadata may not pretend an incomplete system is live.
- Proofs remain green only while their source hashes and dependency hashes match.
- Local repo source can be read directly.
- External source must pass through an NSQ recode carrier before stamp save.
- Other language surfaces are translation and recode inputs under NSQ authority.

Object classes:

    source_authority
    nsq_carrier
    law_spec
    support_tool
    generated_index
    proof_report
    quarantine

Impact statuses:

    current
    changed
    added
    removed
    stale_due_to_dependency
    needs_metadata_refresh
    needs_stamp_refresh
    needs_proof_rerun

Clean metadata is positive tracking: it lets NSQ find, verify, and update the right thing without flattening NSQ into another language model.

## ASM Operating Law Metadata Update

ASM is now tracked as the operating, recode, and optimization form for NSQ.

Metadata must preserve the distinction between:

- NSQ source truth
- ASM operating form
- Braxon binary translation output
- protected personal moral invariant

Generated binary translations are downstream artifacts and must not be promoted into NSQ source authority without explicit lineage and authority metadata.

No metadata file may override, morph, invert, or translate against the protected personal moral invariant.
