use nsq_citadel::IntentSeed;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const WOWAS_REALIZATION_SCHEMA: &str = "braxon.wowas.realization.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasRealizedPacket {
    pub packet_id: String,
    pub book_num: u32,
    pub book_code: String,
    pub title: String,
    pub kind: String,
    pub status: String,
    pub ordinal: u32,
    pub source_character_id: String,
    pub source_character_name: String,
    pub source_role: String,
    pub source_region: String,
    pub source_anchor: String,
    pub encounter_id: String,
    pub event_id: String,
    pub core_intent: String,
    pub universal_intent: String,
    pub prose_gate: String,
    pub runtime_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasWorldStateDelta {
    pub state_id: String,
    pub book_num: u32,
    pub packet_id: String,
    pub domain: String,
    pub key: String,
    pub value: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WowasRealization {
    pub schema: String,
    pub series: String,
    pub book_count: u32,
    pub packet_count: u32,
    pub packets: Vec<WowasRealizedPacket>,
    pub world_state: Vec<WowasWorldStateDelta>,
    pub realization_hash: String,
}

impl WowasRealization {
    pub fn from_ordered_manifest(raw: &str) -> Result<Self, String> {
        let value: serde_json::Value = serde_json::from_str(raw)
            .map_err(|error| format!("invalid WOWAS ordered realization manifest: {error}"))?;
        let series = value
            .get("series")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "realization manifest requires series".to_string())?;
        let books = value
            .get("books")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| "realization manifest requires books".to_string())?;
        if books.len() != 33 {
            return Err(format!(
                "realization requires exactly 33 books, found {}",
                books.len()
            ));
        }
        let mut packets = Vec::new();
        let mut ids = BTreeSet::new();
        for (expected, book) in books.iter().enumerate() {
            let order = book
                .get("order")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| "book order missing".to_string())? as u32;
            if order != expected as u32 + 1 {
                return Err(format!(
                    "book order is not contiguous at position {}",
                    expected + 1
                ));
            }
            let book_code = string(book, "book_code")?;
            let book_title = string(book, "title")?;
            let slots = book
                .get("ordered_slots")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(|| format!("book {order} has no ordered slots"))?;
            let mut core_ordinal = 0u32;
            for (index, slot) in slots.iter().enumerate() {
                let packet_id = string(slot, "scene_id")?;
                if !ids.insert(packet_id.clone()) {
                    return Err(format!("duplicate packet id {packet_id}"));
                }
                let kind = string(slot, "kind")?;
                let is_core = kind == "existing_core_scene";
                if is_core {
                    core_ordinal += 1;
                }
                let ordinal = index as u32 + 1;
                let title = string(slot, "title")?;
                let character_id = optional(slot, "source_character_id");
                let character_name = optional(slot, "source_character_name");
                let role = optional(slot, "source_role");
                let region = optional(slot, "source_region");
                let anchor = optional(slot, "source_anchor");
                let encounter_id = optional(slot, "encounter_id");
                let event_id = optional(slot, "event_id");
                let core_intent = if is_core {
                    format!("core_scene;book={order};book_code={book_code};scene={packet_id};title={title}")
                } else {
                    format!("bridge_scene;book={order};book_code={book_code};packet={packet_id};title={title};character={character_id};role={role};region={region};anchor={anchor};encounter={encounter_id};event={event_id};function={}", string(book, "function")?)
                };
                let universal_intent = format!("nsq.intent.v1::{}", stable_hash(&core_intent));
                let prose_gate = if is_core {
                    "canonical_scene_existing"
                } else {
                    "requires_prose_realization"
                };
                packets.push(WowasRealizedPacket {
                    packet_id: packet_id.clone(),
                    book_num: order,
                    book_code: book_code.clone(),
                    title,
                    kind: kind.clone(),
                    status: string(slot, "status")?,
                    ordinal,
                    source_character_id: character_id,
                    source_character_name: character_name,
                    source_role: role,
                    source_region: region,
                    source_anchor: anchor,
                    encounter_id,
                    event_id,
                    core_intent,
                    universal_intent,
                    prose_gate: prose_gate.into(),
                    runtime_ready: true,
                });
            }
            if core_ordinal > 0 && core_ordinal as usize > slots.len() {
                return Err(format!("book {order} core ordinal invalid"));
            }
            let _ = book_title;
        }
        let world_state = packets
            .iter()
            .map(|packet| WowasWorldStateDelta {
                state_id: format!("state:{}", stable_hash(&packet.universal_intent)),
                book_num: packet.book_num,
                packet_id: packet.packet_id.clone(),
                domain: "wowas_scene_runtime".into(),
                key: format!("{}.{}", packet.book_code, packet.kind),
                value: packet.core_intent.clone(),
                source: "wowas_ordered_stretched_spine".into(),
            })
            .collect::<Vec<_>>();
        let digest_input = packets
            .iter()
            .map(|p| p.universal_intent.as_str())
            .collect::<Vec<_>>()
            .join("|");
        Ok(Self {
            schema: WOWAS_REALIZATION_SCHEMA.into(),
            series: series.into(),
            book_count: 33,
            packet_count: packets.len() as u32,
            packets,
            world_state,
            realization_hash: stable_hash(&digest_input),
        })
    }

    pub fn packet_intent_seed(&self, packet_id: &str) -> Result<IntentSeed, String> {
        let packet = self
            .packets
            .iter()
            .find(|p| p.packet_id == packet_id)
            .ok_or_else(|| format!("unknown realization packet {packet_id}"))?;
        Ok(IntentSeed::new(&packet.packet_id, &packet.core_intent))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != WOWAS_REALIZATION_SCHEMA
            || self.book_count != 33
            || self.packet_count as usize != self.packets.len()
        {
            return Err("WOWAS realization metadata mismatch".into());
        }
        let mut last_book = 0u32;
        let mut last_ordinal = 0u32;
        for packet in &self.packets {
            if packet.book_num < last_book {
                return Err("WOWAS packets are out of book order".into());
            }
            if packet.book_num == last_book && packet.ordinal <= last_ordinal {
                return Err(format!(
                    "WOWAS packet order invalid in book {}",
                    packet.book_num
                ));
            }
            if !packet.runtime_ready || packet.universal_intent.is_empty() {
                return Err(format!("packet {} is not runtime-ready", packet.packet_id));
            }
            last_book = packet.book_num;
            last_ordinal = packet.ordinal;
        }
        Ok(())
    }
}

fn string(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .filter(|v| !v.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("manifest field {field} is required"))
}
fn optional(value: &serde_json::Value, field: &str) -> String {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn stable_hash(value: &str) -> String {
    let mut hash = 14695981039346656037u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_non_33_book_manifest() {
        assert!(WowasRealization::from_ordered_manifest(r#"{"series":"x","books":[]}"#).is_err());
    }
    #[test]
    fn deterministic_realization_has_runtime_packets() {
        let raw = include_str!("../../../config/wowas/ordered_stretched_spine_manifest.json");
        let one = WowasRealization::from_ordered_manifest(raw).unwrap();
        let two = WowasRealization::from_ordered_manifest(raw).unwrap();
        assert_eq!(one, two);
        assert_eq!(one.book_count, 33);
        assert!(one.packet_count > 100);
        one.validate().unwrap();
        assert!(one.packet_intent_seed("B01_C001").is_ok());
    }
}
