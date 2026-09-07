use std::{
    fmt::{Debug, Display},
    sync::{Arc, RwLock, Weak},
};

use crate::{EguiProbe, Probe};

#[cfg(feature = "parking_lot")]
mod parking_lot;

impl<T> EguiProbe for RwLock<T>
where
    T: Clone + PartialEq + EguiProbe + Debug,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut cloned = { self.read().unwrap().clone() };
        let probe = cloned.probe(ui, style);
        let changed = {
            let guard = self.read().unwrap();
            cloned != *guard
        };
        if probe.changed() || changed {
            {
                *self.write().unwrap() = cloned;
            }
        }
        probe
    }
    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let mut cloned = { self.read().unwrap().clone() };
        cloned.iterate_inner(ui, f);
    }
}

impl<T> EguiProbe for Arc<RwLock<T>>
where
    T: Clone + PartialEq + EguiProbe + Debug,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut cloned = { self.read().unwrap().clone() };
        let probe = cloned.probe(ui, style);
        let changed = { cloned != *self.read().unwrap() };
        if probe.changed() || changed {
            {
                *self.write().unwrap() = cloned;
            }
        }
        probe
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        let mut cloned = { self.read().unwrap().clone() };
        cloned.iterate_inner(ui, f);
        let changed = {
            let guard = self.read().unwrap();
            cloned != *guard
        };
        if changed {
            *self.write().unwrap() = cloned;
        }
    }
}

/// The [`Weak`] acts like an [`Option`]. If the [`Weak`] cannot be upgraded,
/// it is considered [`None`]. If it is [`None`] `T` is used to provide a list
/// of possible values that may be selected using [`ListOptions`].
/// [`ListOptions::resolve`] then returns the new [`Weak`] value that gets updated.
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

/// A helper trait used togehter with [`Weak`] to list possibile values that
/// can be selected
// perhaps this can also be used in Option instead of the Default bound
pub trait ListOptions {
    /// The type of item that can be selected. It must implement [`Display`]
    /// since it is represented as [`selectable_label`](egui::Ui::selectable_label)
    /// Then the [`ListOptions::resolve`] should return an [`RwLock<T>`] where
    /// `T` `=` [`ListOptions::Resolve`]
    type Item: Display;
    type Resolve;
    /// RwLock<Self::Resolve>
    type Lock;
    fn list_available(ctx: &egui::Context) -> impl Iterator<Item = Self::Item>;
    /// resolve the selection
    fn resolve(ctx: &egui::Context, value: Self::Item) -> Weak<Self::Lock>;
}
