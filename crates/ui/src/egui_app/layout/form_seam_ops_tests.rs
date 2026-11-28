//! #1122: the seam helpers that the common operation tabs added keep the
//! behaviour of the raw calls that they replace: a wrapped row and a
//! read-out keep their text, a read-out takes no selection, and a choice
//! button shows its state and takes a click only while it is enabled.
//! Provenance: our own tests over our own wrappers; no vendor material.

#[derive(Default)]
struct OpsProbe {
    on_hits: usize,
    off_hits: usize,
}

#[test]
fn the_operation_tab_seam_helpers_keep_clicks_text_and_state() {
    use egui::accesskit::{Role, Toggled};
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};

    let mut h = Harness::builder().with_size(egui::Vec2::new(480.0, 240.0)).build_ui_state(
        |ui, p: &mut OpsProbe| {
            super::list_row_wrapped(ui, "Boundary settings do not apply to it: the kind cuts holes.");
            super::readout(ui, "Setup 1 offset: X 0.000 Y 0.000 Z 0.000");
            if super::choice_button_enabled(ui, true, true, "Include").clicked() {
                p.on_hits += 1;
            }
            if super::choice_button_enabled(ui, false, false, "Exclude").clicked() {
                p.off_hits += 1;
            }
        },
        OpsProbe::default(),
    );
    h.run();

    for text in ["Boundary settings do not apply to it: the kind cuts holes.", "Setup 1 offset: X 0.000 Y 0.000 Z 0.000"] {
        assert!(h.query_by_label(text).is_some(), "{text} is on the page");
    }
    let toggled = |label: &str| {
        h.root()
            .children_recursive()
            .find(|n| n.accesskit_node().role() == Role::Button && n.accesskit_node().label().as_deref() == Some(label))
            .and_then(|n| n.accesskit_node().toggled())
    };
    assert_eq!(toggled("Include"), Some(Toggled::True), "the selected choice says so");
    assert_eq!(toggled("Exclude"), Some(Toggled::False), "the other choice says so");

    h.get_by_label("Include").click();
    h.get_by_label("Exclude").click();
    h.run();
    assert_eq!(h.state().on_hits, 1, "an enabled choice takes a click");
    assert_eq!(h.state().off_hits, 0, "a greyed-out choice takes no click");
}

/// An optional group's switch has no visible caption: the seam names it
/// "Use {title}", so a screen reader reads more than "check box".
#[test]
fn an_optional_groups_switch_is_named_by_its_group() {
    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};

    let mut h = Harness::builder().with_size(egui::Vec2::new(480.0, 200.0)).build_ui_state(
        |ui, on: &mut bool| {
            super::group(ui, "Retract Z", Some(on), |ui| super::list_row(ui, "Offset 10.0 mm"));
        },
        false,
    );
    h.run();
    let unnamed = h.root().children_recursive().any(|n| {
        let a = n.accesskit_node();
        a.role() == Role::CheckBox && a.label().is_none_or(|l| l.is_empty())
    });
    assert!(!unnamed, "no check box without a name");
    h.get_by_role_and_label(Role::CheckBox, "Use Retract Z").click();
    h.run();
    assert!(*h.state(), "the named switch is the group's switch");
}

#[derive(Default)]
struct MachineProbe {
    text: String,
    slots: u32,
    sized_hits: usize,
    filled_hits: usize,
    off_hits: usize,
}

/// The helpers that the machine pages added draw what `ui.add` drew: a
/// text field and a number field take input, a sized button keeps its
/// size, and a filled button takes a click only while it is enabled.
#[test]
fn the_machine_seam_helpers_keep_input_size_and_clicks() {
    use egui::accesskit::Role;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};

    let mut h = Harness::builder().with_size(egui::Vec2::new(480.0, 240.0)).build_ui_state(
        |ui, p: &mut MachineProbe| {
            let t = super::text_field(ui, egui::TextEdit::singleline(&mut p.text).desired_width(120.0));
            crate::egui_app::fields::named(t, "Machine model");
            let d = super::drag_field(ui, egui::DragValue::new(&mut p.slots).suffix(" slots"));
            crate::egui_app::fields::named(d, "Tool changer slots");
            if super::button_sized(ui, egui::Vec2::new(150.0, 24.0), "Load machine (.mch)").clicked() {
                p.sized_hits += 1;
            }
            if super::button_enabled_fill(ui, true, "New blank", Some(egui::Color32::from_rgb(40, 90, 160))).clicked() {
                p.filled_hits += 1;
            }
            if super::button_enabled_fill(ui, false, "Edit machine", None).clicked() {
                p.off_hits += 1;
            }
        },
        MachineProbe::default(),
    );
    h.run();

    let sized = h.get_by_label("Load machine (.mch)").rect();
    assert!((sized.width() - 150.0).abs() < 1.0, "the sized button keeps its width: {sized:?}");
    h.get_by_role_and_label(Role::TextInput, "Machine model").click();
    h.run();
    h.get_by_role_and_label(Role::TextInput, "Machine model").type_text("VMC-1");
    h.run();
    assert_eq!(h.state().text, "VMC-1", "the text field takes input");
    assert!(
        h.root().children_recursive().any(|n| n.accesskit_node().role() == Role::SpinButton
            && n.accesskit_node().label().as_deref() == Some("Tool changer slots")),
        "the number field is drawn and named"
    );
    h.get_by_label("Load machine (.mch)").click();
    h.get_by_label("New blank").click();
    h.get_by_label("Edit machine").click();
    h.run();
    assert_eq!(h.state().sized_hits, 1);
    assert_eq!(h.state().filled_hits, 1, "an enabled filled button takes a click");
    assert_eq!(h.state().off_hits, 0, "a greyed-out button takes no click");
}
