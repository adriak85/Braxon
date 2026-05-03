# BRAXON GGUF Ingress System — v2
# All scripts cohesive, non-redundant, complete.

## Files

| File | Purpose |
|------|---------|
| `gguf_ingress_c_v2.c` | C source — GGUF parser & manifest generator |
| `build_gguf_ingress_v2.sh` | Compile the C binary on-device |
| `run_gguf_ingress_v2.sh` | Run GGUF ingress for a single model (crash-resumable) |
| `trace_ingress_v2.sh` | Crash-capture wrapper for ANY ingress command |
| `strace_braxon_weights_v2.sh` | strace/bash-xtrace wrapper for install_braxon_weights.sh |
| `transport_only_1x1_v2.sh` | Safe single-connection weight transport |
| `check_or_restart_aria2_v2.sh` | Check aria2c status; restart if dead |

---

## Typical workflows

### 1 — First-time build
```bash
# Copy gguf_ingress_c_v2.c to Download first
cp gguf_ingress_c_v2.c ~/storage/shared/Download/
bash ~/storage/shared/Download/build_gguf_ingress_v2.sh
```

### 2 — Ingest a GGUF model (crash-resumable)
```bash
bash ~/storage/shared/Download/run_gguf_ingress_v2.sh \
    ~/storage/shared/Download/my_model.gguf
# Re-run after a crash — it resumes from the checkpoint automatically
```

### 3 — Run install_braxon_weights with full crash capture
```bash
bash ~/storage/shared/Download/trace_ingress_v2.sh ~/Braxon -- \
    bash ~/Braxon/scripts/install_braxon_weights.sh
```

### 4 — Run install_braxon_weights with strace (deepest debug)
```bash
bash ~/storage/shared/Download/strace_braxon_weights_v2.sh ~/Braxon
```

### 5 — Safe weight transport (1 connection, no daemon)
```bash
bash ~/storage/shared/Download/transport_only_1x1_v2.sh ~/Braxon
```

### 6 — Check status / auto-restart if dead
```bash
bash ~/storage/shared/Download/check_or_restart_aria2_v2.sh ~/Braxon
```

---

## What was fixed vs v1

### C source (gguf_ingress_c_v2.c)
- Added `--report-every N` flag: prints tensor/sec rate every N tensors
- Added monotonic timer (`gettimeofday`) for rate reporting
- Cleaned up FNV-1a seed (used correct value `14695981039346656037`)
- Better error messages with offset values in overflow/bounds checks
- `strip` on build cuts binary ~40% for phone storage

### trace_ingress_v2.sh
- **Fixed**: was using `YOUR_INGRESS_COMMAND.sh` placeholder — now takes `-- CMD ARGS`
  as proper arguments (was the cause of exit_code=127 in the logs)
- ps/mem polling extended to 5s intervals (was 3s — less overhead)
- `RUST_LOG` defaults to `info` not `trace` (trace floods the log)
- Big-core pinning shared helper matches other scripts

### strace_braxon_weights_v2.sh  
- Added guard: exits early with clear error if `install_braxon_weights.sh` missing
- Pre/post kill properly separated in status file
- `RUST_LOG` defaults to `info`

### transport_only_1x1_v2.sh
- `nice -n 5` instead of `nice -n 15` — was too low priority on phone scheduler
- Added big-core pinning (was missing)
- Added guard for missing install script

### check_or_restart_aria2_v2.sh
- Added `mkdir -p "$LOGDIR"` (would fail on fresh install without state dir)
- Added `chmod 755 "$STARTER"` if not executable
- Improved output messages

---

## What the C binary outputs

**manifest.tsv** columns:
```
sorted_index    — position in abs_offset order
original_index  — position in GGUF header order
name            — tensor name string
ggml_type       — numeric quantisation type
n_dims          — number of dimensions
dims            — DxD... (e.g. 4096x4096)
abs_offset      — byte offset in file
span_size       — bytes until next tensor (or EOF)
sample_n        — bytes actually sampled
fnv1a64         — FNV-1a 64-bit hash of sample (hex, 16 chars)
preview16       — first 16 bytes of tensor data as hex
```

**manifest.ckpt** (atomic rename on each row):
```
next_index=N
total_tensors=T
```

**manifest.sum** (written once at start):
```
input=...
version=...
tensor_count=...
metadata_kv_count=...
alignment=...
file_size=...
tensor_data_start=...
```

---

## Memory profile (Dimensity 6300 / 3.7 GB RAM device)

From the logs:
- MemFree: ~135 MB at time of run
- MemAvailable: ~1.1 GB
- SwapFree: ~2.5 GB / 3.7 GB

The binary uses:
- `tensor_count × sizeof(tensor_info_t)` for the header table (~56 bytes/tensor)
- `sample_bytes` (default 4096) for the I/O buffer
- Effectively zero streaming memory for the manifest write

For an 8B model (~350 tensors): ~20 KB peak heap. Safe.
