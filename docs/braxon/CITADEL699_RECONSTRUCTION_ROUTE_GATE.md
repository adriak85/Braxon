# Citadel699 Reconstruction Route Gate

Citadel699 does not require ordinary full local FP32 weight storage as the source truth.

The intended model route is:

```text
input handle
  -> bus record
  -> stamp wake
  -> stored operation/framework
  -> materialization recipe
  -> output state
  -> validation digest
  -> moral invariant check
  -> identity preservation check```

## Pointer-looking files are not automatically failure

A small file that looks like a pointer is inert only when no reconstruction route evidence exists.

If that file is named by a bus record, reconstruction manifest, seed pack, wake packet, stamp registry, materialization recipe, validation digest, moral invariant check, or identity preservation check, Braxon must classify it as a reconstruction handle until the route verifier proves or rejects the route.

## Status ladder

```text
inert_pointer_stub_or_catalog_only
  -> reconstruction_handle_unverified
  -> reconstruction_route_verified_not_hot_live
  -> reconstruction_route_executed_not_runtime_bound
  -> hot_live_verified
```

## Hot-live rule

Metadata alone is not hot-live.

A manifest alone is not hot-live.

A reserved artifact name alone is not hot-live.

Hot-live requires executable route proof naming the input handle, bus record, stamp wake, stored operation or framework, materialization recipe, output state, validation digest, moral invariant check, and identity preservation check.

## Citadel699 boundary

Citadel699 is the ten-lane uncensored model stack.

All ten lanes require emotional routing, moral invariant preservation, identity preservation, NSQ authority validation, and route execution proof before hot-live claims.

The safety boundary is the emotional-routing layer plus moral invariant preservation, identity preservation, NSQ authority, stamp wake validation, materialization proof, and executable runtime proof.

## Repository honesty

Do not downgrade reconstruction handles to inert pointer stubs just because they are small.

Do not upgrade reconstruction handles to hot-live just because a manifest, bus marker, or reserved artifact name exists.

The truthful middle states are:

```text
reconstruction_handle_unverified
reconstruction_route_verified_not_hot_live
reconstruction_route_executed_not_runtime_bound
```
