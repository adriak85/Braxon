> NOTE: any u16/u32 or similar width-class notation in this file is derived boundary-carrier description only, never canonical NSQ truth.

# NSQ Source Surface

## Purpose
NSQ supports multiple source surfaces that map into preserved semantic records.

Supported surfaces currently include:
- canonical
- sexpr
- lua_shape
- python_shape

These surfaces are not to be erased during canonical compilation.

---

## Rule

Compilation must preserve both:
- the parsed semantic record
- the original source surface line or form

A compiler may normalize for parsing, but it may not destroy source traceability in the canonical artifact.

---

## Canonical semantic families

### noise
Core semantics:
- symbol
- macro_name
- a
- b
- pos
- amp

### triple
Core semantics:
- subject
- relation
- object
- layer
- plane
- anchor
- weight
- flags

### membrane
Core semantics:
- cell
- state
- flux
- gate
- phase

---

## Canonical surface examples

noise willow_song :macro bloom :a 4 :b 9 :pos 120 :amp 48
triple daisy -> protects -> pip :layer 2 :plane 1 :anchor 88 :weight 40 :flags 0
membrane east_gate :state open :flux 24 :gate 1 :phase 2

---

## sexpr examples

(noise willow_song macro bloom a 4 b 9 pos 120 amp 48)
(triple daisy protects pip layer 2 plane 1 anchor 88 weight 40 flags 0)
(membrane east_gate state open flux 24 gate 1 phase 2)

---

## lua_shape examples

noise willow_song macro=bloom a=4 b=9 pos=120 amp=48
triple daisy rel=protects obj=pip layer=2 plane=1 anchor=88 weight=40 flags=0
membrane east_gate state=open flux=24 gate=1 phase=2

---

## python_shape examples

noise(willow_song, macro=bloom, a=4, b=9, pos=120, amp=48)
triple(daisy, rel=protects, obj=pip, layer=2, plane=1, anchor=88, weight=40, flags=0)
membrane(east_gate, state=open, flux=24, gate=1, phase=2)

---

## Invalid behavior

The following is invalid for canonical compilation:
- lowering every source surface into one reduced transport line and treating that as canonical
- replacing direct symbols or relations with integer IDs before writing the native artifact
- defining canonical source behavior in terms of u16/u32 storage classes
