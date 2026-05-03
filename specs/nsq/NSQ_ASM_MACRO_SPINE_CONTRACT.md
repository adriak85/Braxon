# NSQ ASM Macro Spine Contract

Authority:

- NSQ is the lowest base language.
- NSQ is the substrate.
- NSQ is the machine.
- ASM is the first specialized native macro surface.
- Other language surfaces remain translation and recode inputs.
- Other language surfaces are not separate runtime authorities.
- Local repo source is read directly.
- External source must pass through an NSQ recode carrier before stamp save.
- Useful tool functions and structures are saved as ASM macro stamps.
- ASM macro stamps are saved under directories matching their source library.
- Benchmarks run twice.
- Round 1 is warmup and discovery.
- Round 2 is the only scored round.
- The first required index set is alphabet plus Unicode range index.

Directory law:

    state/nsq/stamps/libraries/<matching_library>/asm/macros
    state/nsq/stamps/libraries/<matching_library>/metadata
    state/nsq/stamps/registry
    state/nsq/stamps/indices

No acquisition is started by this spine.
