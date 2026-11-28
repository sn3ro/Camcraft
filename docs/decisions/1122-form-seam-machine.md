## #1122 - The machine pages draw through the form seam

**What was decided.**
- The "machine" family is the setup's machine page and the machine
  library and file editors: `machine_page.rs`, `machines_pane.rs`,
  `machine_grid.rs`, `machine_editor.rs`, `machine_file_editor.rs`,
  `machine_axis_chain.rs`, `machine_edm.rs` and `machine_edm_editor.rs`.
  The guard holds it at 0 raw calls (145 on `main`).
- Four new seam helpers, each a thin wrapper that draws what the raw call
  drew:
  - `layout::text_field`: a configured `TextEdit` outside a property row;
  - `layout::drag_field`: a configured `DragValue` outside a property row;
  - `layout::button_sized`: a button at a fixed size;
  - `layout::button_enabled_fill`: `button_enabled` with an optional fill
    (the accent of "New blank" once a kind is picked).
- `text_field` and `drag_field` do not join the pane's Tab / Enter ring
  and take no name from a row. The machine editors lay their own grids and
  name each field (`fields::named`, `labelled_by`), so the slice keeps that
  behaviour exactly. A later change can move them to `drag_row` and the
  ring on purpose.
- The family table of the guard moves to its own test file
  (`form_seam_families.rs`), because `form_seam_tests.rs` went past 400
  lines with the new family.

**Why.** One seam for the controls lets the design system of #4245 restyle
the machine pages with the operation pages, in one place.

**Considered and dropped.**
- Put the machine fields on the Tab ring in this slice. It changes the
  focus order, so it is not behaviour-neutral.
- One generic `add(ui, widget)` helper. It hides which control a page
  draws, so a look cannot restyle by control.

**Cost to change later.** Small: each helper is a few lines, and each call
site is one line.
