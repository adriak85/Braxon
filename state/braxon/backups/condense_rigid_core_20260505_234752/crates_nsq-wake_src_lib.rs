//! NSQ Wake System — Stamp Interception and Framework Hydration
//!
//! When a stamp is thrown to a build position, the wake system intercepts it,
//! retrieves the precompiled framework bundle for that stamp type, hydrates
//! the symbol table at the target position, and initiates execution without
//! cold-start cost.
//!
//! Without the wake system, stamps land in a void. The precompiled frameworks
//! exist but nothing initiates them. The stamp dispatch is complete but
//! nothing builds around it. nsq-wake is the crate that closes this loop.
//!
//! Wake law:
//!   stamp thrown → StampInterceptor catches it at the target position
//!   → FrameworkCache retrieves the precompiled bundle (already compiled, not built on demand)
//!   → SymbolHydrator resolves surrounding app symbols at the target position
//!   → WakeDispatch initiates the framework — no cold start, no rebuild
//!
//! The framework was waiting. Not building. Waiting.
//! The stamp is the signal. The wake is the response.

use nsq_core::{Dialect, NSQLever, NSQSlot, Nu16, Nu64};
use nsq_core::intent::{
    CouncilPole, IntentPressure, IntentSurface, NsqIntentScaleAnchor, NsqIntentVariable,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Stamp identity ─────────────────────────────────────────────────────────

/// A stamp identity — uniquely identifies a stamp type in the framework cache.
/// A stamp is an NSQSlot with Dialect::Stamp. Its identity is derived from
/// the lever positions in its body. The wake system keys its cache on this.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StampIdentity {
    /// The canonical NSQ representation of the stamp's lever body.
    pub canonical_nsq: String,
    /// The stamp class — what kind of framework this stamp triggers.
    pub stamp_class: StampClass,
    /// The build position this stamp was thrown to.
    pub target_build_position: Nu64,
}

impl StampIdentity {
    pub fn from_slot(slot: &NSQSlot, target_build_position: Nu64) -> Result<Self, String> {
        if slot.dialect != Dialect::Stamp {
            return Err(format!(
                "StampIdentity requires Dialect::Stamp, got {:?}",
                slot.dialect
            ));
        }
        let canonical_nsq = slot.to_nsq();
        let stamp_class = StampClass::from_lever_body(&slot.body);
        Ok(Self {
            canonical_nsq,
            stamp_class,
            target_build_position,
        })
    }
}

/// The class of a stamp — determines which framework bundle the cache retrieves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StampClass {
    /// A court macro stamp — triggers a precompiled macro framework.
    CourtMacro,
    /// An agent wake stamp — triggers the agentic dispatch framework.
    AgentWake,
    /// A council dispatch stamp — triggers a council pole dispatch framework.
    CouncilDispatch,
    /// A sensory render stamp — triggers a sensory output framework.
    SensoryRender,
    /// A memory recall stamp — triggers the continuity/recall framework.
    MemoryRecall,
    /// A world-build stamp — triggers the world-body spatial framework.
    WorldBuild,
    /// An unknown stamp class — the framework cache will report a miss.
    Unknown,
}

impl StampClass {
    /// Derive the stamp class from the lever body of the stamp slot.
    /// The first lever position determines the class range.
    pub fn from_lever_body(levers: &[NSQLever]) -> Self {
        let first_position = levers.first().map(|l| l.position).unwrap_or(0);
        match first_position {
            1..=187 => Self::CourtMacro,
            188..=375 => Self::AgentWake,
            376..=562 => Self::CouncilDispatch,
            563..=750 => Self::SensoryRender,
            751..=937 => Self::MemoryRecall,
            938..=1126 => Self::WorldBuild,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CourtMacro => "court_macro",
            Self::AgentWake => "agent_wake",
            Self::CouncilDispatch => "council_dispatch",
            Self::SensoryRender => "sensory_render",
            Self::MemoryRecall => "memory_recall",
            Self::WorldBuild => "world_build",
            Self::Unknown => "unknown",
        }
    }
}

// ── Framework bundle ───────────────────────────────────────────────────────

/// A precompiled framework bundle — stored in the FrameworkCache.
///
/// This is NOT built on demand when a stamp arrives.
/// It is precompiled and stored. The stamp is the key that retrieves it.
/// Cold-start cost is eliminated because the framework was already compiled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkBundle {
    pub stamp_class: StampClass,
    /// The intent pressure context this framework was compiled against.
    pub compiled_intent_context: IntentPressure,
    /// The council poles this framework activates.
    pub activated_poles: Vec<CouncilPole>,
    /// The symbol table precompiled for this framework.
    pub symbol_table: HashMap<String, SymbolEntry>,
    /// The build instructions — what to execute at the target position.
    pub build_instructions: Vec<BuildInstruction>,
    /// True if this bundle has been compiled and is ready for dispatch.
    pub compiled: bool,
    /// The NSQ address this framework targets.
    pub framework_nsq_address: String,
}

impl FrameworkBundle {
    pub fn new(
        stamp_class: StampClass,
        compiled_intent_context: IntentPressure,
        activated_poles: Vec<CouncilPole>,
    ) -> Self {
        Self {
            stamp_class,
            compiled_intent_context,
            activated_poles,
            symbol_table: HashMap::new(),
            build_instructions: Vec::new(),
            compiled: false,
            framework_nsq_address: String::new(),
        }
    }

    pub fn mark_compiled(mut self, nsq_address: impl Into<String>) -> Self {
        self.compiled = true;
        self.framework_nsq_address = nsq_address.into();
        self
    }

    pub fn with_symbol(mut self, key: impl Into<String>, entry: SymbolEntry) -> Self {
        self.symbol_table.insert(key.into(), entry);
        self
    }

    pub fn with_instruction(mut self, instruction: BuildInstruction) -> Self {
        self.build_instructions.push(instruction);
        self
    }
}

/// A single symbol entry in a precompiled framework's symbol table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub symbol_id: String,
    pub symbol_class: SymbolClass,
    pub resolved_address: Nu64,
    pub intent_pressure_at_resolution: IntentPressure,
}

/// The class of a symbol in the framework symbol table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolClass {
    CourtRole,
    CouncilPoleBinding,
    MacroEntry,
    AlgorithmEntry,
    LanguageEntry,
    ControlEntry,
}

/// A single build instruction — what the wake system executes at target position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildInstruction {
    pub instruction_id: String,
    pub instruction_type: InstructionType,
    /// The intent pressure this instruction carries to its target.
    pub intent_pressure: IntentPressure,
    pub target_position: Nu64,
}

/// The type of a build instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionType {
    /// Bind a symbol to its resolved position.
    BindSymbol,
    /// Activate a council pole at this build position.
    ActivatePole,
    /// Route an intent pressure frame to the target.
    RouteIntentPressure,
    /// Initiate a precompiled macro at the target position.
    InitiateMacro,
    /// Mark the build position as ready for requests.
    MarkReady,
}

// ── Framework cache ────────────────────────────────────────────────────────

/// The precompiled framework cache.
///
/// Keyed by StampClass. When a stamp arrives, the cache retrieves
/// the precompiled bundle immediately — no compilation on demand.
///
/// The cache is populated at court boot time. By the time a stamp arrives,
/// every framework it might need is already compiled and waiting.
#[derive(Debug, Default)]
pub struct FrameworkCache {
    bundles: HashMap<StampClass, FrameworkBundle>,
    miss_count: u64,
    hit_count: u64,
}

impl FrameworkCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a precompiled framework bundle in the cache.
    /// Returns an error if the bundle is not marked compiled.
    pub fn store(&mut self, bundle: FrameworkBundle) -> Result<(), String> {
        if !bundle.compiled {
            return Err(format!(
                "cannot cache uncompiled bundle for {:?}; compile it first",
                bundle.stamp_class
            ));
        }
        self.bundles.insert(bundle.stamp_class, bundle);
        Ok(())
    }

    /// Retrieve a precompiled bundle for a stamp class.
    /// Returns None if the stamp class has no precompiled bundle (cache miss).
    pub fn retrieve(&mut self, stamp_class: StampClass) -> Option<&FrameworkBundle> {
        let result = self.bundles.get(&stamp_class);
        if result.is_some() {
            self.hit_count += 1;
        } else {
            self.miss_count += 1;
        }
        result
    }

    pub fn hit_count(&self) -> u64 {
        self.hit_count
    }

    pub fn miss_count(&self) -> u64 {
        self.miss_count
    }

    pub fn is_warm(&self) -> bool {
        !self.bundles.is_empty()
    }

    pub fn compiled_class_count(&self) -> usize {
        self.bundles.len()
    }
}

// ── Symbol hydrator ────────────────────────────────────────────────────────

/// The symbol hydrator — resolves surrounding app symbols at the target position.
///
/// When a stamp arrives and the framework bundle is retrieved, the hydrator
/// builds out the symbol context at the target build position. All symbols
/// the framework references are resolved against the current court state
/// before the framework executes.
#[derive(Debug)]
pub struct SymbolHydrator {
    resolved_count: u64,
    failed_count: u64,
}

impl SymbolHydrator {
    pub fn new() -> Self {
        Self {
            resolved_count: 0,
            failed_count: 0,
        }
    }

    /// Hydrate the symbol table in a framework bundle against the build position.
    /// Returns the count of successfully resolved symbols.
    pub fn hydrate(
        &mut self,
        bundle: &FrameworkBundle,
        build_position: Nu64,
    ) -> HydrationResult {
        let mut resolved = Vec::new();
        let mut failed = Vec::new();

        for (key, entry) in &bundle.symbol_table {
            // In a live system this resolves against the actual court symbol registry.
            // Here we verify the symbol has a non-zero address and its intent pressure
            // is non-zero — sufficient for structural validation.
            let address_valid = entry.resolved_address > 0;
            let intent_valid = entry.intent_pressure_at_resolution
                .variable_positions
                .iter()
                .any(|&pos| pos > 0);

            if address_valid && intent_valid {
                resolved.push(key.clone());
                self.resolved_count += 1;
            } else {
                failed.push(key.clone());
                self.failed_count += 1;
            }
        }

        HydrationResult {
            build_position,
            resolved_symbols: resolved,
            failed_symbols: failed.clone(),
            hydration_complete: failed.is_empty(),
        }
    }

    pub fn resolved_count(&self) -> u64 {
        self.resolved_count
    }

    pub fn failed_count(&self) -> u64 {
        self.failed_count
    }
}

impl Default for SymbolHydrator {
    fn default() -> Self {
        Self::new()
    }
}

/// The result of hydrating a framework's symbol table at a build position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationResult {
    pub build_position: Nu64,
    pub resolved_symbols: Vec<String>,
    pub failed_symbols: Vec<String>,
    pub hydration_complete: bool,
}

// ── Stamp interceptor ──────────────────────────────────────────────────────

/// The stamp interceptor — the entry point of the wake system.
///
/// When a stamp is thrown (an NSQSlot with Dialect::Stamp arrives on
/// IntentSurface::StampDispatch), the interceptor:
///   1. Validates it is a real stamp slot
///   2. Derives the stamp identity and class
///   3. Hands off to WakeDispatch
#[derive(Debug)]
pub struct StampInterceptor {
    intercepted_count: u64,
    rejected_count: u64,
}

impl StampInterceptor {
    pub fn new() -> Self {
        Self {
            intercepted_count: 0,
            rejected_count: 0,
        }
    }

    /// Intercept an intent pressure frame arriving on a stamp dispatch surface.
    /// Returns the stamp identity if the surface and slot are valid.
    pub fn intercept(
        &mut self,
        pressure: &IntentPressure,
        slot: &NSQSlot,
        target_build_position: Nu64,
    ) -> Result<StampIdentity, InterceptError> {
        // Only StampDispatch surface is valid for stamp interception
        if !pressure.court_surface.is_stamp_surface() {
            self.rejected_count += 1;
            return Err(InterceptError::WrongSurface {
                actual: pressure.court_surface.as_str().to_string(),
            });
        }

        // Only Stamp dialect slots are valid
        if slot.dialect != Dialect::Stamp {
            self.rejected_count += 1;
            return Err(InterceptError::WrongDialect);
        }

        let identity = StampIdentity::from_slot(slot, target_build_position)
            .map_err(|e| InterceptError::InvalidStamp(e))?;

        self.intercepted_count += 1;
        Ok(identity)
    }

    pub fn intercepted_count(&self) -> u64 {
        self.intercepted_count
    }

    pub fn rejected_count(&self) -> u64 {
        self.rejected_count
    }
}

impl Default for StampInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

/// An error from the stamp interceptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterceptError {
    WrongSurface { actual: String },
    WrongDialect,
    InvalidStamp(String),
}

impl std::fmt::Display for InterceptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongSurface { actual } => {
                write!(f, "stamp must arrive on StampDispatch surface, got {actual}")
            }
            Self::WrongDialect => write!(f, "stamp slot must use Dialect::Stamp"),
            Self::InvalidStamp(msg) => write!(f, "invalid stamp: {msg}"),
        }
    }
}

// ── Wake dispatch ──────────────────────────────────────────────────────────

/// The wake dispatch — the complete stamp-to-execution pipeline.
///
/// Receives a stamp identity, retrieves the precompiled framework,
/// hydrates its symbols at the target position, and initiates execution.
///
/// This is what makes stamps useful. Without this, a stamp lands
/// and nothing happens. With this, a stamp lands and a complete
/// precompiled framework activates at the target position immediately.
pub struct WakeDispatch {
    pub interceptor: StampInterceptor,
    pub cache: FrameworkCache,
    pub hydrator: SymbolHydrator,
    dispatch_count: u64,
    miss_count: u64,
}

impl WakeDispatch {
    pub fn new() -> Self {
        Self {
            interceptor: StampInterceptor::new(),
            cache: FrameworkCache::new(),
            hydrator: SymbolHydrator::new(),
            dispatch_count: 0,
            miss_count: 0,
        }
    }

    /// The primary wake dispatch entry point.
    ///
    /// Takes a stamp pressure frame and its NSQ slot, retrieves the
    /// precompiled framework, hydrates symbols at the target position,
    /// and returns the wake result. No cold start. No rebuild.
    pub fn dispatch(
        &mut self,
        pressure: &IntentPressure,
        slot: &NSQSlot,
        target_build_position: Nu64,
    ) -> WakeResult {
        // 1. Intercept and validate
        let identity = match self.interceptor.intercept(pressure, slot, target_build_position) {
            Ok(id) => id,
            Err(err) => {
                return WakeResult::intercepted_failed(target_build_position, err);
            }
        };

        // 2. Retrieve precompiled framework
        let bundle = match self.cache.retrieve(identity.stamp_class) {
            Some(b) => b.clone(),
            None => {
                self.miss_count += 1;
                return WakeResult::cache_miss(target_build_position, identity);
            }
        };

        // 3. Hydrate symbols at target position
        let hydration = self.hydrator.hydrate(&bundle, target_build_position);

        // 4. Initiate — the framework is already compiled and waiting
        self.dispatch_count += 1;
        WakeResult::dispatched(target_build_position, identity, bundle, hydration)
    }

    pub fn dispatch_count(&self) -> u64 {
        self.dispatch_count
    }

    pub fn miss_count(&self) -> u64 {
        self.miss_count
    }

    pub fn cache_is_warm(&self) -> bool {
        self.cache.is_warm()
    }
}

impl Default for WakeDispatch {
    fn default() -> Self {
        Self::new()
    }
}

/// The result of a wake dispatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeResult {
    pub target_build_position: Nu64,
    pub status: WakeStatus,
    pub stamp_identity: Option<StampIdentity>,
    pub framework_address: Option<String>,
    pub hydration: Option<HydrationResult>,
    pub initiated: bool,
}

/// The status of a wake dispatch result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WakeStatus {
    /// Framework retrieved, symbols hydrated, execution initiated.
    Initiated,
    /// Stamp did not pass interception (wrong surface or dialect).
    InterceptFailed,
    /// Stamp class had no precompiled framework in the cache.
    CacheMiss,
    /// Symbol hydration was incomplete (some symbols failed to resolve).
    HydrationIncomplete,
}

impl WakeResult {
    fn dispatched(
        position: Nu64,
        identity: StampIdentity,
        bundle: FrameworkBundle,
        hydration: HydrationResult,
    ) -> Self {
        let status = if hydration.hydration_complete {
            WakeStatus::Initiated
        } else {
            WakeStatus::HydrationIncomplete
        };
        Self {
            target_build_position: position,
            status,
            stamp_identity: Some(identity),
            framework_address: Some(bundle.framework_nsq_address),
            hydration: Some(hydration),
            initiated: true,
        }
    }

    fn cache_miss(position: Nu64, identity: StampIdentity) -> Self {
        Self {
            target_build_position: position,
            status: WakeStatus::CacheMiss,
            stamp_identity: Some(identity),
            framework_address: None,
            hydration: None,
            initiated: false,
        }
    }

    fn intercepted_failed(position: Nu64, _err: InterceptError) -> Self {
        Self {
            target_build_position: position,
            status: WakeStatus::InterceptFailed,
            stamp_identity: None,
            framework_address: None,
            hydration: None,
            initiated: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nsq_core::{Charge, NSQLever};

    fn make_stamp_slot(first_position: Nu16) -> NSQSlot {
        NSQSlot::new(
            Dialect::Stamp,
            vec![NSQLever { charge: Charge::Positive, position: first_position }],
        )
    }

    fn make_stamp_pressure() -> IntentPressure {
        let mut pressure = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        pressure.court_surface = IntentSurface::StampDispatch;
        pressure
    }

    fn make_compiled_bundle(stamp_class: StampClass) -> FrameworkBundle {
        FrameworkBundle::new(
            stamp_class,
            IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale),
            vec![CouncilPole::MaverickLogic],
        )
        .mark_compiled("nsq://framework/court_macro/v1")
    }

    #[test]
    fn stamp_class_derived_from_lever_position() {
        assert_eq!(StampClass::from_lever_body(&[NSQLever { charge: Charge::Positive, position: 1 }]), StampClass::CourtMacro);
        assert_eq!(StampClass::from_lever_body(&[NSQLever { charge: Charge::Positive, position: 400 }]), StampClass::CouncilDispatch);
        assert_eq!(StampClass::from_lever_body(&[NSQLever { charge: Charge::Positive, position: 1000 }]), StampClass::WorldBuild);
        assert_eq!(StampClass::from_lever_body(&[]), StampClass::Unknown);
    }

    #[test]
    fn interceptor_rejects_wrong_surface() {
        let mut interceptor = StampInterceptor::new();
        let pressure = IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale);
        // Default surface is Internal, not StampDispatch
        let slot = make_stamp_slot(100);
        let result = interceptor.intercept(&pressure, &slot, 1000);
        assert!(matches!(result, Err(InterceptError::WrongSurface { .. })));
        assert_eq!(interceptor.rejected_count(), 1);
    }

    #[test]
    fn interceptor_rejects_wrong_dialect() {
        let mut interceptor = StampInterceptor::new();
        let pressure = make_stamp_pressure();
        let slot = NSQSlot::new(
            Dialect::Intent,
            vec![NSQLever { charge: Charge::Positive, position: 100 }],
        );
        let result = interceptor.intercept(&pressure, &slot, 1000);
        assert!(matches!(result, Err(InterceptError::WrongDialect)));
    }

    #[test]
    fn interceptor_accepts_valid_stamp() {
        let mut interceptor = StampInterceptor::new();
        let pressure = make_stamp_pressure();
        let slot = make_stamp_slot(100);
        let result = interceptor.intercept(&pressure, &slot, 9999);
        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity.stamp_class, StampClass::CourtMacro);
        assert_eq!(identity.target_build_position, 9999);
        assert_eq!(interceptor.intercepted_count(), 1);
    }

    #[test]
    fn cache_miss_on_unwarmed_cache() {
        let mut cache = FrameworkCache::new();
        assert!(!cache.is_warm());
        assert!(cache.retrieve(StampClass::CourtMacro).is_none());
        assert_eq!(cache.miss_count(), 1);
    }

    #[test]
    fn cache_stores_and_retrieves_compiled_bundle() {
        let mut cache = FrameworkCache::new();
        let bundle = make_compiled_bundle(StampClass::CourtMacro);
        cache.store(bundle).unwrap();
        assert!(cache.is_warm());
        assert!(cache.retrieve(StampClass::CourtMacro).is_some());
        assert_eq!(cache.hit_count(), 1);
    }

    #[test]
    fn cache_rejects_uncompiled_bundle() {
        let mut cache = FrameworkCache::new();
        let bundle = FrameworkBundle::new(
            StampClass::CourtMacro,
            IntentPressure::baseline(NsqIntentScaleAnchor::SystemWorldScale),
            vec![],
        );
        assert!(cache.store(bundle).is_err());
    }

    #[test]
    fn wake_dispatch_cache_miss_when_cold() {
        let mut wake = WakeDispatch::new();
        let pressure = make_stamp_pressure();
        let slot = make_stamp_slot(100);
        let result = wake.dispatch(&pressure, &slot, 1000);
        assert_eq!(result.status, WakeStatus::CacheMiss);
        assert!(!result.initiated);
        assert_eq!(wake.miss_count(), 1);
    }

    #[test]
    fn wake_dispatch_initiates_when_cache_warm() {
        let mut wake = WakeDispatch::new();
        let bundle = make_compiled_bundle(StampClass::CourtMacro);
        wake.cache.store(bundle).unwrap();

        let pressure = make_stamp_pressure();
        let slot = make_stamp_slot(100);
        let result = wake.dispatch(&pressure, &slot, 1000);

        // Hydration completes (empty symbol table = trivially complete)
        assert_eq!(result.status, WakeStatus::Initiated);
        assert!(result.initiated);
        assert_eq!(wake.dispatch_count(), 1);
        assert_eq!(wake.miss_count(), 0);
    }

    #[test]
    fn wake_dispatch_is_not_cold_start() {
        // When the cache is warm, retrieval is O(1) — no compilation on dispatch.
        // This test verifies the bundle was precompiled (compiled flag set)
        // before the wake dispatch, not during it.
        let mut cache = FrameworkCache::new();
        let bundle = make_compiled_bundle(StampClass::AgentWake);
        assert!(bundle.compiled, "bundle must be precompiled before dispatch");
        cache.store(bundle).unwrap();

        let retrieved = cache.retrieve(StampClass::AgentWake).unwrap();
        assert!(retrieved.compiled, "retrieved bundle must already be compiled");
    }
}
