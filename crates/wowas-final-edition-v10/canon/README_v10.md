# WoWaS Update 10 Patch Bundle

This bundle is the post-v8/post-v9 canon and gameplay patch layer for Braxon ingest.

## What this bundle does
- stages Update 10 as authoritative addendum files
- preserves the v8 core bundle and the v9 polish patch as earlier layers
- adds an install script that creates a normalized `wowas_final_edition_BRAXON_ready_v10` folder in `~/storage/shared/Download`
- writes an apply-order manifest so Braxon can ingest `v8 -> v9 -> v10`

## Intended install order
1. `wowas_final_edition_BRAXON_ready_v8.zip`
2. `wowas_v9_polish_patch.tar.gz`
3. this Update 10 bundle

## Output folder after install
`~/storage/shared/Download/wowas_final_edition_BRAXON_ready_v10`

## Important note
Update 10 is delivered as authoritative patch/control documents and staging manifests. It does **not** pretend that every earlier patch has already been collapsed into one monolithic merged scene index. The design intent is: Braxon loads the base files first, then the v9 polish files, then the v10 addendum/control files.
