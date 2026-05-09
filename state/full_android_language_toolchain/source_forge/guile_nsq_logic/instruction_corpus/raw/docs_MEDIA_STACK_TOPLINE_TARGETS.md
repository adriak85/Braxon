# Phase Three Media Stack Targets

Current truth:
The workspace still lacks bound media-stack presence on the audited windowing, UI shell, GPU, Android media, audio I/O, and decode lanes.

## Topline target stack
Window and surface bridge:
- winit
- raw-window-handle

GPU runtime:
- wgpu
- naga
- ash

UI shell:
- egui
- egui-winit
- eframe only if you intentionally want the higher-level shell path

Android native integration:
- ndk

Audio I/O:
- oboe

Audio decode and playback orchestration:
- symphonia
- kira

## Phase Three tests to write before claiming readiness
1. dependency_presence_media_stack
- assert root workspace metadata includes the chosen media crates

2. capability_matrix_promotes_windowing
- windowing present only when both winit and raw-window-handle are present

3. capability_matrix_promotes_gpu_compat
- gpu_compat present only when wgpu is present

4. capability_matrix_promotes_gpu_flagship
- gpu_flagship present only when ash and naga are both present

5. capability_matrix_promotes_android_media
- android media lane present only when ndk is present
- audio I/O lane present only when oboe is present

6. capability_matrix_promotes_audio_decode
- audio decode lane present only when symphonia or kira-backed decode path is present

7. fail_closed_media_absence
- runtime and docs must not advertise top-line media readiness while the matrix is false

## Phase Three file targets
- crates/braxon-core/tests/media_capability_matrix.rs
- docs/MEDIA_STACK_TOPLINE_TARGETS.md
- state/braxon/braxon_capability_matrix.json

## Ready-to-bind order
1. winit + raw-window-handle
2. wgpu + naga
3. ash
4. ndk
5. oboe
6. symphonia
7. kira
8. egui + egui-winit
