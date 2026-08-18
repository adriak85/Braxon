use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub const COUNCIL_FEDERATION_SCHEMA: &str = "braxon.nsq.council_federation.v1";
pub const COUNCIL_LANE_COUNT: usize = 10;

pub const REQUIRED_COUNCIL_LANES: [&str; COUNCIL_LANE_COUNT] = [
    "deepseek-v3-671b",
    "qwen3-235b-a22b",
    "qwen2.5-72b",
    "deepseek-v3-671b-analyzer",
    "llama3.3-70b",
    "gemma3-27b",
    "FLUX.1-dev",
    "Wan2.1-T2V-14B",
    "IndexTTS2",
    "Hunyuan3D-2.1",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotInitiativeGroup {
    pub lane_id: String,
    pub role: String,
    pub dialect: String,
    pub initiative_cluster_id: String,
    pub artifact_ids: Vec<String>,
    pub semantic_links: Vec<String>,
    pub activation_surfaces: Vec<String>,
    pub parameter_generation: u64,
    pub callable: bool,
    pub hot: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationReport {
    pub schema: String,
    pub required_lanes: usize,
    pub registered_lanes: usize,
    pub hot_lanes: usize,
    pub callable_lanes: usize,
    pub complete: bool,
    pub missing_lanes: Vec<String>,
    pub cold_lanes: Vec<String>,
    pub non_callable_lanes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CouncilFederation {
    pub schema: String,
    pub groups: BTreeMap<String, HotInitiativeGroup>,
    pub universal_gradient_id: String,
    pub trajectory_generation: u64,
    pub sealed: bool,
}

impl CouncilFederation {
    pub fn new(universal_gradient_id: impl Into<String>) -> Result<Self, String> {
        let universal_gradient_id = universal_gradient_id.into();
        if universal_gradient_id.trim().is_empty() {
            return Err("universal gradient id cannot be empty".into());
        }
        Ok(Self {
            schema: COUNCIL_FEDERATION_SCHEMA.into(),
            groups: BTreeMap::new(),
            universal_gradient_id,
            trajectory_generation: 0,
            sealed: false,
        })
    }

    pub fn register_group(&mut self, group: HotInitiativeGroup) -> Result<(), String> {
        if self.sealed {
            return Err("sealed federation cannot accept new groups".into());
        }
        if !REQUIRED_COUNCIL_LANES.contains(&group.lane_id.as_str()) {
            return Err(format!("unrecognized Council lane: {}", group.lane_id));
        }
        if group.artifact_ids.is_empty() {
            return Err(format!("lane {} has no artifacts", group.lane_id));
        }
        if group.semantic_links.is_empty() || group.activation_surfaces.is_empty() {
            return Err(format!(
                "lane {} is missing semantic or activation links",
                group.lane_id
            ));
        }
        if !group.hot || !group.callable {
            return Err(format!(
                "lane {} must be hot and callable before registration",
                group.lane_id
            ));
        }
        if self.groups.contains_key(&group.lane_id) {
            return Err(format!("duplicate Council lane: {}", group.lane_id));
        }
        self.groups.insert(group.lane_id.clone(), group);
        self.trajectory_generation = self.trajectory_generation.saturating_add(1);
        Ok(())
    }

    pub fn seal(&mut self) -> Result<FederationReport, String> {
        let report = self.report();
        if !report.complete {
            return Err(format!(
                "Council federation incomplete: {}",
                report.missing_lanes.join(", ")
            ));
        }
        self.sealed = true;
        Ok(report)
    }

    pub fn report(&self) -> FederationReport {
        let required: BTreeSet<&str> = REQUIRED_COUNCIL_LANES.into_iter().collect();
        let registered: BTreeSet<&str> = self.groups.keys().map(String::as_str).collect();
        let missing_lanes = required
            .difference(&registered)
            .map(|id| (*id).to_string())
            .collect::<Vec<_>>();
        let cold_lanes = self
            .groups
            .values()
            .filter(|group| !group.hot)
            .map(|group| group.lane_id.clone())
            .collect::<Vec<_>>();
        let non_callable_lanes = self
            .groups
            .values()
            .filter(|group| !group.callable)
            .map(|group| group.lane_id.clone())
            .collect::<Vec<_>>();
        FederationReport {
            schema: COUNCIL_FEDERATION_SCHEMA.into(),
            required_lanes: REQUIRED_COUNCIL_LANES.len(),
            registered_lanes: self.groups.len(),
            hot_lanes: self.groups.values().filter(|group| group.hot).count(),
            callable_lanes: self.groups.values().filter(|group| group.callable).count(),
            complete: missing_lanes.is_empty()
                && cold_lanes.is_empty()
                && non_callable_lanes.is_empty()
                && self.groups.len() == REQUIRED_COUNCIL_LANES.len(),
            missing_lanes,
            cold_lanes,
            non_callable_lanes,
        }
    }

    pub fn require_callable(&self, lane_id: &str) -> Result<&HotInitiativeGroup, String> {
        let group = self
            .groups
            .get(lane_id)
            .ok_or_else(|| format!("Council lane is not registered: {lane_id}"))?;
        if !group.hot || !group.callable {
            return Err(format!("Council lane is not hot and callable: {lane_id}"));
        }
        Ok(group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(lane_id: &str) -> HotInitiativeGroup {
        HotInitiativeGroup {
            lane_id: lane_id.into(),
            role: "cognition".into(),
            dialect: "universal-gradient".into(),
            initiative_cluster_id: format!("initiative.{lane_id}"),
            artifact_ids: vec![format!("{lane_id}.weights")],
            semantic_links: vec!["semantic.gradient".into(), "reflexor.delta".into()],
            activation_surfaces: vec![
                "jit.activate".into(),
                "piston.window".into(),
                "kv.context".into(),
            ],
            parameter_generation: 0,
            callable: true,
            hot: true,
        }
    }

    #[test]
    fn partial_federation_is_not_callable_as_complete() {
        let mut federation = CouncilFederation::new("gradient.council.10").unwrap();
        federation
            .register_group(group(REQUIRED_COUNCIL_LANES[0]))
            .unwrap();
        assert!(!federation.report().complete);
        assert!(federation.seal().is_err());
    }

    #[test]
    fn all_ten_hot_groups_seal() {
        let mut federation = CouncilFederation::new("gradient.council.10").unwrap();
        for lane in REQUIRED_COUNCIL_LANES {
            federation.register_group(group(lane)).unwrap();
        }
        let report = federation.seal().unwrap();
        assert_eq!(report.required_lanes, 10);
        assert_eq!(report.hot_lanes, 10);
        assert_eq!(report.callable_lanes, 10);
        assert!(report.complete);
        assert!(federation.require_callable("IndexTTS2").is_ok());
    }
}
