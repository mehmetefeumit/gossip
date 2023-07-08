use std::collections::HashSet;

use super::{
    filter_relay, relay_filter_combo, relay_sort_combo, GossipUi,
};
use crate::db::DbRelay;
use crate::globals::GLOBALS;
use crate::ui::widgets;
use crate::{comms::ToOverlordMessage};
use eframe::egui;
use egui::{Context, Ui};
use egui_winit::egui::{Id};
use nostr_types::RelayUrl;

pub(super) fn update(app: &mut GossipUi, _ctx: &Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
    let is_editing = app.relays.edit.is_some();
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        ui.heading("Active Relays");
        ui.add_space(50.0);
        ui.set_enabled(!is_editing);
        widgets::search_filter_field(ui, &mut app.relays.search, 200.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.add_space(20.0);
            relay_filter_combo(app, ui);
            ui.add_space(20.0);
            relay_sort_combo(app, ui);
        });
    });
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    ui.heading("Coverage");

    if GLOBALS.relay_picker.pubkey_counts_iter().count() > 0 {
        for elem in GLOBALS.relay_picker.pubkey_counts_iter() {
            let pk = elem.key();
            let count = elem.value();
            let name = GossipUi::display_name_from_pubkeyhex_lookup(pk);
            ui.label(format!("{}: coverage short by {} relay(s)", name, count));
        }
        ui.add_space(12.0);
        if ui.button("Pick Again").clicked() {
            let _ = GLOBALS.to_overlord.send(ToOverlordMessage::PickRelays);
        }
    } else {
        ui.label("All followed people are fully covered.".to_owned());
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    let connected_relays: HashSet<RelayUrl> = GLOBALS
        .connected_relays
        .iter()
        .map(|r| r.key().clone())
        .collect();

    let mut relays: Vec<DbRelay> = GLOBALS
        .all_relays
        .iter()
        .map(|ri| ri.value().clone())
        .filter(|ri| connected_relays.contains(&ri.url) && filter_relay(&app.relays, ri))
        .collect();

    if !is_editing {
        relays.sort_by(|a, b| super::sort_relay(&app.relays, a, b));
    } else {
        // when editing, use constant sorting by url so the sorting doesn't change on edit
        relays.sort_by(|a, b| a.url.cmp(&b.url));
    }

    let id_source: Id = "RelayActivityMonitorScroll".into();

    super::relay_scroll_list(app, ui, relays, id_source);
}
