# NSQ Language and Documentation Contract Index

This index is an **executable-context companion** to the Braxon context manifest. It provides stable identities, repository paths, NSQ capability routes, and explicit evidence boundaries for documentation and language-facing surfaces. It does not claim that a capability contract is equivalent to an installed external runtime, a parsed external source corpus, or model-weight execution.

| Canonical ID | Contract surface | Repository authority | NSQ capability route | Evidence boundary |
|---|---|---|---|---|
| `braxon.documentation.index` | Documentation index | `docs/NSQ_LANGUAGE_CONTRACT_INDEX.md` | `documentation.index` | Repository documentation index only |
| `braxon.guile.contract` | Guile control-plane contract | `crates/nsq-core/src/raw_intent_engine.rs` | `guile.rebuild_intent` | Capability contract; external Guile installation is checked separately |
| `braxon.apropos.contract` | Apropos discovery contract | `crates/nsq-core/src/raw_intent_engine.rs` | `apropos.discover` | Capability contract; installed apropos database is checked separately |
| `braxon.tree_sitter.contract` | Syntax parsing contract | `crates/nsq-core/src/raw_intent_engine.rs` | `tree_sitter.parse` | Capability contract; parser artifact and AST inventory are checked separately |
| `braxon.ast.contract` | Structured syntax contract | `crates/nsq-core/src/syntax_intent.rs` (`NsqSyntaxTree`) | `tree_sitter.parse` | Repository syntax-intent structure, not a claim of a tree-sitter grammar binary |
| `braxon.tokenizer.contract` | Boundary tokenization contract | `crates/braxon-core/src/native_model_substrate.rs` | `tokenizer.boundary` | Native-band inventories and translation mappings are checked separately |

Every record is addressed through `config/braxon_context_manifest.json` and the chain-root registry. The closure verifier must resolve each record from identity to manifest pointer, canonical address, source path, capability route, and released on-demand receipt. A missing source, route, chain record, parser artifact, or model-specific mapping is a failed or blocked closure gate rather than an inferred success.

## Runtime separation

> **Hard state** and **derived state** may cross runtime boundaries. **User presentation** and **narrative** may not be committed as hard runtime state.

The language contracts in this index are repository context. They do not start a resident Guile service, parser service, model, or GUI process.
