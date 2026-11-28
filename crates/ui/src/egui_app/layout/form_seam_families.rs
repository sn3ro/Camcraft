//! #1122: the page families that the form-seam guard holds at zero raw
//! egui control calls (`form_seam_tests.rs`), each with its source files.
//! Split from the guard for the 400-line law.
//! Provenance: our own source-level guard over `crates/ui/src/egui_app`.

pub(super) struct Family<'a> {
    pub(super) name: &'a str,
    pub(super) max_raw: usize,
    pub(super) sources: &'a [(&'a str, &'a str)],
}

pub(super) const FAMILIES: &[Family<'_>] = &[
    Family {
        name: "holes",
        max_raw: 0,
        sources: &[
            ("holes_page.rs", include_str!("../holes_page.rs")),
            ("thread_page.rs", include_str!("../thread_page.rs")),
        ],
    },
    Family {
        name: "turning",
        max_raw: 0,
        sources: &[
            ("turning_page.rs", include_str!("../turning_page.rs")),
            ("turning_more_page.rs", include_str!("../turning_more_page.rs")),
            ("turning_start_exit_rows.rs", include_str!("../turning_start_exit_rows.rs")),
            ("thread_infeed_page.rs", include_str!("../thread_infeed_page.rs")),
        ],
    },
    Family {
        name: "multi-axis",
        max_raw: 0,
        sources: &[
            ("multiaxis_page.rs", include_str!("../multiaxis_page.rs")),
            ("rotary_page.rs", include_str!("../rotary_page.rs")),
            ("multiaxis_deburr_page.rs", include_str!("../multiaxis_deburr_page.rs")),
        ],
    },
    Family {
        name: "2D",
        max_raw: 0,
        sources: &[
            ("mill2d_page.rs", include_str!("../mill2d_page.rs")),
            ("mill2d_more_page.rs", include_str!("../mill2d_more_page.rs")),
            ("trochoid_page.rs", include_str!("../trochoid_page.rs")),
            ("vcarve_page.rs", include_str!("../vcarve_page.rs")),
            ("probe_page.rs", include_str!("../probe_page.rs")),
            ("plasma_page.rs", include_str!("../plasma_page.rs")),
            ("waterjet_page.rs", include_str!("../waterjet_page.rs")),
            ("wire_page.rs", include_str!("../wire_page.rs")),
            ("sinker_page.rs", include_str!("../sinker_page.rs")),
            ("sinker_pick_rows.rs", include_str!("../sinker_pick_rows.rs")),
            ("deburr_page.rs", include_str!("../deburr_page.rs")),
            ("pockets_page.rs", include_str!("../pockets_page.rs")),
            ("approach_page.rs", include_str!("../approach_page.rs")),
            ("predrilled_page.rs", include_str!("../predrilled_page.rs")),
            ("rest_pocket_page.rs", include_str!("../rest_pocket_page.rs")),
            ("start_exit_rows.rs", include_str!("../start_exit_rows.rs")),
            ("pass_start_row.rs", include_str!("../pass_start_row.rs")),
            ("load_field.rs", include_str!("../load_field.rs")),
            ("lead_page.rs", include_str!("../lead_page.rs")),
            ("kind_editors.rs", include_str!("../kind_editors.rs")),
        ],
    },
    Family {
        name: "3D",
        max_raw: 0,
        sources: &[
            ("mill3d_page.rs", include_str!("../mill3d_page.rs")),
            ("undercut_page.rs", include_str!("../undercut_page.rs")),
        ],
    },
    // The tabs and rows that every operation page shares: the page frame,
    // the geometry picks, the boundary, the heights and the feeds.
    Family {
        name: "operation tabs",
        max_raw: 0,
        sources: &[
            ("op_page.rs", include_str!("../op_page.rs")),
            ("op_page/header.rs", include_str!("../op_page/header.rs")),
            ("geo_page.rs", include_str!("../geo_page.rs")),
            ("boundary.rs", include_str!("../boundary.rs")),
            ("heights_page.rs", include_str!("../heights_page.rs")),
            ("feeds_page.rs", include_str!("../feeds_page.rs")),
        ],
    },
    // The machine pages: the setup's machine page, the machine library
    // pane and the machine file editors.
    Family {
        name: "machine",
        max_raw: 0,
        sources: &[
            ("machine_page.rs", include_str!("../machine_page.rs")),
            ("machines_pane.rs", include_str!("../machines_pane.rs")),
            ("machine_grid.rs", include_str!("../machine_grid.rs")),
            ("machine_editor.rs", include_str!("../machine_editor.rs")),
            ("machine_file_editor.rs", include_str!("../machine_file_editor.rs")),
            ("machine_axis_chain.rs", include_str!("../machine_axis_chain.rs")),
            ("machine_edm.rs", include_str!("../machine_edm.rs")),
            ("machine_edm_editor.rs", include_str!("../machine_edm_editor.rs")),
        ],
    },
];
