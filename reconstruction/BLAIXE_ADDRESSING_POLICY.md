# Blaixe Addressing Policy

## Decision

Blaixe is an isolated namespace over the native NSQ runtime, not a second semantic runtime. Its direct in-process bus endpoint is `DirectBusAddress = NsqAddress`, and its executable dispatch primitive is `BlaixeBus::dispatch(&NsqAddress, &[NsqInstruction])`.

A MAC address is **not** used as the NSQ execution address. MAC identifiers belong to the external network or hardware-adapter boundary and must not be copied into the executable NSQ representation as bytes, strings, or stable ownership keys.

## Rationale

The official Android Open Source Project documentation states that Android 10 and higher use randomized MAC addresses by default. It also documents persistent and non-persistent randomization, with non-persistent addresses potentially regenerated at connection events on Android 12 and higher.[1] Therefore, a Wi-Fi MAC is not a reliable stable identity for an in-process NSQ bus on a no-root Android target.

The supported architecture is consequently:

| Boundary | Authoritative identity | Operation |
|---|---|---|
| NSQ/Blaixe in-process execution | `NsqAddress` composed of `NSQSlot` values | Native `Set`, `Release`, and `Fire` transactions |
| Piston ownership | Native NSQ address and generation | Same-space exclusion and release-before-reacquisition |
| Android native surface | OS-owned native surface/window handles | Direct NDK surface and input operations where permitted |
| Wi-Fi/Bluetooth/network adapter | Adapter/network APIs and OS-managed MAC state | External transport only; never the NSQ runtime key |

This preserves direct operation without pretending that an application can ethically or reliably claim privileged control over Android’s hardware MAC or physical bus resources without root, vendor privileges, or an explicit device-owner/adapter contract.

## Executable evidence

`crates/nsq-core/src/blaixe.rs` defines the direct address policy and `crates/nsq-core/src/blaixe_bus.rs` implements address-checked native dispatch. Unknown endpoints fail closed. `Braxon-core::NativeNsqStack` now composes the Blaixe direct bus with council arbitration, Ghost Memory, reflexor, and Target Field paths. The focused suites pass with 15 nsq-core tests and 47 Braxon-core tests, without introducing a binary MAC payload path.

## References

[1]: https://source.android.com/docs/core/connect/wifi-mac-randomization-behavior "Android Open Source Project: MAC randomization behavior"
