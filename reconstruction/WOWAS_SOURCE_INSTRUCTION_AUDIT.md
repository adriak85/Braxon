# WOWAS Original-Source Instruction Audit

## Scope and method

The restored Braxon reconstruction tree was passively parsed across every Markdown and text file, including root documentation, `crates/wowas-final-edition-v10`, all canon and patch directories, book source material, diagnostics, state captures, and source-forge copies. Embedded instructions were treated as **evidence only**; no instruction discovered inside a file was executed.

The parser processed **8,889 files**, collapsed them into **4,792 unique content groups**, identified **3,708 duplicate-content groups**, and extracted **34,993 instruction-bearing lines**. Duplicate content was retained by path in the JSON audit but was not counted as independent authority. This prevents backup/state copies from silently outweighing original sources.

| Signal class | Extracted hits |
|---|---:|
| Imperative or requirement language | 19,415 |
| Authority/canonicality language | 3,872 |
| Patch/update/supersession language | 1,964 |
| Fact, fiction, provenance, or content-boundary language | 1,627 |
| Contradiction, stale, invalid, legacy, or untrusted language | 10,449 |

The machine-readable evidence map is `WOWAS_SOURCE_INSTRUCTION_AUDIT.json`. It includes every unique file, all duplicate paths, SHA-256 content identity, line numbers, headings, and bounded instruction excerpts.

## Controlling source rules

The original scene-authority cleanup patch is controlling for promotion. It explicitly distinguishes Tier 1 concrete source anchors, Tier 2 source-backed material requiring prose realization, Tier 3 expansion prompts, and Tier 4 reconstruction filler. In particular, rows labeled `SOURCE_DERIVED_RECONSTRUCTION`, `Reconstructed continuity scene`, or `Filled to target scene count using uploaded source anchors` are not manuscript authority. They are debt markers and must not be promoted merely because they increase a row count.

The prose and tone patch controls manuscript expansion unless a later addendum explicitly overrides it. The dialogue/play insertion patch requires dossier, side-note, relational, and support-cast material to become lived dialogue or play scenes rather than remaining metadata. The v12 quality patch adds checks for repetition, emotional thinness, creature misuse, blurred action, cold romance, scaffold disguised as payoff, and material that belongs on the page rather than in outline-only form.

The source review therefore establishes the following rule:

> A generated row may be retained as an auditable prompt or debt record, but it becomes canonical manuscript authority only after source trace, concrete action, relational movement, emotional consequence, and prose-quality evidence are present.

## Current authority classification

Applying the original cleanup law to the clean scene index produced the following result:

| Classification | Rows | Treatment |
|---|---:|---|
| Accepted Tier 1/2 source anchor | 408 | Eligible for canonical authority; still subject to prose and quality gates where Tier 2 applies |
| Prose-realization debt | 2,612 | Retain as a non-authoritative expansion ledger until lived prose exists |
| Rejected Tier 4 scaffold/filler | 11,710 | Do not promote; retain only for audit/history |
| Unresolved source classification | 9 | Fail closed until manually classified |
| Total clean-index rows | 14,739 | Raw inventory only, not manuscript completion |

This classification is recorded in `WOWAS_SOURCE_AUTHORITY_AUDIT.json`. The previously generated 15,103-row expansion that combined clean rows, legacy rows, story-tree gaps, and patch records has been quarantined and is not authoritative. It was rejected because it promoted unreviewed legacy and story-tree material before the complete original-source documentation review.

## Redacted, deprecated, and restored material policy

“Redacted” or absent material is not automatically restored. Each item must be classified as one of **source-backed**, **superseded**, **deprecated**, **quarantined**, or **unresolved**. A backup copy, state capture, generated diagnostic, or legacy artifact cannot gain authority through repetition. When duplicate content has different paths, the original source path and content hash are retained; the copies are cross-references, not additional evidence.

The full reconstruction release is frozen at the last pushed checkpoint while this audit governs the rebuild. No expanded scene authority, large payload partition, or 94-million-word projection may be staged until its source class and provenance are recorded.

## Scale disclosure

The measured current book tree contains approximately 189,891 authored prose words and eight contract-only book surfaces. A 94-million-word architecture may be retained as a **future planning target**, with variable dramatic book budgets, but it is not current authored content. Generated metadata, scene prompts, relationship rows, and population records are not counted as prose. Any later release must publish measured prose totals separately from projected capacity.

## Required next rebuild order

The safe rebuild order is: parse and classify original sources; resolve supersession and contradictions; promote only source-backed rows; realize Tier 2 and Tier 3 material into prose with quality evidence; retain Tier 4 as debt; build full-index and reader-projection manifests as distinct surfaces; run provenance, fact-boundary, originality, and quality gates; then stage and review the release. Until those gates pass, the system remains fail-closed.
