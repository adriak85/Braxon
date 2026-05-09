# Braxon Offline Public Access Mission

## Purpose

Braxon must work completely offline after its first lawful source-edge materialization because one of its core intended users is a person with almost no reliable network access.

This includes homeless users, people living out of shelters or cars, people relying on limited government phones, and anyone whose practical internet access may be capped at a few gigabytes of transfer data.

For those users, an online-only assistant is not a real support system. It is another locked door.

## Design Motive

Braxon is being built so that a person with no connection still has something healthy, structured, and constructive to use.

The system should give them something better to do than suffer, spiral, or look for something destructive to tweak on. It should support repair, learning, writing, planning, local organization, technical practice, emotional stabilization, and useful problem solving without requiring a live cloud dependency.

## Offline Requirement

After the one-time source-edge build and translation path completes:

- Braxon must not require GitHub.
- Braxon must not require Hugging Face.
- Braxon must not require CPAN network access.
- Braxon must not require pip, npm, cargo network access, curl, wget, git fetch, or live package download to operate.
- Braxon must not require a paid online inference endpoint.
- Braxon must not treat network availability as a normal runtime assumption.

Network contact may exist only as a constrained source-edge materialization step, where Citadel travels to the source edge, receives raw payload there if needed, translates it to NSQ, and returns or streams NSQ language material. Runtime use after materialization must be local.

## Data Budget Target

The runtime must respect users with extremely limited data.

The design target is not merely lower bandwidth. It is practical dignity under scarcity:

- install once where possible;
- transfer only what is necessary;
- avoid repeated downloads;
- avoid fetch loops;
- avoid telemetry dependency;
- avoid cloud-only capability;
- preserve local usefulness when the user has no data left.

## Healthy-Use Target

The offline system should provide local activities and constructive surfaces that are useful even without the internet:

- writing and editing;
- learning and tutoring from local material;
- local project work;
- repair planning;
- journaling and emotional regulation;
- coding practice;
- offline documentation search;
- worldbuilding and creative work;
- step-by-step technical diagnostics;
- practical planning for food, shelter, documents, benefits, transport, and safety where local resources are already stored.

## Engineering Rule

Offline is not an optional feature.

Offline is a dignity requirement.

A Braxon build is not complete if its normal operation depends on a network that the intended user may not have.
