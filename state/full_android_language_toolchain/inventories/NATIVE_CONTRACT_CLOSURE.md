# Braxon Native Contract Closure Inventory

Generated: 2026-08-21T11:25:46.668Z

> This report is derived from retained source/build surfaces and canonical contracts. It does not claim a native Android build, ABI validation, consumer execution, or Rust nightly promotion unless the corresponding target receipt exists.

## Target boundary

The declared target is **aarch64-linux-android** at Android API 24. This generator executed on **x64/linux**; therefore every native interface remains **BLOCKED_TARGET** until an actual target compile/link/run and downstream-consumer proof is retained.

## Declared native interfaces and observed source/build references

| Interface | Class | Header | Source/build references | Current status |
|---|---|---|---:|---|
| sem_clockwait | G_ABI_LINKAGE_ADAPTATION | semaphore.h | 8 | BLOCKED_TARGET |
| pthread_getname_np | G_ABI_LINKAGE_ADAPTATION | pthread.h | 72 | BLOCKED_TARGET |
| close_range | E_SYSCALL_BACKED_IMPLEMENTATION | unistd.h | 98 | BLOCKED_TARGET |
| statx | E_SYSCALL_BACKED_IMPLEMENTATION | sys/stat.h | 480 | BLOCKED_TARGET |
| copy_file_range | E_SYSCALL_BACKED_IMPLEMENTATION | unistd.h | 192 | BLOCKED_TARGET |
| getrandom | E_SYSCALL_BACKED_IMPLEMENTATION | sys/random.h | 454 | BLOCKED_TARGET |
| memfd_create | E_SYSCALL_BACKED_IMPLEMENTATION | sys/mman.h | 152 | BLOCKED_TARGET |
| eventfd | E_SYSCALL_BACKED_IMPLEMENTATION | sys/eventfd.h | 474 | BLOCKED_TARGET |
| eventfd_read | G_ABI_LINKAGE_ADAPTATION | sys/eventfd.h | 35 | BLOCKED_TARGET |
| eventfd_write | G_ABI_LINKAGE_ADAPTATION | sys/eventfd.h | 32 | BLOCKED_TARGET |
| pipe2 | E_SYSCALL_BACKED_IMPLEMENTATION | unistd.h | 260 | BLOCKED_TARGET |
| dup3 | E_SYSCALL_BACKED_IMPLEMENTATION | unistd.h | 236 | BLOCKED_TARGET |
| accept4 | E_SYSCALL_BACKED_IMPLEMENTATION | sys/socket.h | 274 | BLOCKED_TARGET |

## No-depth-limit retained source/build scans

| Source root | Regular files traversed | Source/build files scanned | Bytes scanned | Oversized source/build files skipped |
|---|---:|---:|---:|---:|
| state/full_android_language_toolchain/src/llvm-project | 158696 | 84728 | 822757798 | 1 |
| state/full_android_language_toolchain/src/rust | 239685 | 169538 | 660457315 | 0 |
| state/full_android_language_toolchain/src/cpython | 5549 | 3826 | 86181977 | 0 |

## Required native closure chain

Each discovered requirement follows the established Braxon chain: source-derived inventory → classification → staged header when necessary → native implementation only when necessary → AArch64 compilation → archive/shared overlay → symbol inspection → consumer compile/link → target execution → downstream consumer probe → machine-readable evidence → NSQ/Reflexor registration → canonicality validation.

## Required target sequence

1. `scripts/braxon_reconstruct.sh preflight`
2. `scripts/braxon_reconstruct.sh source-edge`
3. `BRAXON_SOURCE_BUILD_APPROVED=1 JOBS=1 scripts/braxon_reconstruct.sh source-build`
4. `BRAXON_SOURCE_BUILD_APPROVED=1 JOBS=1 scripts/braxon_reconstruct.sh edge-nightly-build`
5. `scripts/braxon_reconstruct.sh calibrate`
6. `scripts/braxon_reconstruct.sh verify`

## Truth state

Native contract family: **BLOCKED_TARGET**. Rust edge nightly: **REPOSITORY_CONTAINED_NATIVE_PROMOTION_PATH_IMPLEMENTED_TARGET_BUILD_PENDING**. CPython and LLVM source/build routes are repository-contained but are not target-complete until their own actual receipts exist. The NSQ Reflexor route is canonical and on-demand; no resident runtime is constructed.

## Evidence files

The sibling JSON artifacts in this directory contain the complete toolchain, missing-symbol, contract, ABI, semantic Reflexor, provenance, licensing, prerequisite, and final-closure records.
