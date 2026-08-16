use nsq_core::{Charge, Dialect, NSQLever, NSQSlot, CANONICAL_LEVER_MAX_POSITION};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitadelBitState { Active, Destabilizing, Reapproaching, Persistent }
impl CitadelBitState { pub fn as_str(self) -> &'static str { match self { Self::Active=>"active", Self::Destabilizing=>"destabilizing", Self::Reapproaching=>"reapproaching", Self::Persistent=>"persistent" } } }

pub struct CitadelBit { pub lane:u128, pub state:CitadelBitState, pub instructions:Vec<NSQSlot>, pub priority:u16 }
impl CitadelBit {
    pub fn new(lane:u128)->Self{Self{lane,state:CitadelBitState::Persistent,instructions:Vec::new(),priority:255}}
    pub fn check_in(&mut self, input:Vec<NSQSlot>){ if self.state==CitadelBitState::Destabilizing{self.release_and_reapproach();} self.instructions.extend(input); if self.state==CitadelBitState::Persistent{self.state=CitadelBitState::Active;} }
    fn release_and_reapproach(&mut self){self.state=CitadelBitState::Reapproaching;self.instructions.clear();self.instructions.push(NSQSlot::new(Dialect::Intent,vec![NSQLever::new(Charge::Positive,CANONICAL_LEVER_MAX_POSITION).unwrap()]));self.state=CitadelBitState::Active;}
    pub fn is_live(&self)->bool{!self.instructions.is_empty()&&self.state!=CitadelBitState::Destabilizing}
    pub fn pressure_sum(&self)->u64{self.instructions.iter().flat_map(|s|s.body.iter()).filter(|l|l.charge==Charge::Positive).map(|l|l.position).sum()}
}
