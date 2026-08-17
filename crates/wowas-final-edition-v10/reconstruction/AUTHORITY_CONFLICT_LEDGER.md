# Authority Conflict Ledger

These conflicts were found in the source itself and are preserved rather than silently resolved.

## C01 — 25-book vs 33-book surface

- v14 authority: active canonical saga = 25 books.
- `canon/active/book_spine_33.tsv`: 33 rows; B26-B33 are explicitly marked `active_insert` / `inserted_capacity`.
- v14 source-of-truth registry: 25-book structure remains active **unless cast density forces inserted books/volumes** and explicitly allows 33-book expansion.

Resolution: retain the 33-book index as capacity/variant data while treating v14's 25-book core as the current authority until density forces promotion.

## C02 — v14 absorbed patches vs v13 enduring patches

- v14 source-of-truth registry says patch folders are law while ingested and then become history; final control files are the active home.
- v13 authority router says placed patches remain enduring authority and must not be erased/demoted merely because v13 is a router.

Resolution: preserve both statements as versioned authority. v14 controls current active-home routing; v13 remains an audit/router surface. Do not delete the patch files from the historical corpus.

## C03 — v14 scene/character counts vs expanded generated surfaces

- v14 authority reports 14,739 scenes and 51+ tracked characters.
- v14 registry targets 5,000 story characters, 2,000,000 ancillary people, and 5,000 creatures.
- Character registry reports 50 named characters and a tier distribution that totals the 2,000,000 population target.

Resolution: distinguish current authoritative named-cast state from expansion capacity. Counts are not interchangeable.

## C04 — character source contamination

The character registry contains direct source-name lists such as Stitch, Tasslehoff, Barefoot, Asta, Yuno, Gray Fullbuster, Lucy Heartfilia, Drizzt, Menolly, Hermione Granger, F'nor, Erza Scarlet, Treebeard, Groot, Yggdrasil, and others.

Resolution: these are treated as provenance/generation contamination signals. They must not be copied into the active-world identity layer. Replacement candidates require independent identity, morphology, behavior, and world function.

## C05 — creature source contamination

The monster registry has explicit `source_media_register_*` columns and observed records reference recognizable external characters/properties (for example Joey Tribbiani, Professor McGonagall, Nemesis, Kimmy Gibbler, Yuno, Lucy Heartfilia, Lacuna Coil, Bad Santa, and others).

Resolution: retain the source record for audit, but reject the recognizable-source identity from active promotion and route it through originality rehash.

## C06 — runtime retrieval law

`src/lib.rs` and v14 authority both identify scene-level retrieval as the safe runtime mode rather than loading the entire corpus into context.

Resolution: graph reconstruction uses source-addressable segments and SERIEL/index keys; it does not flatten the corpus into one prompt-sized artifact.
