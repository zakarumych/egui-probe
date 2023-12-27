use crate::{
    collections::{DeleteMe, EguiProbeFrozen},
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

    fn has_inner(&self) -> bool {
        !self.is_empty()
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        let mut idx = 0;
        self.retain_mut(|value| {
            let mut item = DeleteMe {
                value,
                delete: false,
            };
            f(&format!("[{idx}]"), &mut item);
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

    fn has_inner(&self) -> bool {
        !self.value.is_empty()
    }

    fn iterate_inner(&mut self, f: &mut dyn FnMut(&str, &mut dyn EguiProbe)) {
        for (i, value) in self.value.iter_mut().enumerate() {
            f(&format!("[{i}]"), value);
        }
    }
}
