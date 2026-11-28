//! #4511: a setup-change stop (M0 / M1) after a tilted section lifts Z
//! to its reference point. A tilted section leaves G43.4 armed, and a
//! reference return under TCPM is one many controls alarm on. The post
//! cancels TCPM (G49) first, as its Go home and its tool-change retract
//! (#4341) do, and lifts Z once.
//! Provenance: the posts' own Go-home order; our synthetic streams.

use super::{run, tilted_path, upright_path};
use camcraft_post::post::{self, AfterOperation, Dump, cnc};
use serde_json::json;

fn words(l: &str) -> Vec<&str> {
    l.split_whitespace().filter(|w| !w.starts_with('N')).collect()
}

/// The tilted section asks `after` with a setup change to "Setup 2";
/// the upright section runs on the same tool, so no tool-change
/// retract hides the stop's own lift.
fn records(after: AfterOperation) -> Vec<cnc::Record> {
    let dump = |path, op, after| {
        let d = Dump {
            op_name: op,
            tool_number: 1,
            tool_name: "BN-0600",
            tool_diameter: 6.0,
            rpm: 8000.0,
            clearance_z: 20.0,
            retract_z: 10.0,
            top_z: 0.0,
            wcs: 1,
            origin: [0.0; 3],
            stock: [-50.0, 50.0, -50.0, 50.0],
            stock_top_z: 0.0,
            after,
            ..Default::default()
        };
        post::dump(path, &d)
    };
    let (lean, flat) = (tilted_path(), upright_path());
    let mut r = dump(&lean, "Lean1", after);
    // The app's order (posting_setup_change::ask): just ahead of the ask.
    let at = r
        .iter()
        .position(|x| matches!(x, cnc::Record::Param { name, .. } if name == "AfterOperation"))
        .expect("the tilted section asks its stop");
    r.insert(at, cnc::Record::Param { name: "SetupChange".into(), value: json!("Setup 2") });
    r.extend(dump(&flat, "Flat2", AfterOperation::default()));
    r
}

#[test]
fn fanuc_and_doosan_cancel_tcpm_before_the_setup_change_lift() {
    for (after, stop) in [(AfterOperation::ProgramStop, "M0"), (AfterOperation::OptionalStop, "M1")] {
        for stem in ["fanuc", "doosan-dnm"] {
            let out = run(stem, &records(after));
            let prog = &out.program;
            assert!(out.warnings.is_empty(), "{stem} {stop}: {:?}\n{prog}", out.warnings);
            let lines: Vec<Vec<&str>> = prog.lines().map(words).collect();
            let arm = lines.iter().position(|w| w.contains(&"G43.4")).unwrap_or_else(|| panic!("{stem}: fixture arms TCPM:\n{prog}"));
            let halt = lines.iter().position(|w| w.contains(&stop)).unwrap_or_else(|| panic!("{stem}: the {stop} stop:\n{prog}"));
            let spin = lines[..halt].iter().rposition(|w| w.contains(&"M5")).unwrap_or_else(|| panic!("{stem}: spindle stop:\n{prog}"));
            assert!(arm < spin, "{stem}: TCPM armed in the tilted section:\n{prog}");
            assert!(
                prog.contains("SETUP CHANGE - NEXT: Setup 2"),
                "{stem} {stop}: the stop names the next setup:\n{prog}"
            );
            let gap = &lines[spin..halt];
            let lifts: Vec<usize> = gap
                .iter()
                .enumerate()
                .filter(|(_, w)| w.contains(&"G28") && w.contains(&"Z0"))
                .map(|(i, _)| i)
                .collect();
            assert_eq!(lifts.len(), 1, "{stem} {stop}: one Z lift before the stop: {gap:?}");
            let cancel = gap
                .iter()
                .position(|w| w.contains(&"G49"))
                .unwrap_or_else(|| panic!("{stem} {stop}: G49 before the setup-change lift: {gap:?}\n{prog}"));
            assert!(cancel < lifts[0], "{stem} {stop}: TCPM off before the reference return: {gap:?}");
            // The next preamble finds Z up and TCPM off: no second lift.
            let flat = lines.iter().position(|w| w.iter().any(|x| x.contains("Flat2"))).unwrap_or(halt);
            let again = lines[halt..flat.max(halt)].iter().filter(|w| w.contains(&"G28") && w.contains(&"Z0")).count();
            assert_eq!(again, 0, "{stem} {stop}: no second Z lift after the stop:\n{prog}");
        }
    }
}
