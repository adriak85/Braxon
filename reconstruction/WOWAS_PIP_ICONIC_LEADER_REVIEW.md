# WOWAS Pip Iconic-Leader Review

## Core invariant

Pip is an **iconic leader, not a martyr**. His leadership is not based on believing that every outcome will be favorable. It is based on giving his best effort, choosing the best action he can honestly identify with incomplete knowledge, accepting consequences without romanticizing suffering, and continuing to build a better world with other people.

The governing sequence is:

> **Build. Build love. Love. Live. Live.**

This is treated as an action-and-continuity invariant, not as a slogan that replaces decisions. After loss, Pip preserves memory without becoming trapped inside it. He accepts help, delegates, revises a choice when evidence changes, protects life without treating his own erasure as the default price, and makes rebuilding a shared practice.

## Coverage

The constraint layer covers **13,852 Pip-linked metadata records** across the reconciled scene and candidate surface. Every row is keyed by `record_id`; `scene_id` is retained only as contextual metadata because scene IDs can repeat across source lineages.

| Encounter mode | Rows | Function |
|---|---:|---|
| World-pressure witness | 8,086 | The world answers Pip’s choices materially; he adapts and repairs rather than performing sacrifice. |
| Builder or beneficiary | 4,325 | Other characters contribute to rebuilding and receive consequences; Pip is a coordinator, not a solitary savior. |
| Peer or witness | 1,435 | Characters respond from their own values rather than automatic worship or dependence. |
| Challenger or adversary | 6 | Opposition tests Pip’s reasoning and exposes costs or blind spots without making self-erasure the answer. |

These modes are structured constraints. They do not generate prose and do not force every encounter into the same emotional shape.

## Dynamic requirements

Every Pip-linked realization must preserve four distinctions. First, **best effort is not guaranteed success**; failure can be real without proving that Pip should have sacrificed more of himself. Second, **leadership is distributed**; other characters must contribute knowledge, care, labor, refusal, correction, or courage. Third, **love and life are active forces**; they appear through choices, repair, humor, food, work, shelter, memory, and continued relationship rather than only through declarations. Fourth, **loss changes state**; it cannot be reset between books or used only as decorative tragedy.

No encounter may reduce another character to a worshipper, dependent, disposable victim, or copy of Pip. Each response mode carries its own values, pressure, and agency. Adversaries may be right about a cost. Witnesses may disagree. Builders may surpass Pip in a domain. Beneficiaries must still act. The world itself can resist him without turning him into a sacrificial symbol.

## Validation

The updated preflight requires `pip_leadership_constraints.tsv`, checks unique `record_id` values, confirms the non-martyr invariant on every Pip constraint row, confirms `prose_status=no_generated_prose`, and retains the staged tone/style/cadence/token and rolling-state requirements. The preflight passed with prose generation disabled.
