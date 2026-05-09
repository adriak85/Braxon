# NSQ damage report v2

## flattening and integer-lane contamination

### /data/data/com.termux/files/home/Braxon/crates/nsq-compile/src/main.rs
- severity: DIRECT
- counts:
  - put_u16: 15
  - put_u32: 3
  - btree_u16: 3
  - sym_order: 11
  - macro_order: 8
  - symbol_id_class: 3
  - macro_id_class: 3
  - u16: 12
  - u32: 3
- first_hits:
  - line 14: put_u16
  - line 14: u16
  - line 15: put_u32
  - line 15: u32
  - line 17: btree_u16
  - line 17: u16
  - line 21: u16
  - line 30: u16
  - line 31: u32
  - line 46: symbol_id_class
  - line 47: macro_id_class
  - line 73: symbol_id_class
  - line 74: macro_id_class
  - line 343: btree_u16
  - line 343: u16
  - line 344: sym_order
  - line 345: btree_u16
  - line 345: u16
  - line 346: macro_order
  - line 348: u32

### /data/data/com.termux/files/home/Braxon/crates/nsq-index/src/lib.rs
- severity: DIRECT
- counts:
  - put_u16: 11
  - put_u32: 10
  - hash_u32: 3
  - u16: 21
  - u32: 46
- first_hits:
  - line 9: u32
  - line 10: u32
  - line 11: u32
  - line 12: u16
  - line 13: u16
  - line 14: u16
  - line 14: u32
  - line 15: u16
  - line 16: u16
  - line 16: u32
  - line 19: u32
  - line 39: u32
  - line 40: u16
  - line 48: u16
  - line 71: hash_u32
  - line 71: u32
  - line 73: u32
  - line 74: u32
  - line 75: u32
  - line 76: u32

### /data/data/com.termux/files/home/Braxon/crates/nsq-pressure-bench/src/main.rs
- severity: DIRECT
- counts:
  - put_u16: 15
  - put_u32: 3
  - u16: 18
  - u32: 8
- first_hits:
  - line 58: put_u16
  - line 58: u16
  - line 61: put_u32
  - line 61: u32
  - line 67: u16
  - line 68: u16
  - line 69: u16
  - line 73: u32
  - line 74: u32
  - line 75: u32
  - line 130: put_u16
  - line 130: u16
  - line 131: put_u16
  - line 131: u16
  - line 134: put_u16
  - line 134: u16
  - line 139: put_u16
  - line 139: u16
  - line 156: u16
  - line 160: u16

### /data/data/com.termux/files/home/Braxon/crates/nsq-real-bench/src/main.rs
- severity: DIRECT
- counts:
  - hash_u32: 1
  - u32: 4
- first_hits:
  - line 10: u32
  - line 11: u32
  - line 70: hash_u32
  - line 70: u32
  - line 80: u32

### /data/data/com.termux/files/home/Braxon/crates/nsq-optimize/src/main.rs
- severity: STRUCTURAL
- counts:
  - symbol_id_class: 2
  - macro_id_class: 2
  - u16: 8
  - u32: 8
- first_hits:
  - line 19: u32
  - line 20: u16
  - line 21: u16
  - line 26: symbol_id_class
  - line 27: macro_id_class
  - line 92: u16
  - line 93: u32
  - line 96: u32
  - line 97: u32
  - line 98: u16
  - line 98: u32
  - line 99: u32
  - line 102: u16
  - line 103: u16
  - line 156: u32
  - line 185: u32
  - line 226: u16
  - line 231: u16
  - line 354: symbol_id_class
  - line 355: macro_id_class

### /data/data/com.termux/files/home/Braxon/crates/nsq-calibrate/src/main.rs
- severity: STRUCTURAL
- counts:
  - symbol_id_class: 3
  - macro_id_class: 3
- first_hits:
  - line 28: symbol_id_class
  - line 29: macro_id_class
  - line 73: symbol_id_class
  - line 74: macro_id_class
  - line 134: symbol_id_class
  - line 135: macro_id_class

### /data/data/com.termux/files/home/Braxon/crates/nsq-generate/src/main.rs
- severity: RESIDUAL
- counts:
  - u16: 5
  - u32: 2
- first_hits:
  - line 85: u32
  - line 86: u16
  - line 94: u32
  - line 95: u16
  - line 102: u16
  - line 110: u16
  - line 111: u16

### /data/data/com.termux/files/home/Braxon/specs/nsq/native_artifact.md
- severity: STRUCTURAL
- counts:
  - u16: 13
  - u32: 2
  - macro_count_u16: 1
  - macro_table_u16: 1
- first_hits:
  - line 6: u16
  - line 7: u16
  - line 8: macro_count_u16
  - line 8: u16
  - line 22: u16
  - line 23: macro_table_u16
  - line 23: u16
  - line 26: u32
  - line 27: u16
  - line 31: u16
  - line 32: u16
  - line 33: u16
  - line 36: u32
  - line 37: u16
  - line 42: u16
  - line 43: u16
  - line 44: u16

### /data/data/com.termux/files/home/Braxon/specs/nsq/source_surface.md
- severity: RESIDUAL
- counts:
  - u16: 3
  - u32: 2
- first_hits:
  - line 9: u16
  - line 9: u32
  - line 12: u16
  - line 12: u32
  - line 15: u16

## priority cut order

- DIRECT: /data/data/com.termux/files/home/Braxon/crates/nsq-compile/src/main.rs
- DIRECT: /data/data/com.termux/files/home/Braxon/crates/nsq-index/src/lib.rs
- DIRECT: /data/data/com.termux/files/home/Braxon/crates/nsq-pressure-bench/src/main.rs
- DIRECT: /data/data/com.termux/files/home/Braxon/crates/nsq-real-bench/src/main.rs
- STRUCTURAL: /data/data/com.termux/files/home/Braxon/crates/nsq-calibrate/src/main.rs
- STRUCTURAL: /data/data/com.termux/files/home/Braxon/crates/nsq-optimize/src/main.rs
- STRUCTURAL: /data/data/com.termux/files/home/Braxon/specs/nsq/native_artifact.md
- RESIDUAL: /data/data/com.termux/files/home/Braxon/crates/nsq-generate/src/main.rs
- RESIDUAL: /data/data/com.termux/files/home/Braxon/specs/nsq/source_surface.md
