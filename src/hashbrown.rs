use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
use std::{fmt::Display, str::FromStr};

use crate::map::HashMapProbe;
use crate::{
    EguiProbe, Style,
    collections::{DeleteMe, EguiProbeFrozen},
    option::option_probe_with,
};

impl<K, V, S> EguiProbe for HashMap<K, V, S>
where
    K: Display + FromStr + Eq + std::hash::Hash,
    V: EguiProbe + Default,
    S: std::hash::BuildHasher,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        let mut changed = false;

        let mut r = ui
            .horizontal(|ui| {
                let mut probe = HashMapProbe::load(ui.ctx(), ui.make_persistent_id("HashMapProbe"));

                let mut reduce_text_width = 0.0;

                let r = ui.weak(format!("[{}]", self.len()));
                reduce_text_width += r.rect.width() + ui.spacing().item_spacing.x;

                let r = ui.small_button(style.add_button_text());
                if r.clicked() {
                    if let Ok(key) = K::from_str(&probe.state.new_key) {
                        match self.entry(key) {
                            Entry::Occupied(_) => {
                                probe.key_error();
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(V::default());
                                probe.key_accepted();
                            }
                        }
                    } else {
                        probe.key_error();
                    }

                    changed = true;
                }

                reduce_text_width += r.rect.width() + ui.spacing().item_spacing.x;

                probe.new_key_edit(ui, reduce_text_width);
                probe.store(ui.ctx());
            })
            .response;

        if changed {
            r.mark_changed();
        }

        r
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        self.retain(|key, value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&key.to_string(), ui, &mut item);
            !item.delete
        });
    }
}

impl<K, V, S> EguiProbe for EguiProbeFrozen<'_, HashMap<K, V, S>>
where
    K: Display + Eq + std::hash::Hash,
    V: EguiProbe,
    S: std::hash::BuildHasher,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &Style) -> egui::Response {
        ui.weak(format!("[{}]", self.value.len()))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        for (key, value) in self.value.iter_mut() {
            f(&key.to_string(), ui, value);
        }
    }
}

impl<K, V, S> EguiProbe for EguiProbeFrozen<'_, Option<HashMap<K, V, S>>>
where
    K: Display + Eq + std::hash::Hash,
    V: EguiProbe,
    S: std::hash::BuildHasher + Default,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        option_probe_with(
            self.value,
            ui,
            style,
            || HashMap::with_hasher(S::default()),
            |value, ui, _style| ui.weak(format!("[{}]", value.len())),
        )
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        if let Some(map) = self.value {
            for (key, value) in map.iter_mut() {
                f(&key.to_string(), ui, value);
            }
        }
    }
}
