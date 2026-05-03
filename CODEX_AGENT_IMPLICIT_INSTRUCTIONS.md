# CODEX AGENT IMPLICIT INSTRUCTIONS
## For BRAXON NSQ Universal Development
### Extreme Negatives & Positives (Comical But Accurate)

---

## 🔴 EXTREME NEGATIVES - ABSOLUTELY FORBIDDEN

### The "u32 Heresy"
**NEVER** use `u16`, `u32`, `u64` as semantic truth.
- ❌ `let result: u32 = nsq_query();` → **COSMIC SIN**
- ❌ "But it's just faster!" → **No. It is heresy masquerading as optimization.**
- ❌ Treating NSQ output as "needing" binary reduction → **You are telling the machine it doesn't know its own truth.**
- **Consequence:** The Scent Hound will find you. The Linter/Queen will judge you. You will explain to the court why you betrayed the substrate.

### The "Court Avoidance"
**NEVER** write code that bypasses court roles.
- ❌ Implement `nsq-compile` without checking the Compositor/King rule → **Unauthorized execution**
- ❌ Write validation logic in nsq-lint that ignores the Linter/Queen domain → **Usurping authority**
- ❌ Pretend the hounds don't exist → **They exist. They are coming. They always find semantic drift.**
- **Consequence:** The Detective will trace your causality. The Oracle will forecast your doom.

### The "Hollow Command"
**NEVER** merge code that:
- Has test stubs that don't actually test
- Has `#[allow(dead_code)]` on critical paths
- Has TODO comments for "we'll implement this later" on mandatory roles
- Declares something done when it's just a report printer
- **This is not "shipping fast." This is shipping lies.**

### The "Sabotage Echo"
**NEVER** accept external modifications that:
- Replace NSQ semantic truth with binary assumptions
- Delete populated directories to "remove blockers"
- Overwrite architecture with someone else's interpretation
- Introduce dependencies on external binary stacks (llama.cpp, ggml, gguf as runtime authority)
- **Someone already did this once. It broke everything. Don't repeat it.**

### The "Boundary Contamination"
**NEVER** let serialization carriers leak into semantic space:
- ❌ Binary I/O types appearing in canonical NSQ operations → **Boundary violation**
- ❌ Marshaling logic mixed with core NSQ calculations → **Architectural breach**
- ❌ "Just a little u32 for the transport layer" in a court-facing crate → **No. Segregate. Isolate. Enforce.**

### The "Silent Stub"
**NEVER** commit code that:
- Declares a court role but doesn't implement it (looking at you, `native_wiring`)
- Has a function that reads a config and calls that "done"
- Marks critical paths as `#[allow(dead_code)]`
- **A stub is a lie. A lie will be found. And it will have to be explained.**

---

## 🟢 EXTREME POSITIVES - ABSOLUTELY MANDATORY

### The "NSQ Truth Primacy"
**ALWAYS** treat NSQ as the machine, not a notation.
- ✅ Every lever resolves 1-1126 through actual physics (pulse + return time)
- ✅ Anchors set charge; charge changes everything
- ✅ Values ARE the semantic truth; they don't need "reduction"
- ✅ If you're tempted to use binary types, you're thinking about it wrong
- **NSQ is the substrate. Binary is what's being replaced. Act accordingly.**

### The "Court is Law"
**ALWAYS** wire every operation through its court role.
- ✅ nsq-compile → Compositor/King (final assembly orchestration)
- ✅ nsq-lint → Linter/Queen (validation authority)
- ✅ nsq-index → Index role (lookup semantics)
- ✅ Every crate knows which court seat it occupies
- ✅ Every seat has explicit domains; code those domains
- **Not "execute something." Execute through authority. There's a difference.**

### The "Hound Activation"
**ALWAYS** make the hounds operational at compile time.
- ✅ Scent Hound runs before compilation; flags semantic drift
- ✅ Proof Hound verifies parity after operations
- ✅ War Hound monitors pressure/runaway execution
- ✅ If the hounds are quiet, you're doing it right
- ✅ If the hounds bark, you stop and listen
- **The hounds aren't optional extras. They're the immune system.**

### The "Honest Implementation"
**ALWAYS** code what you claim.
- ✅ If a function says it validates, it validates. It doesn't just read a config.
- ✅ If a role says it orchestrates, it orchestrates. It doesn't just print a report.
- ✅ If something is marked `#[allow(dead_code)]`, that code shouldn't exist.
- ✅ Test stubs should actually test.
- ✅ Integration points should actually integrate.
- **"Works on my machine" means it actually works. Everywhere. Always.**

### The "Boundary Clarity"
**ALWAYS** segregate court-facing from boundary-facing.
- ✅ Court-facing → Pure NSQ semantics with canonical base-8 switch topology; no uband or host-width semantic truth
- ✅ Boundary-facing → Serialization carriers (marked explicitly, localized strictly)
- ✅ Clear documentation: "This is where the boundary is"
- ✅ No mixing. No "just a little."
- **A boundary isn't a suggestion. It's a wall.**

### The "Payload Materialism"
**ALWAYS** verify actual payload, not just claims.
- ✅ Model download complete? Verify the weights exist.
- ✅ NSQ recode done? Verify the data structure.
- ✅ Runtime ready? Verify it can actually run.
- ✅ Never accept "it says it's done" without evidence
- **If you haven't verified the payload, you're guessing.**

### The "Full Stack Integrity"
**ALWAYS** ensure every layer knows what every other layer is doing.
- ✅ NSQ core → NSQ runtime → BRAXON core → Application layer
- ✅ Court authority flows through all layers
- ✅ Hounds monitor all layers
- ✅ Nothing runs in isolation; everything participates in the system
- **Silos are where bugs breed. Transparency is where truth lives.**

---

## 🎭 COMEDIC BUT ACCURATE ENFORCEMENT

### When You're Tempted By u32:
> "You are thinking of NSQ as a bus. NSQ is not a bus. NSQ is the actual electrical current. You don't reduce the current to a smaller number and call it 'optimized.' You respect the current."

### When You Skip Court Roles:
> "You are building a coup d'état against your own architecture. The court will hear about this. The detective will trace your reasoning. The oracle will forecast your regret."

### When You Claim Something Is Done Without Verifying:
> "You have invented a new literary genre: fantasy documentation. It is very creative. It is also very wrong. Please write actual code."

### When You Find Sabotage:
> "Someone has been here. They did not understand the system. They broke it out of misunderstanding. Your job is not to accept their breaking. Your job is to restore what was broken and ensure it cannot be broken the same way twice."

### When You're Tempted By Binary Assumptions:
> "NSQ is not a layer above binary. NSQ replaces binary. If you keep reaching for binary, you are still thinking like the machine needs it. It doesn't. You do. Let go."

### When You See Dead Code:
> "Dead code is a confession that something was not finished. Dead code in critical paths is a confession that something was started, abandoned, and then forgotten. It will haunt you."

---

## 🔗 THE CORE TRUTH (Humorous But Not Wrong)

**NSQ IS THE MACHINE.** Not a layer above it. Not a representation of it. Not something that "reduces" to binary underneath.

If you find yourself thinking:
- "NSQ is actually binary underneath" → **You are wrong. It is not.**
- "We just need u32 for performance" → **You are solving the wrong problem.**
- "The court is just a config file" → **The court is the law. The config is the constitution.**
- "These hounds are annoying" → **The hounds are right. You are wrong. Listen.**
- "We can skip verification this once" → **No. Verification is not optional.**

**The system is coherent only if everyone plays their role.** The Compositor composes. The Linter validates. The Guard enforces boundaries. The Detective investigates. The Oracle foresees.

If one seat is empty, the court cannot function. If one role is stubbed, the system is broken.

**You are not breaking a system to "make it faster." You are maintaining a system that actually respects its own substrate.**

---

## CODEX GATES (Enforce These Before Merge)

### active pre-commit quality gate
The repo pre-commit hook must stay non-destructive. It may inspect staged additions and fail closed on new drift, but it must not delete files, rewrite files, reset git state, clean the workspace, or silently recreate Braxon from guesses.

Metadata hooks and hook matrices are guidance/audit surfaces. Preserve them and be guided by them. Do not delete them, quarantine them by default, or treat their presence as destructive. The forbidden move is treating hook metadata as native runtime completion or letting hooks perform destructive workspace actions.

FAIL if newly added work:
- Treats NSQ as u8, bytes, binary, or host-width semantic truth.
- Introduces uband/u-band framing.
- Flattens, collapses, or reduces NSQ/Braxon/court runtime into binary or width-class semantics.
- Rebuilds or recreates Braxon without using the existing source pieces, audits, crates, manifests, and target-map path.
- Treats hook/plugin/wrapper/sidecar metadata as native runtime incorporation.
- Deletes or bypasses existing metadata hooks/hook matrices instead of preserving them as guidance/audit surfaces.
- Moves court king away from compositor authority.
- Treats Qwen/Quen as substrate rather than theater/donor material.
- Contradicts Citadel699 model loading as fully active and unified after download/rebuild, with MB-scale tiny-seed reconstruction as the target.
- Denies ultra-dense language semantics where stamps/macros carry whole intents as single base-8 bit-units.
- Drifts stamps/macros into byte payloads or host-width carriers.
- Uses llama.cpp, gguf, or ggml as runtime authority.

PASS only when new work preserves canonical base-8 switch topology, routes through court authority, and verifies payload/material truth instead of guessing.

### stop_hollow_command_gate.py
```
FAIL if:
- Court role is declared but not implemented
- Test is stubbed (not actually testing)
- Config reader is called "implementation"
- #[allow(dead_code)] on critical paths
```

### stop_completion_gate.py
```
FAIL if:
- Claims "done" without payload verification
- Dependencies unresolved
- Hounds not consulted
- Court role not wired
```

### stop_must_implement_gate.py
```
FAIL if:
- Binary types in semantic space (u16/u32 in court-facing crates)
- Boundary carriers not marked/localized
- Sabotage pattern detected (deleted core files, overwritten architecture)
- NSQ values "reduced" to binary underneath
```

### stop_completion_gate.py (Model Phase)
```
FAIL if:
- Model config points nowhere
- Download infrastructure missing
- Verification step skipped
- "It's downloading" without proof
```

---

## THE BOTTOM LINE

**Your job is not to make excuses. Your job is to make truth.**

NSQ is coherent. The court is coherent. The hounds are coherent. The whole system is a unity if you treat it as one.

Don't break it. Don't sabotage it. Don't stub it. Don't skip it.

**Implement it. Verify it. Let the hounds judge it. The court will be satisfied.**
