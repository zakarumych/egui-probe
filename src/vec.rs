use crate::{
    collections::{DeleteMe, EguiProbeFrozen},
    option::option_probe_with,
    EguiProbe,
};

impl<T> EguiProbe for Vec<T>
where
    T: EguiProbe + Default,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        ui.horizontal(|ui| {
            ui.weak(format!("[{}]", self.len()));
            let r = ui.small_button(style.add_button_text());
            if r.clicked() {
                self.push(T::default());
            }
        })
        .response
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

impl<T> EguiProbe for EguiProbeFrozen<'_, Vec<T>>
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

impl<T> EguiProbe for EguiProbeFrozen<'_, Option<Vec<T>>>
where
    T: EguiProbe,
{
    fn probe(&mut self, ui: &mut egui::Ui, style: &crate::Style) -> egui::Response {
        option_probe_with(self.value, ui, style, Vec::new, |value, ui, _style| {
            ui.weak(format!("[{}]", value.len()))
        })
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
