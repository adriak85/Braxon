# Real-world source findings

The official [UNESCO World Heritage List](https://whc.unesco.org/en/list/) exposes stable list identifiers and place records. The page currently reports 1,273 properties across 173 States Parties, with cultural, natural, mixed, and transboundary distinctions. UNESCO records are suitable for landmark and heritage provenance, but their factual descriptions must remain separate from fictional WOWAS transformations.

The official [GeoNames](https://www.geonames.org/) site states that its geographical database covers all countries and contains more than eleven million placenames available for download. It exposes free gazetteer downloads and web services, making it suitable for city, settlement, coordinate, and alternate-name normalization. GeoNames should be used for canonical place identity and coordinates, not as a complete source of cultural interpretation.

The Wikidata city entity [Q515](https://www.wikidata.org/wiki/Q515) demonstrates stable item identifiers and structured claims for city classification, urban settlement relationships, geography/history properties, and linked city entities. Wikidata item IDs can anchor cross-source identity, while individual claims require their own references and should not be treated as unqualified prose.

Converter rule: retain source URL, source identifier, retrieval date, factual claim, and confidence for every real-world record; then create a separate WOWAS transformation record linked by SERIEL. Real-world facts remain `FACT`; fictionalized material is marked `WOWAS_TRANSFORM`; uncertain or interpretive material is marked `REVIEW_REQUIRED`.
