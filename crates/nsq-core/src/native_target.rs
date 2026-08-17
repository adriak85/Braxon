use crate::{NSQSlot, NsqAddress, NsqInstruction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeNsqTargetField {
    pub target: NsqAddress,
    pub desired: NSQSlot,
    pub watermark: Option<NSQSlot>,
}

impl NativeNsqTargetField {
    pub fn new(target: NsqAddress, desired: NSQSlot) -> Self {
        Self {
            target,
            desired,
            watermark: None,
        }
    }
    pub fn reconcile(&mut self, observed: Option<&NSQSlot>) -> Option<NsqInstruction> {
        if observed == Some(&self.desired) {
            self.watermark = Some(self.desired.clone());
            None
        } else {
            self.watermark = Some(self.desired.clone());
            Some(NsqInstruction::Set {
                address: self.target.clone(),
                value: self.desired.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Charge, Dialect, NSQLever};

    fn address(position: u64) -> NsqAddress {
        NsqAddress::root(NSQSlot::new(
            Dialect::Graphics,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        ))
    }
    fn slot(position: u64) -> NSQSlot {
        NSQSlot::new(
            Dialect::Graphics,
            vec![NSQLever::new(Charge::Positive, position).unwrap()],
        )
    }

    #[test]
    fn target_field_emits_native_delta_then_quiets_at_watermark() {
        let target = address(1);
        let desired = slot(2);
        let mut field = NativeNsqTargetField::new(target.clone(), desired.clone());
        assert!(
            matches!(field.reconcile(None), Some(NsqInstruction::Set { address, value }) if address == target && value == desired)
        );
        assert!(field.reconcile(Some(&desired)).is_none());
        assert_eq!(field.watermark, Some(desired));
    }
}
