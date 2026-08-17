# NSQ Ghost Memory and Piston Window

## Contract

Ghost Memory is a **logical NSQ wire-address space**, not a claim of new physical RAM, CPU cache, CPU registers, or undocumented CPS resources. The full parameter, weight, tokenizer, launcher, and fact surfaces remain represented as wire pages in the virtual extension. Only the page currently fired through the Piston is exposed through the software-controlled CPU aperture.

The current implementation uses a **15 MiB firing window**. A larger wire-resident region is split into 15 MiB pages, and the Piston rotates one page at a time. The same aperture is reused only after the prior lease reaches `Release`; a second intent cannot overwrite the active space. Every firing has an intent owner, page identity, aperture address, phase, and monotonic generation.

> The wire address is virtual protocol state. The CPU aperture is a software mapping contract. Neither field is a physical CPU address or a promise that the runtime controls the phone’s cores, caches, MMU, scheduler, registers, or physical memory controller.

## State transitions

| State | Meaning | CPU execution permitted |
|---|---|---|
| `OnWire` | Page exists in the NSQ wire-address namespace and is not active in the aperture. | No |
| `Firing` | Piston acquired the page and reserved the aperture; commit has not completed. | No |
| `Mapped` | Piston commit completed and the page is the active CPU-visible window. | Yes, subject to the host adapter |

A `Release` transition returns the page to `OnWire`, clears its aperture mapping, and makes the single software aperture available for the next page. Pressure, unknown pages, overlap, stale leases, and same-space contention fail closed.

## Physical boundary

The implementation reports `physical_cpu_resources_touched: false` for every firing. It does not use privileged instructions, device mappings, physical addresses, CPU affinity, cache controls, or Android-target builds. A real device adapter may later connect the software aperture to an approved OS virtual-memory mechanism, but that adapter must preserve the same contract and provide platform-specific acceptance evidence.

## Source

The executable contract is in `crates/braxon-core/src/ghost_memory.rs`, exported by `braxon-core`. The fixed-window constant is `FIRING_WINDOW_BYTES = 15 * 1024 * 1024`; the virtual extension begins at `2^48`, outside the ordinary 48-bit CPU virtual-address range used by the model contract. These are protocol coordinates, not physical addresses.
