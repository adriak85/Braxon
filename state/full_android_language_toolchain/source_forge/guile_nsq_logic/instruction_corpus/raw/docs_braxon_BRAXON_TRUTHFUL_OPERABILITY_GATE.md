# Braxon Truthful Operability Gate

Generated: `2026-04-24T14:47:11.318241+00:00`

## Result

- `assets_ready`: `true`
- `can_attempt_launch`: `true`
- `runtime_route_available`: `true`
- `runtime_route_proven`: `false`
- `loaded_binding_proven`: `false`
- `runtime_hot_live_proven`: `false`
- `final_active_digest_present`: `false`

## Rule

This gate proves only what the phone actually has.

It may mark local assets ready when the full safetensors shard set, tokenizer/config files, NSQ weight artifact, registry, binding file, and manifest are present.

It must not mark runtime hot-live, loaded binding, runtime route proof, or final digest as true until those are proven by a live route execution.
