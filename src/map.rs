use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    str::FromStr,
};

use crate::{
    collections::{DeleteMe, EguiProbeFrozen},
    option::option_probe_with,
    EguiProbe, Style,
};

#[derive(Clone)]
struct HashMapProbeState {
    new_key: String,
    error: bool,
}

struct HashMapProbe {
    state: HashMapProbeState,
    dirty: bool,
    id: egui::Id,
}

impl HashMapProbe {
    fn load(cx: &egui::Context, id: egui::Id) -> HashMapProbe {
        let state = cx.data_mut(|d| {
            d.get_temp_mut_or(
                id,
                HashMapProbeState {
                    new_key: String::new(),
                    error: false,
                },
            )
            .clone()
        });

        HashMapProbe {
            state,
            dirty: false,
            id,
        }
    }

    fn store(self, cx: &egui::Context) {
        if self.dirty {
            cx.data_mut(|d| d.insert_temp(self.id, self.state));
            cx.request_repaint();
        }
    }

    fn new_key_edit(&mut self, ui: &mut egui::Ui, reduce_text_width: f32) {
        let text_edit = egui::TextEdit::singleline(&mut self.state.new_key)
            .hint_text("new key")
            .text_color_opt(if self.state.error {
                Some(ui.visuals().error_fg_color)
            } else {
                None
            })
            .desired_width(ui.spacing().text_edit_width - reduce_text_width);

        let r = ui.add(text_edit);
        if r.changed() {
            self.dirty = true;
            self.state.error = false;
        }
    }

    fn key_error(&mut self) {
        if self.state.error {
            return;
        }
        self.state.error = true;
        self.dirty = true;
    }

    fn key_accepted(&mut self) {
        if self.state.new_key.is_empty() {
            return;
        }
        self.state.new_key.clear();
        self.dirty = true;
    }
}

impl<K, V, S> EguiProbe for HashMap<K, V, S>
where
    K: Display + FromStr + Eq + std::hash::Hash,
    V: EguiProbe + Default,
    S: std::hash::BuildHasher,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
        ui.horizontal(|ui| {
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
            }

            reduce_text_width += r.rect.width() + ui.spacing().item_spacing.x;

            probe.new_key_edit(ui, reduce_text_width);
            probe.store(ui.ctx());
        })
        .response
    }

    fn has_inner(&mut self) -> bool {
        !self.is_empty()
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        self.retain(|key, value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&key.to_string(), &mut item);
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

    fn has_inner(&mut self) -> bool {
        !self.value.is_empty()
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        for (key, value) in self.value.iter_mut() {
            f(&key.to_string(), value);
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
        option_probe_with(self.value, ui, style, |value, ui, _style| {
            ui.weak(format!("[{}]", value.len()));
        })
    }

    fn has_inner(&mut self) -> bool {
        match self.value {
            Some(value) => !value.is_empty(),
            None => false,
        }
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        if let Some(map) = self.value {
            for (key, value) in map.iter_mut() {
                f(&key.to_string(), value);
            }
        }
    }
}
