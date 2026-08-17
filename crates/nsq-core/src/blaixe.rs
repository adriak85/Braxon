//! Blaixe is the isolated name-space for the direct NSQ runtime boundary.
//! It intentionally contains aliases, not a second semantic or binary runtime.

pub use crate::native_runtime::{
    NativeNsqMachine, NativeNsqOwnership, NativeNsqRuntime, NsqActuator, NsqAddress, NsqInstruction,
};
pub use crate::{NSQLever, NSQSlot};

/// The direct in-process bus endpoint is an NSQ address, not a MAC byte array.
pub type DirectBusAddress = NsqAddress;

/// Hardware MAC identifiers are an external Android/network adapter concern.
/// They must never enter the NSQ executable representation.
pub const BLAIXE_ADDRESSING_POLICY: &str = "nsq-address-native; mac-external-only; no-root-safe";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Charge, Dialect};

    #[test]
    fn blaixe_address_is_native_nsq_without_a_binary_mac_layer() {
        let address = DirectBusAddress::root(NSQSlot::new(
            Dialect::Control,
            vec![NSQLever::new(Charge::Positive, 1).unwrap()],
        ));
        assert_eq!(address.path.len(), 1);
        assert!(BLAIXE_ADDRESSING_POLICY.contains("mac-external-only"));
    }
}
