#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
DL="$HOME/storage/shared/Download"
OUT="$DL/wowas_update10_bundle"
mkdir -p "$OUT"

cat > "$OUT/README_v10.md" <<'EOF_README'
# WoWaS Update 10 Patch Bundle

This bundle is the post-v8/post-v9 canon and gameplay patch layer for Braxon ingest.

## What this bundle does
- stages Update 10 as authoritative addendum files
- preserves the v8 core bundle and the v9 polish patch as earlier layers
- adds an install script that creates a normalized `wowas_final_edition_BRAXON_ready_v10` folder in `~/storage/shared/Download`
- writes an apply-order manifest so Braxon can ingest `v8 -> v9 -> v10`

## Intended install order
1. `wowas_final_edition_BRAXON_ready_v8.zip`
2. `wowas_v9_polish_patch.tar.gz`
3. this Update 10 bundle

## Output folder after install
`~/storage/shared/Download/wowas_final_edition_BRAXON_ready_v10`

## Important note
Update 10 is delivered as authoritative patch/control documents and staging manifests. It does **not** pretend that every earlier patch has already been collapsed into one monolithic merged scene index. The design intent is: Braxon loads the base files first, then the v9 polish files, then the v10 addendum/control files.
EOF_README

cat > "$OUT/wowas_final_canon_control_bundle_v10_addendum.md" <<'EOF_CANON'
# WoWaS Final Canon Control Bundle — Update 10 Addendum

## Purpose
Update 10 locks the post-v8/post-v9 changes discussed after the first Braxon-ready bundle so Braxon ingests the newer canon instead of the older partial assumptions.

## 1) Kyreal / school-era benchmark law
- Kyreal is a **wolf**, one of Pip's best friends, bipedal, and sword-bearing.
- In the school-like survival-magic environment, Kyreal was the strongest in the class in visible performance.
- Pip was already more powerful than Kyreal but did not understand that yet.
- Rylos and Pip both worked harder than anyone because they were visibly weaker in that school-era hierarchy.
- Rylos compensated by learning to be **quiet as a shadow**.
- Pip compensated by obsessive practice.
- Kyreal source-stack update: **Toph**, **Cedric Diggory**, **Sturm**.
- Kyreal wields a **greatsword with the ease of a rapier** and controls **stone**.
- Kyreal later talks Pip into coming out with him to try to help him forget Rylos.
- Kyreal still reads as the strongest in class to Pip long after Pip has surpassed him.

## 2) Diamond power law correction
- The Diamond was conjured to **steal Pip and Rylos's power**.
- Mack intervened by diving in and holding the Diamond back.
- The raw energy of **Mack + Pip + Rylos** filling the Diamond to be drained is what gave the other trapped people power.
- The same Diamond-state is what turned the trapped people into animals while they were inside it.
- When Pip and Rylos enter the portal state, they become **human** because they are no longer under that same Diamond-expression law.

## 3) Pip-only dragon egg law
- The gemstone dragon-egg / shell event belongs **only to Pip**.
- Pip does not understand what is happening.
- No one around him understands what is happening.
- The event must be experienced first as fear, obstruction, and impossible transformation.

## 4) Rylos shadow law
- Rylos has precursor moments of: *did my shadow just move?* followed by denial.
- After the threshold event, his shadow is no longer fully attached and has a mind of its own on his side.
- During the fatal Cockatrice return sequence, shadow-threshold violation becomes possible.
- The strongest locked version is: the threshold opens wrong, Pip pulls Rylos back, and in that rescue instant Cockatrice-ripper runs Rylos through.

## 5) Vaedric / Cockatrice law
- True name: **Vaedric Maldor Obstineth**.
- Public fear-title: **the Cockatrice**.
- In anthropomorphic form he reads more like a terrible rooster/man than a reptilian fantasy basilisk.
- He is roughly ten years older than Rylos and establishes a coercive trauma-bond by promising origin-truth and making Rylos feel dangerous.
- He keeps Rylos hidden from Mack.
- During the altar-of-Neith sequence, he is in almost-attractive human form.
- He tries to use Rylos as a bargaining chip because Neith has something he wants.
- He cuts himself on the red obsidian blade, opens the void, and the invasion begins as cost of his own life.
- Dervish senses the altar activation and seeks Pip because of it.
- Vaedric later returns once as a ripper.
- On that final return, he deals the mortal blow to Rylos.
- A shadow-threshold consequence can carry Vaedric out of clean death and leave later-series recurrence potential without using ordinary resurrection logic.

## 6) Ursula / Mack / core reveal law
- Ursula is not physically seen until the party reaches the core of her realm.
- Ursula reveals Pip's parentage: **Ursula + Mack**.
- Ursula's physical body is gone; she persists in geode-form.
- Ursula is the amethyst surrounding the planet's ice core.
- Mack is the hot ice core.
- Ursula gives Pip an amethyst crystal that can end Mack's failing current life so he can return to core-state.
- As she is dying, she wants to hear Mack's creation song one more time.
- The creation-song / seed logic and Neith's desire for the seed belong to this reveal complex.

## 7) Silence / Lucent / Ursula rewind law
- In the Silence, the air is wrong and wings do not generate lift; glide does not function there.
- Glide returns in Lucent / Ursula realm after the stone break and rewind.
- Breaking Ursula's stone returns the party to the surface in a near-rewind to just before entry.
- Items acquired inside disappear.
- Scars gained inside are reversed.
- **Kyreal's death remains real**.
- If Rolzen's save/heal line was not completed, Rolzen still returns to normal form but permanently thinks less of the player.
- If Rolzen's save/heal line was not completed, Flux is not learned.
- If trust with the people of the Silence is failed, they treat the player as dangerous and remove the ground beneath the party, dropping Pip through the pit without Quantum unlock.

## 8) Quantum + Flux survivability law
- Quantum is Xethrolund's line.
- Flux is Rolzen's line.
- Both are required for a survivable victory against Neith.
- If either one is missing, the player may reach a battle-winning state but the outcome collapses into total fatality.

## 9) Blood Cello judgment law
- If Pip does not play, the world continues as it is.
- If Pip plays but has not done the right things, he may die playing.
- Wrong magic use ages Pip.
- Spending essence wrongly, making undead, or acting against alignment all age him.
- Nihilist outcome: walk away / no song.
- Sadist outcome: mockery song.
- Love outcome: Pip's true song.
- If the player reaches Neith old, alone, corrupt, or unsupported, endings fail in different diagnosable ways.
- Old-man mockery ending: Neith turns Pip into a string marionette and plays him like a cello.
- If the player ignores Rylos's death and slaughters those who try to stop him, the mockery route is reached by real corruption rather than arbitrary branch choice.

## 10) Angel-diamond transfiguration law
- At the end of the attack on the angels, Pip converts the dead into diamonds.
- He is left staring at the burden and the amethyst answers him.
- The amethyst rises, summons the diamonds, drains the charge out of Pip's staff until it falls back into an inert twig, and returns the power in form.
- Good alignment: brilliant clear staff with violet amethyst at both ends.
- Evil alignment: crimson blood-gem greatsword with weight and damage-over-time signature.
- Over-neutrality: no transformation.
- The awakened weapon takes on a life of its own.
- Staff path retains split function into tomahawks or whips.
- Sword path can break at the handle to preserve partial whip functionality.
- Whip traversal pre-existed; **glide** is the new movement expansion.
- In the hardest traversal sections, Pip must first reach stable footing before he can lay a path back for the others.

## 11) Kyreal collapse / first quantum kill law
- After Rolzen's rearrangement, the group learns the Drawn responsible could reverse it.
- Kyreal leads the hunt back out.
- Kyreal freezes.
- Pip touches him.
- Kyreal collapses into what looks like sliced meat.
- The shock is so cold and total that Pip tears loose the quantum field Xethrolund had only been researching.
- He launches a helix / tomahawk / wing-slice kill so quickly the target does not have time to produce a tear.
- This is the **only** time Pip kills before crying.

## 12) Reliquary carry law
- Pip compresses remains into diamond reliquaries as funerary act and emergency emotional focus.
- The medicine pouch at Pip's neck holds the few most intimate ones.
- The black bag holds many more.

## 13) Rolzen / Pip recovery-and-timing law
- Pip loved Rolzen even when Rolzen still looked grotesque and monstrous.
- Rolzen never truly broke; the captors only fully turned people after breaking them first, and the breaking was part of what they enjoyed.
- Once Pip got Rolzen back to his senses after the rippers were gone, it did not take long for Rolzen to become recognizably human in demeanor: sincere, thoughtful, and emotionally present.
- Rolzen flirts with Pip from day one after rescue/recovery, expecting to be turned down.
- Nearly five years later, after a difficult escape, Rolzen makes another half-hearted flirt and Pip finally answers: **"if you want to"**, as if Rolzen must be crazy to be interested.

## 14) Rolzen combat-style correction
- Rolzen's magic is **force / boom**, not fire.
- He uses **crystal balls**, not sais.
- He builds spells through **gravity juggling**, not levitation.
- Default loadout: **six 4-inch crystal spheres** plus one larger top anchor sphere in a David-Bowie/Labyrinth-style stack.
- He can line them up kinetically, strike the line/string, launch one, have it boom, then rebound and catch it back into the pattern.
- He can redirect multiple spheres through flips, kicks, tumbles, and changing vectors.
- He can time ground impacts to suspend or hover himself above opponents for short beats.
- Detonation costs the most energy.
- After detonation the sphere collapses to a bead, which he catches and re-expands in his palm.
- At the end of combat, he swallows the beads to store/recover them.
EOF_CANON

cat > "$OUT/wowas_scene_patch_v10.tsv" <<'EOF_SCENE'
patch_code	target_file	book_band	scene_anchor	action	instruction	reason
v10_scene_001	wowas_clean_scene_index_v2.tsv	B01-B03	School / Diamond origin corridor	insert	Add Kyreal as school-era visible benchmark: wolf, sword, stone-control, strongest in class other than Pip, shaping Pip effort and Rylos shadow-compensation.	Restores early hierarchy and makes later Kyreal scenes land harder.
v10_scene_002	wowas_clean_scene_index_v2.tsv	B02-B03	Diamond explanation cluster	modify	Clarify Diamond power law: Mack + Pip + Rylos charge the Diamond; trapped people receive power and animalization inside it; Pip/Rylos regain human expression through portal transition.	Corrects core cosmology and species-state logic.
v10_scene_003	wowas_clean_scene_index_v2.tsv	B08-B09	Rockies / abduction corridor	insert	Seed Vaedric / origin-truth influence in Rylos path without exposing full intent toward Pip.	Supports later recurring-villain reveal.
v10_scene_004	wowas_clean_scene_index_v2.tsv	B10-B11	Onondaga / dragon-shell sequence	modify	Mark the gemstone dragon-egg event as Pip-only and initially unexplained to everyone present.	Prevents accidental spread of the transformation law.
v10_scene_005	wowas_clean_scene_index_v2.tsv	B10-B11	Rolzen rearrangement and Drawn search	insert	After the group learns the Drawn could reverse Rolzen's rearrangement, have Kyreal lead the search, freeze, collapse into sliced remains when Pip touches him, and trigger Pip's first quantum kill-before-tears scene.	Major emotional and unlock corridor.
v10_scene_006	wowas_clean_scene_index_v2.tsv	B11-B12	Oregon coast / ocean claim	modify	Ensure glide is unavailable in the Silence but regains meaningful use in Lucent/Ursula surface-state after the rewind.	Traversal law correction.
v10_scene_007	wowas_clean_scene_index_v2.tsv	B13-B14	Afterwake / OKC return	insert	After Garden of Glass, Vaedric seeks Rylos and encounters Pip + Kyreal; Pip ignores him until Vaedric demands to know where Rylos is; Pip hurls the ice-mallet strike; Kyreal calls him chicken; Vaedric retorts bitch.	Locks Vaedric/Pip/Kyreal confrontation beat.
v10_scene_008	wowas_clean_scene_index_v2.tsv	B14-B17	Silence / Lucent / Ursula	modify	Relocate the relevant join point so the whole party is present after Silence/Lucent/Ursula instead of after Neith-escape.	Improves party geometry and reveal timing.
v10_scene_009	wowas_clean_scene_index_v2.tsv	B14-B17	Ursula core reveal	insert	Reveal Ursula as amethyst geode around the ice core, Mack as hot ice core, Pip as their child, and the amethyst crystal / creation-song / seed logic.	Turns symbolic geology into explicit family cosmology.
v10_scene_010	wowas_clean_scene_index_v2.tsv	B17-B18	Lucent stone break / rewind	insert	Implement the near-rewind: items/scars reverse, Kyreal remains dead, Rolzen trust/Flux consequence persists, Silence people can drop the ground if trust failed.	Critical branch law.
v10_scene_011	wowas_clean_scene_index_v2.tsv	B18-B24	Rylos shadow escalation	insert	Add precursor beats where Rylos questions his moving shadow; shadow gains autonomy; use threshold mechanics in the Cockatrice-ripper mortal-wound scene.	Builds the later death logic.
v10_scene_012	wowas_clean_scene_index_v2.tsv	B24	Rylos mortal-wound sequence	insert	Pip pulls Rylos back from wrongful shadow-threshold draw; Cockatrice-ripper runs him through in that instant.	Pays off Rylos/Cockatrice line personally.
v10_scene_013	wowas_clean_scene_index_v2.tsv	B25	Blood Cello judgment corridor	insert	Implement branching judgment matrix: no-song world-stasis, love song with support/world-remake, lonely love blackout, mockery corruption, old-man marionette-cello ending, undead punishment, false-victory total fatality.	Turns endgame into moral-resonance audit.
v10_scene_014	wowas_clean_scene_index_v2.tsv	B25	Angel aftermath transfiguration	insert	After angel-diamond compression, let the amethyst judge alignment, drain the old staff to inert twig, and return weapon form based on alignment.	Integrates story, game, and alignment law.
EOF_SCENE

cat > "$OUT/wowas_character_timeline_lattice_patch_v10.tsv" <<'EOF_TL'
character_id	character_name	patch_scope	new_or_corrected_fields
wowas::kyreal_bestfriend_wolf	Kyreal	elevate	species=wolf (bipedal); role=best friend / school-era benchmark / swordsman / stone-controller; source_stack=Toph|Cedric Diggory|Sturm; early_status=strongest_in_class_except_Pip; later_scene_function=Rolzen-Drawn search leader; death_law=collapses into sliced remains; posthumous_effect=triggers Pip quantum unlock kill.
wowas::pip_indalwin_willowjayce	Pip	correct	dragon_shell_event=Pip_only_unexplained; school_power=already_stronger_than_Kyreal_but_unaware; diamond_law=core_battery_with_Mack_and_Rylos; reliquary_law=medicine_pouch_few_black_bag_many; blood_cello_role=endgame_judgment_anchor.
wowas::rylos_vayne_johnson	Rylos	correct	origin_cost=claimed_as_sacrificial_cost_from_first_day; compensation=quiet_as_shadow; shadow_arc=autonomous_and_threshold_active; mortal_wound=dealt_by_Cockatrice_ripper; survivability=requires Pip rescue attempt but still mortally wounded.
wowas::rolzen_warrior_anchor	Rolzen	correct	recovery_law=human_in_demeanor_returns_quickly_after_sense_restoration; flirt_timing=flirts_from_day_one_then_half-hearted_after_five_years_then_Pip_accepts; combat_style=gravity_juggling force-boom crystals; transformation_fix_consequence=if_unsaved_returns_normal_but_thinks_less_of_you; flux_teacher=yes.
wowas::xethrolund_papi	Xethrolund	clarify	quantum_teacher=yes; quantum_line=researched_before_Pip_first_kill; blood_cello_survival_requires=Quantum_plus_Flux.
wowas::vaedric_maldor_obstineth	Vaedric Maldor Obstineth	new	public_title=the Cockatrice; anthropomorphic_form=rooster-man; early_function=older manipulator of Rylos; altar_function=opens void by self-blood cost; return_function=single ripper return; final_function=deals mortal blow to Rylos.
wowas::ursula_cosmic_mother	Ursula	clarify	true_form=amethyst geode around planetary ice core; reveal_window=core_of_her_realm; gift_to_pip=amethyst crystal capable of ending Mack's failing life-state; creation_song_request=hear Mack's creation song one last time.
wowas::mack_willow_father	Mack	clarify	core_state=hot ice core; diamond_intervention=dives in and holds the Diamond back; creation_song_seed_logic=linked to Ursula death corridor.
EOF_TL

cat > "$OUT/wowas_orbit_patch_v10.tsv" <<'EOF_ORBIT'
orbit_code	orbit_scope	change
v10_orbit_001	Pip-Kyreal-Rylos school orbit	Add early hierarchy: Kyreal is the visible benchmark; Pip hides greater depth; Rylos adapts through stealth and silence.
v10_orbit_002	Pip-Rolzen orbit	Lock love-under-ruin: Pip loves Rolzen while still grotesque; Rolzen's humanity returns in demeanor before body; five-year delay before Pip's first yes.
v10_orbit_003	Rylos-Vaedric orbit	Upgrade to coercive trauma-bond / origin-lure / altar-betrayal / ripper-return death-vector.
v10_orbit_004	Pip-Xeth-Rolzen-Rylos core	Preserve quad-phase trajectory: Rolzen hidden-sonomancy witness, Rylos rivalry-softening routes, Quantum+Flux as survivability pair.
v10_orbit_005	Ursula-Mack-Pip cosmology orbit	Make geology, parentage, music, and creation-law one unified reveal system.
EOF_ORBIT

cat > "$OUT/wowas_endgame_judgment_matrix_v10.tsv" <<'EOF_END'
ending_code	required_state	forbidden_state	outcome	hint_logic
blood_cello_good_remake	play=true; song=love; support=full; age=young_enough; quantum=yes; flux=yes	none	Neith remakes the world.	The world answers because Pip stayed true, supported, and structurally whole.
blood_cello_no_song	play=false	n/a	The world continues as it is.	Nihilist refusal. No new world.
blood_cello_lonely_love	play=true; song=love; support=alone; age=young_enough	support=full	Neith cries, asks where everyone is, and everything goes black.	Love without community is insufficient.
blood_cello_false_victory	play=true; battle_state=won; (quantum=no OR flux=no)	both powers present	Battle can look won, but Neith still kills Pip and no one survives.	Missing one foundational power.
blood_cello_undead_failure	play=true; undead_magic_used=yes	undead_magic_used=no	Those meant to be resurrected return undead and tear Pip apart.	Violation of life/death law.
blood_cello_old_man_mockery	play=true; song=mockery; age=old; support=none	age=young_enough	Neith turns Pip into a string marionette and plays him like a cello.	He spent his life-force wrong and turned music into cruelty.
blood_cello_mockery_world	play=true; song=mockery; age=young_or_mid; support=none	song=love	Neith plays with him and creates a disgusting world.	Sadist route.
blood_cello_stalled_collapse	play=true; song=love; alignment_off_path=yes	alignment_true=yes	Neith walks away; Pip dies; the new world collapses for lack of right energy.	He did not stay true enough to the path.
EOF_END

cat > "$OUT/wowas_magic_system_patch_v10.md" <<'EOF_MAGIC'
# WoWaS Magic / Traversal / Weapon Patch — Update 10

## Pip
- Whip traversal pre-exists Update 10.
- Glide is the new movement expansion and belongs to the transfigured post-angel weapon state.
- In the Silence there is **no lift**; glide attempts fail because the air itself is wrong.
- In Lucent/Ursula surface-state, lift returns and glide becomes usable again.
- In the hardest traversal sequences Pip cannot lay the return path for the party until he has a stable anchor point.
- Alignment transfiguration changes not just weapon class but terrain-solving style.

## Rolzen
- Rolzen is force-boom, not fire.
- His combat system is a spell-through-motion lattice built from gravity juggling crystal spheres.
- Default pattern: six 4-inch spheres + one larger top sphere.
- He can strike the lined-up kinetic string, launch one, boom on impact, rebound, recatch, and re-enter the juggle.
- He can redirect multiple spheres through flips, kicks, tumbles, and changing vectors.
- He can time ground booms to suspend himself over opponents briefly.
- Detonation is highest-cost release.
- A detonated sphere collapses to a bead, which he catches and re-expands later.
- At the end of combat he swallows the beads.

## Quantum / Flux
- Quantum = threshold precision, state-breach, impossible action under wrong conditions.
- Flux = adaptive force-continuity, motion-through-change, living pressure transfer.
- Both are required for survivable Neith victory.

## Haptics / sound law (design lock)
- Major power states, wrong magic use, traversal state changes, and Blood Cello judgment outcomes require distinct haptic and sonic identities.
EOF_MAGIC

cat > "$OUT/BRAXON_ready_manifest_v10.md" <<'EOF_MAN'
# BRAXON Ready Manifest — Update 10

## Ingest order
1. Load `wowas_final_edition_BRAXON_ready_v8` base files.
2. Load `wowas_v8_polish` from the v9 tarball as polish / disambiguation layer.
3. Load the Update 10 files in this bundle as newest authority.

## Update 10 authority notes
- Wherever Update 10 conflicts with v8 or v9, Update 10 wins.
- Update 10 is authoritative on: Kyreal weight, Diamond power law, Pip-only dragon shell, Rylos shadow / Cockatrice threshold, Ursula-Mack core reveal, Lucent rewind law, Quantum+Flux survivability, Blood Cello judgment matrix, angel transfiguration, and Rolzen crystal-juggling combat style.

## BRAXON ingest tags
- wowas::canon_update_10
- wowas::blood_cello_judgment
- wowas::quantum_flux_required
- wowas::rolzen_force_boom_crystal_lattice
- wowas::kyreal_benchmark_and_collapse
EOF_MAN

cat > "$OUT/CURRENT_APPLY_ORDER_v10.md" <<'EOF_ORDER'
# Current Apply Order — v8 / v9 / v10

1. Base bundle: `wowas_final_edition_BRAXON_ready_v8.zip`
2. Polish bundle: `wowas_v9_polish_patch.tar.gz`
3. Update 10 bundle: this directory / tarball

## Runtime rule
Braxon should treat later-numbered bundles as newer authority. Do not overwrite the base files destructively unless you explicitly decide to collapse the layers into a merged monolith later.
EOF_ORDER

cat > "$OUT/install_wowas_8_9_10.sh" <<'EOF_INSTALL'
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

DL="$HOME/storage/shared/Download"
V8_ZIP_DEFAULT="$DL/wowas_final_edition_BRAXON_ready_v8.zip"
V9_TAR_DEFAULT="$DL/wowas_v9_polish_patch.tar.gz"
V10_TAR_DEFAULT="$DL/wowas_update10_bundle.tar.gz"
V10_DIR_DEFAULT="$DL/wowas_update10_bundle"
OUT="$DL/wowas_final_edition_BRAXON_ready_v10"

V8_ZIP="${1:-$V8_ZIP_DEFAULT}"
V9_TAR="${2:-$V9_TAR_DEFAULT}"
V10_SRC="${3:-}"

mkdir -p "$DL"
rm -rf "$OUT"
mkdir -p "$OUT" "$OUT/patches/v9" "$OUT/patches/v10"

if [ ! -f "$V8_ZIP" ]; then
  echo "Missing v8 zip: $V8_ZIP" >&2
  exit 1
fi

# Extract base v8 bundle into the normalized v10 folder root.
unzip -q "$V8_ZIP" -d "$OUT/.stage_v8"
if [ -d "$OUT/.stage_v8/wowas_final_edition_BRAXON_ready_v8" ]; then
  cp -a "$OUT/.stage_v8/wowas_final_edition_BRAXON_ready_v8/." "$OUT/"
else
  echo "Unexpected v8 archive layout." >&2
  exit 1
fi
rm -rf "$OUT/.stage_v8"

# Stage v9 polish layer if present.
if [ -f "$V9_TAR" ]; then
  tar -xzf "$V9_TAR" -C "$OUT/patches/v9"
else
  echo "Warning: v9 patch tar not found at $V9_TAR" >&2
fi

# Determine v10 source.
if [ -z "$V10_SRC" ]; then
  if [ -d "$V10_DIR_DEFAULT" ]; then
    V10_SRC="$V10_DIR_DEFAULT"
  elif [ -f "$V10_TAR_DEFAULT" ]; then
    V10_SRC="$V10_TAR_DEFAULT"
  else
    echo "Missing Update 10 source. Place wowas_update10_bundle.tar.gz or wowas_update10_bundle in $DL, or pass a third argument." >&2
    exit 1
  fi
fi

if [ -d "$V10_SRC" ]; then
  cp -a "$V10_SRC/." "$OUT/patches/v10/"
elif [ -f "$V10_SRC" ]; then
  tar -xzf "$V10_SRC" -C "$OUT/patches/v10"
else
  echo "Invalid Update 10 source: $V10_SRC" >&2
  exit 1
fi

# Copy the key update10 authority docs to root for easier ingest discovery.
for f in   wowas_final_canon_control_bundle_v10_addendum.md   wowas_scene_patch_v10.tsv   wowas_character_timeline_lattice_patch_v10.tsv   wowas_orbit_patch_v10.tsv   wowas_endgame_judgment_matrix_v10.tsv   wowas_magic_system_patch_v10.md   BRAXON_ready_manifest_v10.md   CURRENT_APPLY_ORDER_v10.md   README_v10.md
  do
  if [ -f "$OUT/patches/v10/$f" ]; then
    cp -f "$OUT/patches/v10/$f" "$OUT/$f"
  fi
done

cat > "$OUT/INSTALL_SUMMARY_v10.txt" <<EOF
install_target=$OUT
base_v8_zip=$V8_ZIP
v9_patch=$V9_TAR
v10_source=$V10_SRC
status=installed
next_step=point_BRAXON_ingest_at_CURRENT_APPLY_ORDER_v10.md
EOF

echo
echo "Installed WoWaS v8 + v9 + v10 at: $OUT"
echo "Key authority file: $OUT/CURRENT_APPLY_ORDER_v10.md"
echo "Install summary: $OUT/INSTALL_SUMMARY_v10.txt"
EOF_INSTALL
chmod +x "$OUT/install_wowas_8_9_10.sh"

echo "Update 10 bundle written to: $OUT"
echo "Next: bash "$OUT/install_wowas_8_9_10.sh""
