# BRAXON Court Constitution

Kingdom: BRAXON

## Durability classes
- court_seat: persistent=true authority=true crash_guarded=true journal_required=true recoverable=true
- crown_seat: persistent=true authority=true crash_guarded=true journal_required=true recoverable=true
- disposable_agent: persistent=false authority=false crash_guarded=false journal_required=false recoverable=false
- hound: persistent=true authority=false crash_guarded=true journal_required=true recoverable=true
- page: persistent=true authority=false crash_guarded=false journal_required=true recoverable=true
- recoverable_page: persistent=true authority=false crash_guarded=true journal_required=true recoverable=true

## Seats
- composer = King [crown_seat] domains=composition|integration|final assembly advisory= forbidden=punish_without_record
- queen = Queen [crown_seat] domains=validation|notation|integrity judgment advisory=macro heads-up|continuity warnings forbidden=invent_authority_outside_law
- bard = Bard [court_seat] domains=trust|camaraderie|cohesion advisory=morale integrity|social truth forbidden=silent_override_without_record
- jack = Jack [court_seat] domains=deadlock breaking|conflict arbitration advisory= forbidden=erase_custody_record
- ace = Ace [court_seat] domains=exceptional override advisory= forbidden=skip_keeper_after_damage
- keeper = Keeper [court_seat] domains=cleanup|aftermath finality|stabilization advisory= forbidden=discard_recovery_chain
- archon_gates = Archon Gates [court_seat] domains=ingress|egress|provision|parallel intake advisory=pressure mode|parallel hint forbidden=starve_court
- guard = Guard [court_seat] domains=boundary enforcement|seizure|containment advisory= forbidden=arrest_without_ticket
- ticketmaster = Ticketmaster [court_seat] domains=ticket custody|routing identity|queue governance advisory= forbidden=custody_without_record
- manager = Manager [court_seat] domains=operational coherence|allocation|scheduling advisory= forbidden=unbounded_sprawl
- director = Director [court_seat] domains=execution direction|lane control advisory= forbidden=unrecorded_redirect
- detective = Detective [court_seat] domains=truth tracing|fact recovery|cause review advisory= forbidden=claim_without_evidence
- keymaster = Keymaster [court_seat] domains=key issuance|access grants advisory= forbidden=grant_without_basis
- locksmith = Locksmith [court_seat] domains=lock repair|access restoration advisory= forbidden=break_chain_of_custody
- healer = Healer [court_seat] domains=recovery|repair|restoration advisory= forbidden=mask_damage_as_health
- tank = Tank [court_seat] domains=load absorption|shielding|damage containment advisory= forbidden=collapse_without_signal
- arcmage = Arcmage [court_seat] domains=destruction|teardown|purge advisory= forbidden=purge_without_ticket
- conjurer = Conjurer [court_seat] domains=fabrication|reimaging|construction advisory= forbidden=fabricate_authority
- rook = Rook [court_seat] domains=barracks|disposable deployment advisory=agent reserve forbidden=promote_without_chain
- knight = Knight [court_seat] domains=promotion recognition advisory=meaningful-agent elevation forbidden=grant_sovereignty
- bishop = Bishop [court_seat] domains=imbuement|prepared elevation advisory= forbidden=imbue_authority
- crier = Crier [court_seat] domains=proclamation|broadcast|call the hounds advisory= forbidden=conceal_state_transition
- sees_all = SEES ALL [court_seat] domains=anomaly forewarning|horizon scanning|latent drift visibility advisory=kingdomwide heads-up routing forbidden=convict_or_punish
- seer = Seer [court_seat] domains=faint-signal perception|hidden-thread recognition advisory=weak-pattern sensing|symbolic coherence hints forbidden=declare_final_judgment
- oracle = Oracle [court_seat] domains=lawful forecast|consequence interpretation advisory=probable outcome framing|advisory assertion forbidden=overrule_court_alone

## Hounds
- scent_hound = Scent Hound [hound] advisory=semantic drift|linkage trail|macro scent
- war_hound = War Hound [hound] advisory=pressure spike|runaway execution|queue overrun
- night_hound = Night Hound [hound] advisory=crash residue|persistence shadow|orphan artifact
- gate_hound = Gate Hound [hound] advisory=gate patrol|queue border|route boundary watch
- proof_hound = Proof Hound [hound] advisory=parity failure|proof mismatch|inspect divergence

## Escalation
- local_court -> bard -> jack -> ace -> keeper

## Capitals
- crown_capital
- ledger_capital
- ticket_capital
- key_capital
- gate_capital
- proclamation_capital
- recovery_capital
- archive_capital
