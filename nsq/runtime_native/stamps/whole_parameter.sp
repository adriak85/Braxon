id: nsq.runtime.native.model.parameter.whole.v1
type: stamp
version: 1
authority: Braxon
role: whole_parameter_reference_form

meaning:
  one whole-parameter stamp represents stored parameter authority for lazy runtime access

forms:
  - keep the loaded model portion stamp-bound instead of flattening parameter truth into host-width carriers
  - allow each stored parameter to be factored through a single-bit shim reference under NSQ routing
  - allow env-side parameter copies only under explicit lazy-load boundary demand

wake_key: p1
