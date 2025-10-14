use crate::{
    EguiProbe,
    collections::{DeleteMe, EguiProbeFrozen},
    option::option_probe_with,
};

#[cfg(feature = "smallvec1")]
impl<T, const N: usize> EguiProbe for smallvec1::SmallVec<[T; N]>
where
    T: EguiProbe + Default,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut changed = false;
        let mut r = ui
            .horizontal(|ui| {
                ui.weak(format!("[{}]", self.len()));
                let r = ui.small_button(style.add_button_text());
                if r.clicked() {
                    self.push(T::default());
                    changed = true;
                }
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
        let mut idx = 0;
        self.retain_mut(|value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&format!("[{idx}]"), ui, &mut item);
            idx += 1;
            !item.delete
        });
    }
}

#[cfg(feature = "smallvec1")]
impl<T, const N: usize> EguiProbe for EguiProbeFrozen<'_, smallvec1::SmallVec<[T; N]>>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{}]", self.value.len()))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        for (i, value) in self.value.iter_mut().enumerate() {
            f(&format!("[{i}]"), ui, value);
        }
    }
}

#[cfg(feature = "smallvec1")]
impl<T, const N: usize> EguiProbe for EguiProbeFrozen<'_, Option<smallvec1::SmallVec<[T; N]>>>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        option_probe_with(
            self.value,
            ui,
            style,
            smallvec1::SmallVec::new,
            |value, ui, _style| ui.weak(format!("[{}]", value.len())),
        )
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        if let Some(vec) = self.value {
            for (i, value) in vec.iter_mut().enumerate() {
                f(&format!("[{i}]"), ui, value);
            }
        }
    }
}

#[cfg(feature = "smallvec2")]
impl<T, const N: usize> EguiProbe for smallvec2::SmallVec<T, N>
where
    T: EguiProbe + Default,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        let mut changed = false;
        let mut r = ui
            .horizontal(|ui| {
                ui.weak(format!("[{}]", self.len()));
                let r = ui.small_button(style.add_button_text());
                if r.clicked() {
                    self.push(T::default());
                    changed = true;
                }
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
        let mut idx = 0;
        self.retain_mut(|value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&format!("[{idx}]"), ui, &mut item);
            idx += 1;
            !item.delete
        });
    }
}

#[cfg(feature = "smallvec2")]
impl<T, const N: usize> EguiProbe for EguiProbeFrozen<'_, smallvec2::SmallVec<T, N>>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, _style: &crate::Style) -> egui::Response {
        ui.weak(format!("[{}]", self.value.len()))
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        for (i, value) in self.value.iter_mut().enumerate() {
            f(&format!("[{i}]"), ui, value);
        }
    }
}

#[cfg(feature = "smallvec2")]
impl<T, const N: usize> EguiProbe for EguiProbeFrozen<'_, Option<smallvec2::SmallVec<T, N>>>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        option_probe_with(
            self.value,
            ui,
            style,
            smallvec2::SmallVec::new,
            |value, ui, _style| ui.weak(format!("[{}]", value.len())),
        )
    }

    fn iterate_inner(
        &mut self,
        ui: &mut egui::Ui,
        f: &mut dyn FnMut(&str, &mut egui::Ui, &mut dyn EguiProbe),
    ) {
        if let Some(vec) = self.value {
            for (i, value) in vec.iter_mut().enumerate() {
                f(&format!("[{i}]"), ui, value);
            }
        }
    }
}
