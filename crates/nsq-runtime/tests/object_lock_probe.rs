use nsq_core::NsqSurfaceValue;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct RuntimeImage {
    assigned_core: NsqSurfaceValue,
    expected_mode: &'static str,
    object_name: &'static str,
    runtime_epoch: NsqSurfaceValue,
}

#[derive(Debug, Clone)]
struct Instruction {
    op: &'static str,
    target_object: &'static str,
    peers_to_wake: Vec<&'static str>,
    payload: &'static str,
    revision: NsqSurfaceValue,
}

#[derive(Debug, Default)]
struct ObjectLockState {
    held_by: Option<&'static str>,
    active_bits: Vec<&'static str>,
    journal: Vec<String>,
    completions: Vec<String>,
    peer_messages: Vec<String>,
}

#[derive(Debug)]
struct RamImage {
    revision: NsqSurfaceValue,
    queues: HashMap<&'static str, VecDeque<Instruction>>,
    stop: bool,
}

#[derive(Debug, Default)]
struct ProbeState {
    runtime: RuntimeImage,
    ram: RamImage,
    lock: ObjectLockState,
}

impl Default for RuntimeImage {
    fn default() -> Self {
        Self {
            assigned_core: NsqSurfaceValue::zero(),
            expected_mode: "object_lock_probe",
            object_name: "alpha",
            runtime_epoch: NsqSurfaceValue::one(),
        }
    }
}

fn probe_value(value: &str) -> NsqSurfaceValue {
    NsqSurfaceValue::new(value).unwrap()
}

impl Default for RamImage {
    fn default() -> Self {
        Self {
            revision: NsqSurfaceValue::zero(),
            queues: HashMap::new(),
            stop: false,
        }
    }
}

fn spawn_bit(
    name: &'static str,
    shared: Arc<(Mutex<ProbeState>, Condvar)>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let instruction = {
            let (mx, cv) = &*shared;
            let mut st = mx.lock().unwrap();

            while !st.ram.stop
                && st
                    .ram
                    .queues
                    .get(name)
                    .map(|q| q.is_empty())
                    .unwrap_or(true)
            {
                let (guard, _) = cv.wait_timeout(st, Duration::from_millis(25)).unwrap();
                st = guard;
            }

            if st.ram.stop {
                st.lock.journal.push(format!("{name}:stop"));
                return;
            }

            let instr = st
                .ram
                .queues
                .get_mut(name)
                .and_then(|q| q.pop_front())
                .expect("queued instruction must exist");

            st.lock.journal.push(format!(
                "{name}:picked:{}:rev{}",
                instr.op,
                instr.revision.as_text()
            ));
            instr
        };

        {
            let (mx, cv) = &*shared;
            let mut st = mx.lock().unwrap();

            if st.lock.held_by.is_none() {
                st.lock.held_by = Some(name);

                let object_name = st.runtime.object_name;
                let expected_mode = st.runtime.expected_mode;
                let assigned_core = st.runtime.assigned_core.as_text().to_string();
                let runtime_epoch = st.runtime.runtime_epoch.as_text().to_string();

                st.lock.journal.push(format!(
                    "{name}:acquired_lock:{}:{}:core{}:epoch{}",
                    object_name, expected_mode, assigned_core, runtime_epoch
                ));
            } else {
                st.lock
                    .journal
                    .push(format!("{name}:entered_under_existing_lock"));
            }

            if !st.lock.active_bits.contains(&name) {
                st.lock.active_bits.push(name);
            }

            if !instruction.peers_to_wake.is_empty() {
                for peer in &instruction.peers_to_wake {
                    let peer_instr = Instruction {
                        op: "assist",
                        target_object: instruction.target_object,
                        peers_to_wake: vec![],
                        payload: instruction.payload,
                        revision: st.ram.revision.clone(),
                    };
                    st.ram.queues.entry(peer).or_default().push_back(peer_instr);
                    st.lock.peer_messages.push(format!("{name}->{peer}"));
                    st.lock.journal.push(format!("{name}:woke_peer:{peer}"));
                }
                cv.notify_all();
            }
        }

        thread::sleep(Duration::from_millis(15));

        {
            let (mx, _cv) = &*shared;
            let mut st = mx.lock().unwrap();

            st.lock.completions.push(format!(
                "{name}:done:{}:{}:rev{}",
                instruction.op,
                instruction.payload,
                instruction.revision.as_text()
            ));
            st.lock.journal.push(format!(
                "{name}:complete:{}:rev{}",
                instruction.op,
                instruction.revision.as_text()
            ));

            if st.lock.held_by == Some(name) {
                st.lock
                    .journal
                    .push(format!("{name}:kept_object_lock_live_for_circulation"));
            }
        }
    })
}

fn queue_for(
    st: &mut ProbeState,
    who: &'static str,
    op: &'static str,
    peers: Vec<&'static str>,
    payload: &'static str,
) {
    let instr = Instruction {
        op,
        target_object: st.runtime.object_name,
        peers_to_wake: peers,
        payload,
        revision: st.ram.revision.clone(),
    };
    st.ram.queues.entry(who).or_default().push_back(instr);
}

#[test]
fn object_lock_one_called_peer_set_communicates_and_circulates() {
    let shared = Arc::new((Mutex::new(ProbeState::default()), Condvar::new()));

    let bit_a = spawn_bit("bit_a", shared.clone());
    let bit_b = spawn_bit("bit_b", shared.clone());
    let bit_c = spawn_bit("bit_c", shared.clone());

    let started = Instant::now();

    {
        let (mx, cv) = &*shared;
        let mut st = mx.lock().unwrap();

        st.runtime.assigned_core = probe_value("3");
        st.runtime.expected_mode = "object_lock_probe";
        st.runtime.object_name = "alpha";
        st.runtime.runtime_epoch = probe_value("1");
        st.ram.revision = probe_value("1");

        queue_for(
            &mut st,
            "bit_a",
            "primary_call",
            vec!["bit_b", "bit_c"],
            "fanout_v1",
        );
        cv.notify_all();
    }

    thread::sleep(Duration::from_millis(120));

    {
        let (mx, cv) = &*shared;
        let mut st = mx.lock().unwrap();

        st.ram.revision = probe_value("2");
        st.runtime.runtime_epoch = probe_value("2");

        queue_for(
            &mut st,
            "bit_a",
            "followup_call",
            vec!["bit_b"],
            "fanout_v2",
        );
        cv.notify_all();
    }

    thread::sleep(Duration::from_millis(140));

    let snapshot = {
        let (mx, cv) = &*shared;
        let mut st = mx.lock().unwrap();
        st.ram.stop = true;
        cv.notify_all();

        (
            st.lock.held_by,
            st.lock.active_bits.clone(),
            st.lock.journal.clone(),
            st.lock.completions.clone(),
            st.lock.peer_messages.clone(),
            st.ram.revision.clone(),
            st.runtime.runtime_epoch.clone(),
        )
    };

    bit_a.join().unwrap();
    bit_b.join().unwrap();
    bit_c.join().unwrap();

    let (
        held_by,
        active_bits,
        journal,
        completions,
        peer_messages,
        final_ram_rev,
        final_runtime_epoch,
    ) = snapshot;

    assert_eq!(
        held_by,
        Some("bit_a"),
        "object lock should be anchored by the first caller for this probe"
    );

    assert!(active_bits.contains(&"bit_a"));
    assert!(
        active_bits.contains(&"bit_b"),
        "peer bit_b should have become active without direct external call"
    );
    assert!(
        active_bits.contains(&"bit_c"),
        "peer bit_c should have become active without direct external call"
    );

    assert!(
        peer_messages.iter().any(|m| m == "bit_a->bit_b"),
        "bit_a should have signaled bit_b"
    );
    assert!(
        peer_messages.iter().any(|m| m == "bit_a->bit_c"),
        "bit_a should have signaled bit_c on first wave"
    );

    assert!(
        completions
            .iter()
            .any(|c| c.starts_with("bit_b:done:assist:fanout_v1")),
        "bit_b should have completed a peer-woken assist task"
    );
    assert!(
        completions
            .iter()
            .any(|c| c.starts_with("bit_c:done:assist:fanout_v1")),
        "bit_c should have completed a peer-woken assist task"
    );
    assert!(
        completions
            .iter()
            .any(|c| c.starts_with("bit_a:done:followup_call:fanout_v2")),
        "bit_a should have stayed in circulation and handled the second instruction"
    );
    assert!(
        completions
            .iter()
            .any(|c| c.starts_with("bit_b:done:assist:fanout_v2")),
        "bit_b should have picked up second-wave work from the updated RAM image"
    );

    assert_eq!(
        final_ram_rev.as_text(),
        "2",
        "RAM image revision should have advanced"
    );
    assert_eq!(
        final_runtime_epoch.as_text(),
        "2",
        "runtime image epoch should have advanced"
    );

    assert!(
        journal.iter().any(|j| j.contains("acquired_lock")),
        "lock acquisition should be journaled"
    );
    assert!(
        journal
            .iter()
            .any(|j| j.contains("kept_object_lock_live_for_circulation")),
        "circulation behavior should be journaled"
    );

    eprintln!("probe_runtime_ms={}", started.elapsed().as_millis());
    eprintln!("active_bits={:?}", active_bits);
    eprintln!("peer_messages={:?}", peer_messages);
    eprintln!("completions={:#?}", completions);
    eprintln!("journal={:#?}", journal);
}
