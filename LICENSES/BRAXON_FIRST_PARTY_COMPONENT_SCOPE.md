# Braxon First-Party Component Licensing Scope

**Canonical project name:** Braxon
**Copyright owner:** Michael David Norris
**License identifier:** `LicenseRef-Braxon-Private`

This notice identifies the substantially rebuilt, original, and project-authored components governed by the root [Braxon Private Proprietary License](../LICENSE). It is a component-scope notice, not an attempt to relicense external material.

## Covered First-Party Components

The private proprietary license applies to original Braxon-authored or substantially Braxon-reconstructed material, including the following active component families where the files are not separately marked as third-party:

| Component family | Licensing treatment |
|---|---|
| `crates/braxon-core`, `crates/nsq-*`, and `src` | Project-authored NSQ, Citadel, live-bus, Reflexor, and command-routing implementation |
| `scripts`, `scripts/toolchains`, and `tools/toolchain` | Project-authored build, verification, repository-tool dispatch, provenance, and Android target-boundary automation |
| `config/nsq` and `config/toolchains` | Project-authored semantic contracts, execution policies, compatibility matrices, and canonical configuration |
| `tests`, `docs`, `specs`, and first-party root documentation | Project-authored tests, documentation, system contracts, and operational material |
| `state/full_android_language_toolchain/native/android_libc_extensions` | First-party Android/Bionic compatibility-overlay implementation and its generated proof records, subject to retained public-interface provenance |
| First-party generated inventories under `state` | Derived project records only where they identify their first-party provenance and do not embed or replace upstream source-license terms |

New substantially rebuilt first-party components must carry either a `LicenseRef-Braxon-Private` SPDX header where appropriate or an explicit binding to this scope notice and the root `LICENSE`.

## Explicit Exclusions and Retained Notices

The root private license does **not** alter, remove, or supersede the licenses, copyright notices, trademarks, or attribution requirements of third-party and upstream material. This includes, without limitation:

| Material class | Treatment |
|---|---|
| `vendor/` dependencies and package caches | Retain their own license and notice files |
| Rust, LLVM, CPython, Guile, Zig, and other upstream source archives or extracted source trees | Retain original upstream copyright and license terms, even when an archive or extraction is stored in this repository |
| Third-party model files, tokenizer source material, package artifacts, and imported documentation | Retain the terms and provenance attached to the material |
| Historical archives, reports, and captured manifests | Preserve source-era naming and checksums as evidence; they are not a canonical Braxon identity surface |

> The term **Braxon** is a project identity, not a separate legal owner. Michael David Norris is the sole individual human owner of project-owned intellectual property covered by the root license.

This repository notice is a working ownership and provenance record. It should be reviewed by qualified counsel before it is relied upon as a formal legal instrument.
