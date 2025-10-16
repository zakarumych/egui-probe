use std::{
    fmt::Debug,
    sync::{Arc, Weak},
};

use parking_lot::RwLock;

use crate::{EguiProbe, ListOptions, Probe};

impl<T> EguiProbe for RwLock<T>
where
    T: Clone + PartialEq + EguiProbe + Debug,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut cloned = { self.read().clone() };
        let probe = cloned.probe(ui, style);
        let changed = {
            let guard = self.read();
            cloned != *guard
        };
        if probe.changed() || changed {
            {
                *self.write() = cloned;
            }
        }
        probe
    }
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let mut cloned = { self.read().clone() };
        cloned.iterate_inner(ui, f);
    }
}

impl<T> EguiProbe for Arc<RwLock<T>>
where
    T: Clone + PartialEq + EguiProbe + Debug,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut cloned = { self.read().clone() };
        let probe = cloned.probe(ui, style);
        let changed = { cloned != *self.read() };
        if probe.changed() || changed {
            {
                *self.write() = cloned;
            }
        }
        probe
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let mut cloned = { self.read().clone() };
        cloned.iterate_inner(ui, f);
        let changed = {
            let guard = self.read();
            cloned != *guard
        };
        if changed {
            *self.write() = cloned;
        }
    }
}

impl<T> EguiProbe for Weak<RwLock<T>>
where
    T: EguiProbe + PartialEq + Clone + ListOptions + Debug,
    T: ListOptions<Resolve = T, Lock = RwLock<T>>,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        match self.upgrade() {
            Some(mut value) => {
                let mut res = Probe::new(&mut value).show(ui);
                if ui.button(style.remove_button_text()).clicked() {
                    *self = Weak::new();
                    res.mark_changed();
                }
                res
            }
            None => {
                let ctx = ui.ctx().clone();
                let options = T::list_available(&ctx);

                // Use a persistent ID for the combobox state
                let combo_id = ui.make_persistent_id("weak_select_combo");

                let response = egui::ComboBox::from_id_salt(combo_id).show_ui(ui, |ui| {
                    for option in options {
                        let res = ui.selectable_label(false, option.to_string());
                        if res.clicked() {
                            return Some(option);
                        }
                    }
                    None
                });

                if let Some(Some(updated)) = response.inner {
                    *self = T::resolve(&ctx, updated);
                }
                response.response
            }
        }
    }
}
